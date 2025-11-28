# Model Context Protocol for Aurora OS



This project is based on the original [aurora-rs](https://gitcode.com/keygenqt_vz/aurora-rs/blob/main/LICENSE) project by Vitaliy Zarubin (Apache-2.0) and keeps the same cross-platform build setup, adding a minimal MCP server example for Aurora OS.

## MCP server

- Runs on `http://0.0.0.0:8080/mcp`
- Provides two tools: `hello_world` that returns a greeting string and `get_aurora_info` to inspect device/cache info.
- Start locally (after exporting the Aurora sysroot env vars used below):

```shell
cargo run
```

## Platform interfaces demo

A demonstration of cross-platform assembly of a Rust application for Aurora OS and interaction with the main interfaces remains available:

- D-Bus, (deviceinfo).
- Clib, (libappdir).
- С++, (example cxx).
- Qt, (QNetworkConfigurationManager).

## Build

- Cross: https://crates.io/crates/cross
- Aurora CLI: https://crates.io/crates/aurora-cli

### Cross.local.toml

Create it at the root of the project and customize it to suit your needs.

```toml
# armv7hl
[target.armv7-unknown-linux-gnueabihf]
env.volumes = [
    "PKG_CONFIG_SYSROOT_DIR=/Users/keygenqt/.aurora-sysroot/armv7hl",
    "PKG_CONFIG_LIBDIR=/Users/keygenqt/.aurora-sysroot/armv7hl/usr/lib",
    "PKG_CONFIG_PATH=/Users/keygenqt/.aurora-sysroot/armv7hl/usr/lib/pkgconfig",
]

# aarch64
[target.aarch64-unknown-linux-gnu]
env.volumes = [
    "PKG_CONFIG_SYSROOT_DIR=/Users/keygenqt/.aurora-sysroot/aarch64",
    "PKG_CONFIG_LIBDIR=/Users/keygenqt/.aurora-sysroot/aarch64/usr/lib",
    "PKG_CONFIG_PATH=/Users/keygenqt/.aurora-sysroot/aarch64/usr/lib/pkgconfig",
]

# x86_64
[target.x86_64-unknown-linux-gnu]
env.volumes = [
    "PKG_CONFIG_SYSROOT_DIR=/Users/keygenqt/.aurora-sysroot/x86_64",
    "PKG_CONFIG_LIBDIR=/Users/keygenqt/.aurora-sysroot/x86_64/usr/lib",
    "PKG_CONFIG_PATH=/Users/keygenqt/.aurora-sysroot/x86_64/usr/lib/pkgconfig",
]
```

> For Windows you need to use sysroot-unix.

### Build binaries

```shell
# armv7hl
CROSS_CONFIG="Cross.local.toml" cross build --target=armv7-unknown-linux-gnueabihf --verbose
# aarch64
CROSS_CONFIG="Cross.local.toml" cross build --target=aarch64-unknown-linux-gnu --verbose
# x86_64
CROSS_CONFIG="Cross.local.toml" cross build --target=x86_64-unknown-linux-gnu --verbose
```

### Configuration for Visual Studio Code

```json
"rust-analyzer.cargo.extraEnv": {
    "PKG_CONFIG_SYSROOT_DIR": "/Users/keygenqt/.aurora-sysroot/aarch64",
    "PKG_CONFIG_LIBDIR": "/Users/keygenqt/.aurora-sysroot/aarch64/usr/lib",
    "PKG_CONFIG_PATH": "/Users/keygenqt/.aurora-sysroot/aarch64/usr/lib/pkgconfig"
}
```

### Run on emulator

macOS / Linux:
```shell
./run.sh
```

Windows:
```shell
sh run.sh
```
