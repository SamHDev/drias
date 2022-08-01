use crate::span::{Span, ProcSpan};

pub trait Spanned {
    fn span(&self) -> Span;

    fn proc_span(&self) -> ProcSpan { self.span().unwrap() }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

impl Spanned for ProcSpan {
    fn span(&self) -> Span {
        Span::from(self.clone())
    }
}

impl Spanned for &Span {
    fn span(&self) -> Span {
       *self.clone()
    }
}

impl Spanned for &ProcSpan {
    fn span(&self) -> Span {
        Span::from(*self.clone())
    }
}