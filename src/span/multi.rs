use crate::span::{Span, AsSpan};

#[derive(Clone)]
pub struct MultiSpan(Vec<Span>);

impl MultiSpan {
    pub fn new(spans: Vec<Span>) -> Self {
        Self(spans)
    }

    pub fn one(span: Span) -> Self {
        Self(vec![span])
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn into_spans(self) -> Vec<Span> {
        self.0
    }

    pub fn spans(&self) -> &Vec<Span> {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub trait AsMultiSpan {
    fn multi_span(self) -> MultiSpan;
}

impl AsMultiSpan for MultiSpan {
    fn multi_span(self) -> MultiSpan {
        self
    }
}

impl AsMultiSpan for () {
    fn multi_span(self) -> MultiSpan {
        MultiSpan::empty()
    }
}

impl<T: AsSpan> AsMultiSpan for T {
    fn multi_span(self) -> MultiSpan {
        MultiSpan::one(self.span())
    }
}

impl<T: AsSpan> AsMultiSpan for Vec<T> {
    fn multi_span(self) -> MultiSpan {
        MultiSpan::new(self.into_iter().map(|x| x.span()).collect())
    }
}