use std::{env, path::PathBuf};

fn main() {
    let kernel = PathBuf::from(env::var_os("CARGO_BIN_FILE_ORBIT").expect("kernel binary path missing"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR missing"));

    let bios_image = out_dir.join("orbit-bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_image)
        .expect("failed to create BIOS disk image");

    println!("cargo:rustc-env=ORBIT_BIOS_IMAGE={}", bios_image.display());
    println!("cargo:rerun-if-changed=src");
}
