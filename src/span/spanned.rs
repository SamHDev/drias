use std::ops::Deref;
use crate::span::{AsMultiSpan, AsSpan, MultiSpan};

pub struct Spanned<T, S = MultiSpan> {
    value: T,
    span: S
}

impl<T> Spanned<T, MultiSpan> {
    pub fn new<S: AsMultiSpan>(value: T, span: S) -> Self {
        Self { value, span: span.multi_span() }
    }
}

impl<T, S: AsSpan> Spanned<T, S> {
    pub fn new(value: T, span: S) -> Self {
        Self { value, span }
    }
}


impl<T, S> Spanned<T, S> {
    pub fn span(&self) -> &S {
        &self.span
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T, S> Deref for Spanned<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}