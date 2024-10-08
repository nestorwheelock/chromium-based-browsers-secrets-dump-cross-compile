
### Chromium Based Browsers Secrets Dump

This Rust application is designed to extract and decrypt sensitive data stored in the local storage of various Chromium-based web browsers. It decrypts saved passwords and other encrypted data using DPAPI (on Windows) and AES-256-GCM, providing secure and efficient access to browser-stored credentials.

The idea for this project stemmed from the potential practical use in computer repair and data recovery. It could be a valuable tool for recovering important saved information for clients who have lost access to their computers, helping to retrieve stored secrets to aid in data recovery.

Originally forked from [Fastiraz's Chromium-Based Browsers Secrets Dump](https://github.com/Fastiraz/chromium-based-browsers-secrets-dump), this version focuses on cross-compilation on Linux for Windows. Several changes have been made to work with the lower-level `windows-sys` crate, making it more adaptable for such cross-platform scenarios.

---

## Features
- **Cross-Browser Support**: Extracts data from a variety of popular web browsers including Google Chrome, Brave, Microsoft Edge, and more.
- **Encryption Decryption**: Utilizes AES-256-GCM for decrypting sensitive data encrypted by browsers.
- **Flexible Queries**: Supports customizable SQL queries to extract specific data types such as passwords, credit card details, and more.
  - Logins
    - URL
    - Email addresses
    - Passwords
  - Credit cards
    - Card number
    - Expiration date
    - Name on card
  - Cookies

---

## How It Works
The application works by locating browser data directories, decrypting stored secrets using the DPAPI (on Windows) and AES-256-GCM algorithms, and executing SQL queries on the decrypted databases to retrieve relevant information.

---

## Cross-Compiling for Windows on Linux

### Prerequisites and Setup

We encountered several problems during the process of cross-compiling the application for Windows while working on a Linux environment. Here are the steps we took to resolve these issues:

### 1. **Initial Setup and Precompiled Dependency Manager**

Previously, we used a Bash script for managing dependencies, but now we have rewritten the setup process in Rust. This Rust program (`setup_dependencies.rs`) ensures that all required dependencies are installed before building or cross-compiling the project. A precompiled binary version of this dependency manager has been included in the distribution for easier setup.

To use the precompiled binary, simply run:

```bash
./setup_dependencies
```

This binary performs the following tasks:
1. **Rust & Cargo**: Installs Rust and Cargo, the Rust build system.
2. **Target Setup**: Installs the necessary `x86_64-pc-windows-gnu` target for Windows cross-compilation.
3. **System Libraries**:
   - Installs `gcc-mingw-w64` for cross-compilation to Windows.
   - Installs `wine` (optional) for running Windows binaries on Linux.
4. **SSH Key Setup**: Ensures an SSH key is generated and added to the SSH agent for GitHub usage.
5. **Switch Git Remote to SSH**: Updates the Git remote URL to use SSH instead of HTTPS for smoother authentication.

### 2. **Manual Dependency Installation (If Needed)**

If you prefer to manually install the dependencies or modify them, here’s how:

- **Rust & Cargo**: Install using `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```

- **GCC for Windows cross-compilation**:
    ```bash
    sudo apt-get install gcc-mingw-w64
    ```

- **Add Windows cross-compilation target**:
    ```bash
    rustup target add x86_64-pc-windows-gnu
    ```

- **Optional**: Install Wine to run Windows `.exe` files on Linux:
    ```bash
    sudo apt-get install wine
    ```

- **Ensure SSH is properly configured for GitHub**:
    If you don’t have an SSH key, you can generate one:
    ```bash
    ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
    eval "$(ssh-agent -s)"
    ssh-add ~/.ssh/id_rsa
    ```

- **Switch Git Remote to SSH**:
    ```bash
    git remote set-url origin git@github.com:nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile.git
    ```

---

## Cross-Compiling Steps

Once the dependencies are installed and set up, cross-compile the project by running:

```bash
cargo build --target x86_64-pc-windows-gnu
```

For a release build, use:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

You will get a `.exe` file in the `target/x86_64-pc-windows-gnu/release` directory, which can be transferred to a Windows machine or run using Wine:

```bash
wine target/x86_64-pc-windows-gnu/release/chromium-password-stealer.exe
```

---

## Installation

### Build from Source

1. Clone the repository:

```bash
git clone git@github.com:nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile.git
cd chromium-based-browsers-secrets-dump-cross-compile
```

2. Run the precompiled setup binary to install dependencies:

```bash
./setup_dependencies
```

3. Build the project:

```bash
cargo build --release
```

4. Run the project:

```bash
cargo run
```

---

## Usage
The application will automatically search for supported browsers' data directories on your system. Ensure to run the application with administrative or appropriate permissions to access browser data directories and decrypt secrets.

### List of supported browsers

- Avast
- Amigo
- Torch
- Kometa
- Orbitum
- Cent Browser
- 7star
- Sputnik
- Vivaldi
- Google Chrome sxs
- Google Chrome
- Epic Privacy Browser
- Microsoft Edge
- Uran
- Yandex
- Brave Browser
- Iridium
