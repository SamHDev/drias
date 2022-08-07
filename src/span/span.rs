use crate::span::{Span, ProcSpan};

pub trait AsSpan {
    fn span(&self) -> Span;

    fn proc_span(&self) -> ProcSpan { self.span().unwrap() }
}

impl AsSpan for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

impl AsSpan for ProcSpan {
    fn span(&self) -> Span {
        Span::from(self.clone())
    }
}

impl AsSpan for &Span {
    fn span(&self) -> Span {
       *self.clone()
    }
}

impl AsSpan for &ProcSpan {
    fn span(&self) -> Span {
        Span::from(*self.clone())
    }
}