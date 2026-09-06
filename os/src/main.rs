use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let image = env!("ORBIT_BIOS_IMAGE");

    println!("ORBIT OS image: {image}");

    match Command::new("qemu-system-x86_64")
        .args(["-drive", &format!("format=raw,file={image}"), "-serial", "stdio"])
        .status()
    {
        Ok(status) => status
            .code()
            .map(|code| ExitCode::from(code as u8))
            .unwrap_or(ExitCode::FAILURE),
        Err(error) => {
            eprintln!("Unable to start QEMU: {error}");
            eprintln!("Build succeeded. Install QEMU and make qemu-system-x86_64 available on PATH to boot ORBIT.");
            ExitCode::SUCCESS
        }
    }
}
