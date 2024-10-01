
<div align="center">
<h1><code>Chromium Based Browsers Secrets Dump</code></h1>
<p>A Rust application for extracting and decrypting sensitive data from various web browsers' local storage. This tool decrypts saved passwords and other encrypted data using DPAPI and AES-256-GCM, providing insights into browser-stored credentials securely and efficiently.</p>
</div>

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

### 1. **Initial Setup Issues**
   At first, the project was set up to work primarily on Windows using the `windows` crate, which relies on Windows APIs such as `CryptUnprotectData`. However, compiling it on Linux led to issues with missing Windows-specific dependencies.

#### Solution:
We installed the following libraries and made the necessary adjustments for cross-compiling:
   - **GCC for Windows cross-compilation**:
     ```bash
     sudo apt-get install gcc-mingw-w64
     ```

   - **Installing Rust and Required Target**:
     ```bash
     rustup target add x86_64-pc-windows-gnu
     ```

   - **Windows API Crates**: We switched from the `windows` crate to `windows-sys`, which is better suited for cross-compilation due to its lower-level bindings.

### 2. **Problems with Importing Windows APIs**
   - We encountered errors such as unresolved imports for `CryptUnprotectData`, `DATA_BLOB`, and `LocalFree` due to incorrect crate usage and missing feature flags.
   
#### Solution:
   - **Fixing Import Errors**:
     We switched to using `windows-sys` for accessing Windows APIs and enabled the necessary feature flags in `Cargo.toml`:
   
   ```toml
   [dependencies]
   windows-sys = { version = "0.59.0", features = ["Win32_Foundation", "Win32_Security_Cryptography", "Win32_System_Memory"] }
   serde_json = "1.0.128"
   base64 = "0.22.1"
   hex = "0.4.3"
   ```

   This fixed the import errors related to missing `DATA_BLOB` and `LocalFree`.

### 3. **Casting and Memory Management Issues**
   During the decryption process, we encountered casting issues when passing pointers of `DATA_BLOB` and memory management problems.

#### Solution:
   - **Type Alias for `DATA_BLOB`**: We resolved the casting issue by defining a type alias `DataBlob` for `CRYPT_INTEGER_BLOB`, which is a part of the Windows API:
   
   ```rust
   type DataBlob = CRYPT_INTEGER_BLOB;
   ```

   - **Memory Management**: We used `LocalFree` to properly free memory allocated by Windows APIs to avoid memory leaks.

### 4. **Build and Linking Errors**
   We faced several linking errors when trying to build the project for Windows. These errors were related to missing Windows system libraries and functions that were not being resolved.

#### Solution:
   - **Installing the Correct Toolchain**: We ensured that the correct target (`x86_64-pc-windows-gnu`) was installed using `rustup`.
   - **Rebuilding the Project**: We fixed all linking issues by running `cargo clean` and rebuilding the project with:
     ```bash
     cargo build --target x86_64-pc-windows-gnu
     ```

---

### Final Cross-Compiling Steps

1. **Rust & Cargo**: Ensure you have Rust and Cargo installed on your system. You can install them via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```

2. **Target Setup**: Install the necessary toolchain and target for cross-compiling:
    ```bash
    rustup target add x86_64-pc-windows-gnu
    ```

3. **Install Required Libraries**:
   - **GCC for Windows cross-compilation**:
     ```bash
     sudo apt-get install gcc-mingw-w64
     ```

4. **Build the Project**:
   Once the dependencies and libraries are in place, cross-compile the project for Windows by running:
   ```bash
   cargo build --target x86_64-pc-windows-gnu
   ```

   For optimized builds:
   ```bash
   cargo build --release --target x86_64-pc-windows-gnu
   ```

5. **Running on Windows**:
   After building, you can transfer the `.exe` file to a Windows system or run it on Linux using Wine:
   ```bash
   wine target/x86_64-pc-windows-gnu/release/chromium-password-stealer.exe
   ```

---

## Installation

### Build from Source

1. Clone the repository:

```bash
git clone https://github.com/nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile.git
cd chromium-based-browsers-secrets-dump-cross-compile
```

2. Build the project:

```bash
cargo build --release
```

3. Run the project:

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

---

This README now includes details about the issues we encountered during cross-compiling, the system libraries we had to install, and how we fixed them to successfully compile the project on a Linux environment for Windows.
