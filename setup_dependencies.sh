#!/bin/bash

# Function to install a package if it's not already installed
install_if_missing() {
    if ! dpkg -l | grep -q "$1"; then
        echo "Installing $1..."
        sudo apt-get install -y "$1"
    else
        echo "$1 is already installed."
    fi
}

# Ensure Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Ensure Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo is not installed. Installing Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Cargo is already installed."
fi

# Install mingw-w64 for Windows cross-compilation
install_if_missing gcc-mingw-w64

# Install Wine for running Windows binaries (optional)
install_if_missing wine

# Add Windows cross-compilation target if not already added
rustup target list | grep -q 'x86_64-pc-windows-gnu (installed)'
if [ $? -ne 0 ]; then
    echo "Adding Windows cross-compilation target..."
    rustup target add x86_64-pc-windows-gnu
else
    echo "Windows cross-compilation target is already installed."
fi

# Ensure SSH is properly configured
if [ ! -f "$HOME/.ssh/id_rsa" ]; then
    echo "SSH key not found. Generating SSH key..."
    ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
    eval "$(ssh-agent -s)"
    ssh-add ~/.ssh/id_rsa
    echo "Please add the following SSH key to your GitHub account:"
    cat ~/.ssh/id_rsa.pub
else
    echo "SSH key is already set up."
fi

# Check if Git remote is set to SSH
git_remote_url=$(git remote get-url origin 2> /dev/null)
if [[ $git_remote_url == https* ]]; then
    echo "Switching Git remote to use SSH..."
    git remote set-url origin git@github.com:nestorwheelock/chromium-based-browsers-secrets-dump-cross-compile.git
else
    echo "Git remote is already using SSH."
fi

# Final message
echo "All dependencies installed and configured. You are ready to build and cross-compile!"

