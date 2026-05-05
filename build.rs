// build.rs — runs at build time to create the bootable disk image.

use std::path::PathBuf;

fn main() {
    // The kernel ELF binary path
    let kernel_path = std::env::var("CARGO_BIN_FILE_KRISHA_KERNEL_krisha-kernel")
        .unwrap_or_else(|_| {
            std::env::var("CARGO_PKG_NAME").unwrap()
        });

    // We'll write disk images here
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Tell cargo to rerun if main.rs changes
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rustc-env=KERNEL_PATH={}", kernel_path);
}