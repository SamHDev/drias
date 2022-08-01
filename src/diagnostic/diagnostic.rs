use std::fmt::{Debug, Formatter};
use crate::diagnostic::DiagnosticLevel;
use crate::span::{AsMultiSpan, MultiSpan};

#[derive(Clone)]
pub struct Diagnostic {
    pub(crate) level: DiagnosticLevel,
    pub(crate) message: Option<String>,
    pub(crate) spans: MultiSpan,

    pub(crate) parent: Option<Box<Diagnostic>>
}

impl Diagnostic {
    pub fn new<T: Into<String>>(level: DiagnosticLevel, message: T) -> Self {
        Self {
            level,
            message: Some(message.into()),
            spans: MultiSpan::empty(),
            parent: None
        }
    }

    pub fn spanned<S: AsMultiSpan, T: Into<String>>(spans: S, level: DiagnosticLevel, message: T) -> Self {
        Self {
            level,
            message: Some(message.into()),
            spans: spans.multi_span(),
            parent: None,
        }
    }

    pub fn child<T: Into<String>>(self, level: DiagnosticLevel, message: T) -> Self {
        Self {
            level,
            message: Some(message.into()),
            spans: MultiSpan::empty(),
            parent: Some(Box::new(self))
        }
    }

    pub fn child_spanned<S: AsMultiSpan, T: Into<String>>(self, spans: S, level: DiagnosticLevel, message: T) -> Self {
        Self {
            level,
            message: Some(message.into()),
            spans: spans.multi_span(),
            parent: Some(Box::new(self))
        }
    }
}

impl Diagnostic {
    #[cfg(rust_unstable)]
    pub fn nightly(self) -> crate::diagnostic::nightly::NightlyDiagnostic {
        crate::diagnostic::nightly::construct_nightly(self)
    }

    #[cfg(feature="diag_emulate")]
    pub fn primitive(self) -> crate::diagnostic::primitive::PrimitiveDiagnostic {
        crate::diagnostic::primitive::construct_primitive(self)
    }

    #[allow(unreachable_code)]
    pub fn emit(self) {
        #[cfg(rust_unstable)]
        return self.nightly().emit();
        #[cfg(all(rust_stable, feature="diag_emulate"))]
        return self.primitive().emit();
    }

    pub fn root(&self) -> &Diagnostic {
        fn recurse(diag: &Diagnostic) -> &Diagnostic {
            if let Some(parent) = &diag.parent {
                recurse(&parent)
            } else {
                &diag
            }
        }

        recurse(&self)
    }
}

impl Debug for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Diagnostic(")?;
        f.write_str(match self.root().level {
            DiagnosticLevel::Error => "Error",
            DiagnosticLevel::Warning => "Warning",
            DiagnosticLevel::Note => "Note",
            DiagnosticLevel::Help => "Help",
            DiagnosticLevel::None => "None",
        })?;
        f.write_str(")")
    }
}