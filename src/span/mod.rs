mod spanned;
mod multi;

pub use proc_macro2::Span;
pub use proc_macro::Span as ProcSpan;

pub use spanned::Spanned;
pub use multi::MultiSpan;
pub use multi::AsMultiSpan;