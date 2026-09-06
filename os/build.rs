use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR missing"));
    let kernel =
        PathBuf::from(env::var_os("CARGO_BIN_FILE_KERNEL_kernel").expect("kernel binary missing"));

    let bios_path = out_dir.join("orbit-bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("failed to create BIOS disk image");

    println!("cargo:rustc-env=ORBIT_BIOS_IMAGE={}", bios_path.display());
    println!("cargo:rerun-if-changed=../kernel/src");
}
