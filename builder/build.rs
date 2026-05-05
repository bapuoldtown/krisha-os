use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KRISHA_KERNEL_krisha-kernel").unwrap());
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}