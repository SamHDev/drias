use rustc_version::{version, version_meta, Channel};

pub fn main() {
    assert!(version().unwrap().major >= 1);
    match version_meta().unwrap().channel {
        Channel::Stable => {
            println!("cargo:rustc-cfg=rust_stable");
        }
        Channel::Beta => {
            println!("cargo:rustc-cfg=rust_beta");
            println!("cargo:rustc-cfg=rust_stable");
        }
        Channel::Nightly => {
            println!("cargo:rustc-cfg=rust_nightly");
            println!("cargo:rustc-cfg=rust_unstable");
        }
        Channel::Dev => {
            println!("cargo:rustc-cfg=rust_dev");
            println!("cargo:rustc-cfg=rust_unstable");
        }
    }
}