#!/bin/bash

# Exit script on any error
set -e

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Ensure necessary tools are installed
echo "Verifying necessary tools..."

if ! command_exists pip3; then
    echo "Error: pip3 is not installed. Please install it first."
    exit 1
fi

if ! command_exists wget; then
    echo "Error: wget is not installed. Please install it first."
    exit 1
fi

if ! command_exists unzip; then
    echo "Error: unzip is not installed. Please install it first."
    exit 1
fi

echo "All required tools are available."

# Create pyperformance directory if it doesn't exist
if [ ! -d "pyperformance" ]; then
    echo "Creating pyperformance directory..."
    mkdir pyperformance || { echo "Failed to create pyperformance directory"; exit 1; }
else
    echo "pyperformance directory already exists."
fi

# Install pyperformance using pip3
echo "Installing pyperformance..."
pip3 install pyperformance==1.11.0 || { echo "Failed to install pyperformance"; exit 1; }



# Create nofib_repo directory if it doesn't exist
if [ ! -d "nofib_repo" ]; then
    echo "Creating nofib_repo directory..."
    mkdir nofib_repo || { echo "Failed to create nofib_repo directory"; exit 1; }
else
    echo "nofib_repo directory already exists."
fi

# Navigate to nofib_repo directory
cd nofib_repo || { echo "Failed to change directory to nofib_repo"; exit 1; }

# Download nofib archive
NOFIB_URL="https://gitlab.haskell.org/ghc/nofib/-/archive/7ffecc8115865fea9995a951091e6ff23cf8ca3a/nofib-7ffecc8115865fea9995a951091e6ff23cf8ca3a.zip"
NOFIB_ZIP="nofib-7ffecc8115865fea9995a951091e6ff23cf8ca3a.zip"

if [ ! -f "$NOFIB_ZIP" ]; then
    echo "Downloading nofib benchmark..."
    wget "$NOFIB_URL" -O "$NOFIB_ZIP" || { echo "Failed to download nofib"; exit 1; }
else
    echo "$NOFIB_ZIP already exists."
fi

# Unzip nofib archive
if [ ! -d "nofib-7ffecc8115865fea9995a951091e6ff23cf8ca3a" ]; then
    echo "Unzipping nofib..."
    unzip "$NOFIB_ZIP" || { echo "Failed to unzip nofib"; exit 1; }
else
    echo "nofib directory already exists."
fi

# Rename nofib directory
if [ -d "nofib-7ffecc8115865fea9995a951091e6ff23cf8ca3a" ]; then
    echo "Renaming nofib directory..."
    mv nofib-7ffecc8115865fea9995a951091e6ff23cf8ca3a nofib || { echo "Failed to rename nofib directory"; exit 1; }
else
    echo "nofib directory is already renamed."
fi

echo "Download and extraction complete!"
