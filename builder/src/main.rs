fn main() {
    let bios_image = env!("BIOS_IMAGE");
    println!("BIOS_IMAGE={}", bios_image);

    let dest = "krisha.img";
    std::fs::copy(bios_image, dest).expect("Failed to copy disk image");
    println!("Copied bootable image to: {}", dest);
}