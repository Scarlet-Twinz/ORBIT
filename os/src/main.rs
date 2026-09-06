use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let image = env!("ORBIT_BIOS_IMAGE");

    println!("ORBIT OS image: {image}");
    println!("Launching QEMU...");

    match Command::new("qemu-system-x86_64")
        .args([
            "-drive",
            &format!("format=raw,file={image}"),
            "-serial",
            "stdio",
            "-no-reboot",
        ])
        .status()
    {
        Ok(status) => status
            .code()
            .map(|code| ExitCode::from(code as u8))
            .unwrap_or(ExitCode::FAILURE),
        Err(error) => {
            eprintln!("Unable to start QEMU: {error}");
            eprintln!(
                "Build succeeded, but QEMU is not available on PATH. Install QEMU and make qemu-system-x86_64 available."
            );
            ExitCode::FAILURE
        }
    }
}
