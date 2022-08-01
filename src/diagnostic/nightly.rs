pub extern crate proc_macro;

use crate::diagnostic::{Diagnostic, DiagnosticLevel};
use crate::span::{ProcSpan, Span};


pub struct NightlyDiagnostic(Option<proc_macro::Diagnostic>);

impl NightlyDiagnostic {
    pub fn emit(self) {
        if let Some(diag) = self.0 {
            diag.emit();
        }
    }

    pub fn unwrap(self) -> Option<proc_macro::Diagnostic> {
        self.0
    }
}


pub fn construct_nightly(diag: Diagnostic) -> NightlyDiagnostic {
    NightlyDiagnostic(build_sub(diag))
}

fn translate_level(level: &DiagnosticLevel) -> proc_macro::Level {
    match level {
        DiagnosticLevel::Error => proc_macro::Level::Error,
        DiagnosticLevel::Warning => proc_macro::Level::Warning,
        DiagnosticLevel::Help => proc_macro::Level::Help,
        DiagnosticLevel::Note => proc_macro::Level::Note,
        _ => unreachable!()
    }
}

fn translate_params(diag: &Diagnostic) -> (proc_macro::Level, String) {
    (
        translate_level(&diag.level),
        diag.message
            .as_ref()
            .expect("failed to build diagnostic: root diagnostic requires message")
            .to_string()
    )
}

fn translate_spans(spans: Vec<Span>) -> Vec<ProcSpan> {
    spans.into_iter().map(Span::unwrap).collect()
}

fn build_root(diag: Diagnostic) -> Option<proc_macro::Diagnostic> {
    if diag.level == DiagnosticLevel::None {
        None
    } else {
        let (level, message) = translate_params(&diag);


        if diag.spans.is_empty() {
            Some(proc_macro::Diagnostic::new(level, message))
        } else {
            Some(proc_macro::Diagnostic::spanned(
                translate_spans(diag.spans.into_spans()),
                level,
                message
            ))
        }
    }
}

fn build_sub(diag: Diagnostic) -> Option<proc_macro::Diagnostic> {
    let prim = match diag.parent {
        None => return build_root(diag),
        Some(ref parent) => build_sub(*(*parent).clone())?
    };

    if diag.level == DiagnosticLevel::None {
        Some(prim)
    } else {
        let (_level, message) = translate_params(&diag);

        Some(if diag.spans.is_empty() {
            match diag.level {
                DiagnosticLevel::Error => prim.error(message),
                DiagnosticLevel::Warning => prim.warning(message),
                DiagnosticLevel::Note => prim.note(message),
                DiagnosticLevel::Help => prim.help(message),
                DiagnosticLevel::None => unreachable!()
            }
        } else {
            let spans = translate_spans(diag.spans.into_spans());
            match diag.level {
                DiagnosticLevel::Error => prim.span_error(spans, message),
                DiagnosticLevel::Warning => prim.span_warning(spans, message),
                DiagnosticLevel::Note => prim.span_note(spans, message),
                DiagnosticLevel::Help => prim.span_help(spans, message),
                DiagnosticLevel::None => unreachable!()
            }
        })
    }
}