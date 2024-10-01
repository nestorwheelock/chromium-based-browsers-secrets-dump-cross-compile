
# Cross-Compiling Rust Applications: A Journey of Refactoring, Debugging, and Triumph

**By Nestor Wheelock**  
You can find the code for this project in my GitHub repository: [Chromium-Based Browsers Secrets Dump Cross-Compile](https://github.com/nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile). Feel free to contribute or provide feedback!

Cross-compiling is an essential skill for developers, especially when you need to develop applications for platforms other than your primary development environment. As a Rust developer, I recently embarked on a project to take a Rust application that extracts sensitive data from various Chromium-based browsers, and cross-compile it for Windows, even though my development environment was Linux. What followed was a journey of troubleshooting, debugging, and learning that I’m excited to share with you.

In this article, I’ll break down the challenges I faced and the solutions I found, including refactoring the original codebase, swapping libraries, and configuring system dependencies. Along the way, I’ll emphasize the importance of flexibility and resilience in software development, especially when cross-compiling code.

## The Initial Fork

The project I was working on was a fork of a Rust application designed to extract and decrypt sensitive data, such as passwords and credit card information, stored in browsers like Google Chrome, Brave, and Microsoft Edge. The original project worked on Windows by utilizing the Windows Data Protection API (DPAPI) and AES-256-GCM for decryption. The goal was to make it cross-compile on Linux for Windows, which is especially useful for developers who may not always have access to a Windows machine or want ...

I forked the project from [Fastiraz's Chromium-Based Browsers Secrets Dump](https://github.com/Fastiraz/chromium-based-browsers-secrets-dump) and started making some necessary adjustments. But as with most development journeys, the challenges soon started cropping up.

## The Challenge of Cross-Compiling

Cross-compiling an application from Linux to Windows may seem straightforward at first glance, but it comes with its own set of obstacles. The first issue I faced was that the original code relied heavily on the `winapi` crate, which, while feature-rich, brought compatibility problems when trying to cross-compile on Linux.

### The First Set of Errors

Initially, I encountered errors related to missing imports and unresolved references. The `winapi` crate was packed with features that required access to Windows-specific system APIs, and as expected, those features weren’t fully available when working from a Linux machine. Specifically, errors like this kept showing up:

```bash
error[E0433]: failed to resolve: could not find `Win32` in `windows`
 --> src/main.rs:1:14
  |
1 | use windows::Win32::Security::Cryptography::CryptUnprotectData;
  |              ^^^^^ could not find `Win32` in `windows`
```

The `CryptUnprotectData` function, essential for decrypting data on Windows, couldn’t be found in the context of cross-compiling. At this point, it became clear that I had to refactor the code and replace `winapi` with a lower-level library that would allow for cross-platform development.

## Switching to the `windows-sys` Crate

The solution was to switch from the `winapi` crate to `windows-sys`, which provided a more lightweight, lower-level API. `windows-sys` is much closer to the raw system calls and, while more challenging to work with, it is significantly more portable for cross-compilation scenarios.

After swapping the libraries, I encountered a new set of errors, primarily around missing system dependencies and the need to properly configure the target environment for cross-compiling.

## Refactoring the Code

With the `windows-sys` crate, I had to refactor much of the original code to ensure compatibility. For instance, I had to redefine some structs and functions manually since `windows-sys` lacks some of the higher-level abstractions found in `winapi`.

Here’s an example of the refactored code where I manually defined a `DATA_BLOB` struct:

```rust
type DATA_BLOB = CRYPT_INTEGER_BLOB;

fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut blob_in = DATA_BLOB {
        cbData: encrypted_data.len() as u32,
        pbData: encrypted_data.as_ptr() as *mut u8,
    };
    let mut blob_out = DATA_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };

    let result = unsafe {
        CryptUnprotectData(
            &mut blob_in as *mut _,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            &mut blob_out as *mut _,
        )
    };

    if result != 0 {
        let decrypted_data = unsafe {
            std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize).to_vec()
        };
        unsafe { LocalFree(blob_out.pbData as *mut _); }
        Ok(decrypted_data)
    } else {
        Err("Decryption failed".into())
    }
}
```

## System Dependencies and Environment Setup

To ensure that everything worked seamlessly, I had to install several system dependencies on my Linux machine. This included tools like `mingw-w64` for cross-compiling Windows binaries and Wine for running the resulting executables directly on Linux.

To streamline the process, I wrote a Rust script that automatically installs the required dependencies, checks for the correct targets in Rust, and even configures SSH for pushing code to GitHub. This script, `setup_dependencies.rs`, is included as a precompiled binary in the project, making it easier for others to set up the environment without much hassle.

Here’s a brief overview of what the script does:

1. **Checks for Rust and Cargo**: If they aren’t installed, it installs them.
2. **Installs `mingw-w64`**: Ensures the Windows cross-compilation target is configured.
3. **Configures SSH**: Sets up the SSH keys if they aren’t already configured.
4. **Switches Git Remote to SSH**: For easier interaction with the repository.

## The Power of Cross-Compiling

Cross-compiling is more than just a development convenience—it’s a powerful tool that allows developers to target multiple platforms without having to maintain separate machines or environments. With the right setup, you can write your code once and deploy it across platforms, ensuring maximum reach for your software.

For this project, cross-compiling allowed me to continue working from my Linux environment while ensuring that the final application would run smoothly on Windows.

## Final Thoughts

Refactoring this project for cross-compilation was a rewarding experience that pushed me to learn more about the lower-level system APIs and Rust’s powerful toolchain. The project is now available on my GitHub: [Nestor Wheelock's Chromium-Based Browsers Secrets Dump Cross-Compile](https://github.com/nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile).

If you’re a developer looking to dive into cross-compilation or refactoring projects, I hope this article provides some useful insights and inspires you to take on similar challenges. Feel free to contribute to the project, and happy coding!
