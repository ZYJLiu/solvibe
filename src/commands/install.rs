use std::process::Command;

pub fn run_solana_install_script() {
    println!("Running Solana install script...");
    let status = Command::new("sh")
        .arg("-c")
        .arg("curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash")
        .status();
    match status {
        Ok(s) if s.success() => println!("Solana CLI installed successfully."),
        Ok(s) => eprintln!("Install script exited with status: {}", s),
        Err(e) => eprintln!("Failed to run install script: {}", e),
    }
}
