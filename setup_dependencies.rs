use std::process::{Command, exit};
use std::fs;
use std::path::Path;

fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_if_missing(package: &str) {
    let output = Command::new("dpkg")
        .arg("-l")
        .output()
        .expect("Failed to run dpkg -l");

    if !String::from_utf8_lossy(&output.stdout).contains(package) {
        println!("Installing {}...", package);
        let status = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .arg(package)
            .status()
            .expect("Failed to install package");

        if !status.success() {
            eprintln!("Failed to install {}", package);
            exit(1);
        }
    } else {
        println!("{} is already installed.", package);
    }
}

fn ensure_rust_installed() {
    if !command_exists("rustc") {
        println!("Rust is not installed. Installing Rust...");
        let status = Command::new("sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            .status()
            .expect("Failed to install Rust");

        if !status.success() {
            eprintln!("Failed to install Rust");
            exit(1);
        }
        println!("Rust installed successfully!");
    } else {
        println!("Rust is already installed.");
    }
}

fn ensure_cargo_installed() {
    if !command_exists("cargo") {
        println!("Cargo is not installed. Installing Cargo...");
        let status = Command::new("sh")
            .arg("-c")
            .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
            .status()
            .expect("Failed to install Cargo");

        if !status.success() {
            eprintln!("Failed to install Cargo");
            exit(1);
        }
        println!("Cargo installed successfully!");
    } else {
        println!("Cargo is already installed.");
    }
}

fn add_windows_cross_target() {
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .output()
        .expect("Failed to check Rust target list");

    if !String::from_utf8_lossy(&output.stdout).contains("x86_64-pc-windows-gnu (installed)") {
        println!("Adding Windows cross-compilation target...");
        let status = Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg("x86_64-pc-windows-gnu")
            .status()
            .expect("Failed to add Rust target");

        if !status.success() {
            eprintln!("Failed to add Windows cross-compilation target");
            exit(1);
        }
    } else {
        println!("Windows cross-compilation target is already installed.");
    }
}

fn ensure_ssh_setup() {
    if !Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())).exists() {
        println!("SSH key not found. Generating SSH key...");
        Command::new("ssh-keygen")
            .arg("-t")
            .arg("rsa")
            .arg("-b")
            .arg("4096")
            .arg("-C")
            .arg("your_email@example.com")
            .status()
            .expect("Failed to generate SSH key");

        Command::new("ssh-agent")
            .arg("-s")
            .status()
            .expect("Failed to start SSH agent");

        Command::new("ssh-add")
            .arg("~/.ssh/id_rsa")
            .status()
            .expect("Failed to add SSH key");

        println!("Please add the following SSH key to your GitHub account:");
        let pub_key = fs::read_to_string(format!("{}/.ssh/id_rsa.pub", std::env::var("HOME").unwrap()))
            .expect("Failed to read SSH public key");
        println!("{}", pub_key);
    } else {
        println!("SSH key is already set up.");
    }
}

fn switch_git_remote_to_ssh() {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output();

    if let Ok(output) = output {
        let url = String::from_utf8_lossy(&output.stdout);
        if url.starts_with("https") {
            println!("Switching Git remote to use SSH...");
            Command::new("git")
                .arg("remote")
                .arg("set-url")
                .arg("origin")
                .arg("git@github.com:nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile.git")
                .status()
                .expect("Failed to switch Git remote URL to SSH");
        } else {
            println!("Git remote is already using SSH.");
        }
    } else {
        eprintln!("Failed to get Git remote URL");
    }
}

fn main() {
    // Ensure Rust and Cargo are installed
    ensure_rust_installed();
    ensure_cargo_installed();

    // Install necessary packages
    install_if_missing("gcc-mingw-w64");
    install_if_missing("wine"); // Optional

    // Add the Windows cross-compilation target
    add_windows_cross_target();

    // Ensure SSH is set up properly
    ensure_ssh_setup();

    // Switch Git remote to SSH if needed
    switch_git_remote_to_ssh();

    println!("All dependencies installed and configured. You are ready to build and cross-compile!");
}

