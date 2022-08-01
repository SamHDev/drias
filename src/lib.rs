#![cfg_attr(rust_unstable, feature(proc_macro_diagnostic))]
extern crate proc_macro;

pub mod span;
#[cfg(feature = "diag")] pub mod diagnostic;

pub const IS_NIGHTLY: bool = {
    #[allow(unreachable_code)]
    const fn inner() -> bool {
        #[cfg(rust_unstable)] return true;
        return false;
    }

    inner()
};