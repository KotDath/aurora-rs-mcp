#!/bin/bash

# Project dir
cd "$(dirname "$(realpath "$0")")" || exit

# Set aurora-cli
VAR_CLI="aurora-cli"

# Check OS
if [[ "$OSTYPE" == "linux"* ]]; then
  VAR_ARCH="x86_64"
  VAR_TARGET="x86_64-unknown-linux-gnu"
  VAR_SDK="$HOME/AuroraOS"
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
  VAR_ARCH="aarch64"
  VAR_TARGET="aarch64-unknown-linux-gnu"
  VAR_SDK="$HOME/AuroraOS"
fi

if [[ "$OSTYPE" == "msys"* ]]; then
  VAR_ARCH="x86_64"
  VAR_TARGET="x86_64-unknown-linux-gnu"
  VAR_SDK="$HOME/../../AuroraOS"
fi

if [[ -z "$VAR_ARCH" ]]; then
  echo "Not supported $OSTYPE"
fi

# Set emulator
VAR_DEVICE_HOST="localhost"
VAR_DEVICE_PORT="2223"
VAR_DEVICE_KEY="$VAR_SDK/vmshare/ssh/private_keys/sdk"

# Set device
VAR_DEVICE_HOST="192.168.2.13"
VAR_DEVICE_PORT="22"
VAR_DEVICE_KEY="$HOME/.ssh/qtc_id"
VAR_ARCH="armv7hl"
VAR_TARGET="armv7-unknown-linux-gnueabihf"

# Clear build
rm -rf target
rm -rf rpm/RPMS
rm -rf rpm/bin

# Check sysroot
if [ ! -d "$HOME/.aurora-sysroot/$VAR_ARCH/usr/include" ]; then
  $VAR_CLI services --sysroot || exit
fi

# Moc Q_OBJECT files
$VAR_CLI services --moc ./ || exit

# Install cross
# https://github.com/cross-rs/cross
cargo install cross --git https://github.com/cross-rs/cross

# Build
CROSS_CONFIG="Cross.local.toml" cross build --target="$VAR_TARGET" --verbose || exit

# Local build
# SYS="/home/keygenqt/.aurora-sysroot/$VAR_ARCH"
# export PKG_CONFIG_SYSROOT_DIR="/home/keygenqt/.aurora-sysroot/$VAR_ARCH"
# cargo build --target="$VAR_TARGET" --verbose || exit
# unset PKG_CONFIG_SYSROOT_DIR

# Copy bin
mkdir rpm/bin && cp -fr $PWD/target/$VAR_TARGET/debug/aurora-rs-mcp $PWD/rpm/bin || exit

# Build rpm
$VAR_CLI services --rpmbuild "$PWD/rpm" || exit

# Validate
$VAR_CLI services --validate "$PWD/rpm/RPMS/$VAR_ARCH/ru.kotdath.aurora_rs_mcp-0.0.1-1.$VAR_ARCH.rpm" || exit

# Sign rpm
$VAR_CLI services --keysign "$PWD/rpm/RPMS/$VAR_ARCH/ru.kotdath.aurora_rs_mcp-0.0.1-1.$VAR_ARCH.rpm" || exit

# Remove old rmp
ssh -o ConnectTimeout=2 \
  -i $VAR_DEVICE_KEY \
  -o StrictHostKeyChecking=no \
  -p $VAR_DEVICE_PORT defaultuser@$VAR_DEVICE_HOST \
  "rm /home/defaultuser/Downloads/ru.kotdath.aurora_rs_mcp-0.0.1-1.$VAR_ARCH.rpm" 2>/dev/null

# Upload
scp -P $VAR_DEVICE_PORT \
  -i $VAR_DEVICE_KEY \
  ./rpm/RPMS/$VAR_ARCH/ru.kotdath.aurora_rs_mcp-0.0.1-1.$VAR_ARCH.rpm \
  defaultuser@$VAR_DEVICE_HOST:/home/defaultuser/Downloads

# Uninstall
ssh -o ConnectTimeout=10 \
  -i $VAR_DEVICE_KEY \
  -o StrictHostKeyChecking=no \
  -p $VAR_DEVICE_PORT defaultuser@$VAR_DEVICE_HOST \
  "gdbus call --system --dest ru.omp.APM --object-path /ru/omp/APM --method ru.omp.APM.Remove ru.kotdath.aurora_rs_mcp '{}'"

# Install
ssh -o ConnectTimeout=10 \
  -i $VAR_DEVICE_KEY \
  -o StrictHostKeyChecking=no \
  -p $VAR_DEVICE_PORT defaultuser@$VAR_DEVICE_HOST \
  "gdbus call --system --dest ru.omp.APM --object-path /ru/omp/APM --method ru.omp.APM.Install ~/Downloads/ru.kotdath.aurora_rs_mcp-0.0.1-1.$VAR_ARCH.rpm '{}'" || exit

# Delay install
sleep 1

# Run new
ssh -o ConnectTimeout=10 \
  -i $VAR_DEVICE_KEY \
  -o StrictHostKeyChecking=no \
  -p $VAR_DEVICE_PORT defaultuser@$VAR_DEVICE_HOST \
  "runtime-manager-tool Control startDebug ru.kotdath.aurora_rs_mcp --output-to-console" || exit

# Run old
# ssh -o ConnectTimeout=2 \
#   -i $VAR_DEVICE_KEY \
#   -o StrictHostKeyChecking=no \
#   -p $VAR_DEVICE_PORT defaultuser@$VAR_DEVICE_HOST \
#   "invoker --type=qt5 ru.kotdath.aurora_rs_mcp" || exit
