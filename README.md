> **Archived (read-only).** Development has moved to **[`pelorus-marine/platform`](https://github.com/pelorus-marine/platform)** — use **[`pelorus-inspector`](https://github.com/pelorus-marine/platform/tree/main/pelorus-inspector)** (Tauri app: MDF4, DBC, SocketCAN).  
> This repo is **historic only**; issues and PRs are closed. Open new work on **`pelorus-marine/platform`**.

# can-viewer

A desktop application for viewing and analyzing CAN bus data from MDF4 files and live SocketCAN interfaces.

[![Crates.io](https://img.shields.io/crates/v/can-viewer.svg)](https://crates.io/crates/can-viewer)
[![License](https://img.shields.io/crates/l/can-viewer.svg)](LICENSE)

## Features

- **MDF4 File Support** - Load and view CAN data from MDF4 files
- **DBC Decoding** - Decode CAN signals using DBC database files
- **Live Capture** - Capture CAN frames from SocketCAN interfaces (Linux)
- **Export to MDF4** - Save captured frames to MDF4 format
- **Cross-platform UI** - Built with Tauri for native performance

## Installation

### From crates.io (Recommended)

```bash
cargo install can-viewer
can-viewer
```

### From Source

```bash
git clone https://github.com/reneherrero/can-viewer.git
cd can-viewer

# Install frontend dependencies
cd frontend && npm install && cd ..

# Development mode (with devtools)
cargo tauri dev

# Production build
cargo tauri build
```

## Command Line Options

```
can-viewer [OPTIONS]

Options:
  -d, --dbc <PATH>    DBC file to load on startup
  -m, --mdf4 <PATH>   MDF4 file to load on startup
  -h, --help          Print help
```

## Build Requirements

### All Platforms

- Node.js 20+ (for frontend build)
- Rust 1.85+

```bash
# Node.js (via nvm recommended)
nvm install 20
nvm use 20

# Rust (via rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

### Linux (Debian/Ubuntu)

```bash
# Tauri dependencies
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# SocketCAN tools (for live capture)
sudo apt install -y can-utils

# Virtual CAN for testing
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

### macOS

```bash
xcode-select --install
```

Note: SocketCAN live capture is Linux-only. MDF4 viewing and DBC decoding work on all platforms.

### Windows

- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Select "Desktop development with C++" workload

Note: SocketCAN live capture is Linux-only. MDF4 viewing and DBC decoding work on all platforms.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for details on the codebase structure.

## Dependencies

- [mdf4-rs](https://crates.io/crates/mdf4-rs) - MDF4 file parsing
- [dbc-rs](https://crates.io/crates/dbc-rs) - DBC file parsing with FastDbc acceleration
- [Tauri](https://tauri.app) - Desktop application framework

## License

MIT OR Apache-2.0. See [LICENSING.md](LICENSING.md).
