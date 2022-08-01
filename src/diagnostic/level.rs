use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
    None,
}


impl Display for DiagnosticLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Note => "note",
            DiagnosticLevel::Help => "help",
            DiagnosticLevel::None => "none",
        })
    }
}