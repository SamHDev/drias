use std::ops::Deref;
use yansi::{Color, Style};
use crate::diagnostic::{Diagnostic, DiagnosticLevel};

pub struct PrimitiveDiagnostic {
    pub(crate) lines: Vec<String>
}

impl PrimitiveDiagnostic {
    pub fn as_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn print(&self) {
        if !self.lines.is_empty() {
            println!("{}\n", self.as_string());
        }
    }

    pub fn is_none(&self) -> bool {
        self.lines.is_empty()
    }
}

pub fn construct_primitive(diag: Diagnostic) -> PrimitiveDiagnostic {
    let mut lines = Vec::new();
    route_diag(diag, &mut lines);
    PrimitiveDiagnostic { lines }
}

fn prefix_line(char: &str) -> String {
    format!("  {}", Style::new(Color::Cyan).paint(char))
}

fn route_diag(diag: Diagnostic, array: &mut Vec<String>) {
    if diag.level == DiagnosticLevel::None { return }
    match diag.parent {
        None => {
            array.push(top_diag(diag));
            return;
        },
        Some(ref parent) => route_diag(parent.deref().clone(), array)
    };
    if !array.is_empty() { sub_diag(diag, array) }
}


fn top_diag(diag: Diagnostic) -> String {
    let (col, head) = header(&diag.level);
    let header = format!("{}: {}", col.paint(head), diag.message.unwrap_or_default());
    Style::new(Color::Default).bold().paint(header).to_string()
}

fn sub_diag(diag: Diagnostic, array: &mut Vec<String>) {
    if array.len() == 1 { array.push(prefix_line("|")); }

    let (_col, head) = header(&diag.level);
    let header = format!(
        "{} {}: {}",
        prefix_line("="),
        Style::new(Color::Default).bold().paint(head),
        diag.message.unwrap_or_default()
    );
    array.push(Style::new(Color::Default).paint(header).to_string());
}

fn header(level: &DiagnosticLevel) -> (Color, &'static str) {
    match level {
        DiagnosticLevel::Error => (Color::Red, "error"),
        DiagnosticLevel::Warning => (Color::Yellow, "warning"),
        DiagnosticLevel::Note => (Color::Green, "note"),
        DiagnosticLevel::Help => (Color::Blue, "help"),
        DiagnosticLevel::None => unreachable!()
    }
}