use crate::diagnostic::Diagnostic;
use crate::span::AsMultiSpan;
use crate::diagnostic::DiagnosticLevel;

macro_rules! diag_impl {
    (
        $(
            $ident: ident,
            $ident_spanned: ident,
            $ident_new: ident,
            $ident_new_spanned: ident
            => $level: ident;
        )*
    ) => {
        $(
            impl Diagnostic {
                #[cfg(feature = "diag_api")]
                pub fn $ident<T: Into<String>>(self, message: T) -> Self {
                    self.child(DiagnosticLevel::$level, message)
                }

                #[cfg(feature = "diag_api")]
                pub fn $ident_spanned<S: AsMultiSpan, T: Into<String>>(self, spans: S, message: T) -> Self {
                    self.child_spanned(spans, DiagnosticLevel::$level, message)
                }

                #[cfg(feature = "diag_ext")]
                pub fn $ident_new<T: Into<String>>(message: T) -> Self {
                    Self::new(DiagnosticLevel::$level, message)
                }

                #[cfg(feature = "diag_ext")]
                pub fn $ident_new_spanned<S: AsMultiSpan, T: Into<String>>(spans: S, message: T) -> Self {
                    Self::spanned(spans, DiagnosticLevel::$level, message)
                }
            }
        )*
    };
}

diag_impl! {
    error, error_spanned, new_error, new_error_spanned => Error;
    warning, warning_spanned, new_warning, new_warning_spanned => Warning;
    note, note_spanned, new_note, new_note_spanned => Note;
    help, help_spanned, new_help, new_help_spanned => Help;
}