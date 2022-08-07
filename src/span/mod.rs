mod span;
mod multi;
mod spanned;

pub use proc_macro2::Span;
pub use proc_macro::Span as ProcSpan;

pub use span::AsSpan;
pub use multi::MultiSpan;
pub use multi::AsMultiSpan;
pub use spanned::Spanned;