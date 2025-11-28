#!/bin/bash

# Project dir
cd "$(dirname "$(realpath "$0")")" || exit

# Set aurora-cli
VAR_CLI="aurora-cli"

# Architecture argument (default: x86_64)
ARG_ARCH="${1:-x86_64}"

# Validate architecture argument
case "$ARG_ARCH" in
    "x86_64"|"armv7hl"|"aarch64")
        echo "Building for architecture: $ARG_ARCH"
        ;;
    *)
        echo "Error: Unsupported architecture '$ARG_ARCH'"
        echo "Usage: $0 [x86_64|armv7hl|aarch64]"
        echo "Default: x86_64"
        exit 1
        ;;
esac

# Set target based on architecture
case "$ARG_ARCH" in
    "x86_64")
        VAR_TARGET="x86_64-unknown-linux-gnu"
        ;;
    "armv7hl")
        VAR_TARGET="armv7-unknown-linux-gnueabihf"
        ;;
    "aarch64")
        VAR_TARGET="aarch64-unknown-linux-gnu"
        ;;
esac

# Set SDK path
VAR_SDK="$HOME/AuroraOS"

# Set emulator
VAR_DEVICE_HOST="localhost"
VAR_DEVICE_PORT="2223"
VAR_DEVICE_KEY="$VAR_SDK/vmshare/ssh/private_keys/sdk"

# Clear build
rm -rf target
rm -rf rpm/RPMS
rm -rf rpm/bin

# Check sysroot
if [ ! -d "$HOME/.aurora-sysroot/$ARG_ARCH/usr/include" ]; then
  $VAR_CLI services --sysroot || exit
fi

# Moc Q_OBJECT files
$VAR_CLI services --moc ./ || exit

# Install cross
# https://github.com/cross-rs/cross
cargo install cross --git https://github.com/cross-rs/cross

# Build
CROSS_CONFIG="Cross.local.toml" cross build --target="$VAR_TARGET" --verbose || exit

# Copy bin
mkdir rpm/bin && cp -fr $PWD/target/$VAR_TARGET/debug/aurora-rs-mcp $PWD/rpm/bin || exit

# Build rpm
$VAR_CLI services --rpmbuild "$PWD/rpm" || exit

# Validate
$VAR_CLI services --validate "$PWD/rpm/RPMS/$ARG_ARCH/com.keygenqt.aurora_rs_mcp-0.0.1-1.$ARG_ARCH.rpm" || exit

# Sign rpm
$VAR_CLI services --keysign "$PWD/rpm/RPMS/$ARG_ARCH/com.keygenqt.aurora_rs_mcp-0.0.1-1.$ARG_ARCH.rpm" || exit

echo "Build completed successfully for $ARG_ARCH!"