# CAN Viewer

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="ui/assets/sigma-mark.svg">
  <img src="ui/assets/sigma-mark.svg" alt="Sigma" width="40" height="40" />
</picture>

Native desktop application for viewing and analyzing CAN bus data from MDF4 files and live SocketCAN interfaces.

[![Crates.io](https://img.shields.io/crates/v/can-viewer.svg)](https://crates.io/crates/can-viewer)
[![License](https://img.shields.io/crates/l/can-viewer.svg)](LICENSE)

Maintained by [Sigma Tactical Group](https://github.com/sigmatactical-org). See [CONTRIBUTORS.md](CONTRIBUTORS.md) for lineage.

## Features

- **MDF4 inspection** — load, filter, decode, and export CAN log files
- **DBC decoding** — decode frames with DBC databases via [dbc-rs](https://github.com/sigmatactical-org/dbc-rs)
- **DBC catalog** — fetch Sigma Racer schemas from [sigma-updates](https://github.com/sigmatactical-org/updates) (`Catalog` button on MDF4 and DBC tabs)
- **Live capture** — SocketCAN capture on Linux with ring-buffer display and MDF4 export
- **DBC editor** — create and edit messages, signals, nodes, and header metadata
- **Native UI** — Slint desktop UI (no WebView or Node.js toolchain)

## Installation

```bash
cargo install can-viewer
can-viewer
```

## From source

```bash
git clone https://github.com/sigmatactical-org/can-viewer.git
cd can-viewer
cargo run
```

### Linux build dependencies

```bash
sudo apt update
sudo apt install -y libfontconfig1-dev libxkbcommon-dev libgl1-mesa-dev
```

### Requirements

- Rust 1.90+

## Command line options

```
can-viewer [OPTIONS]

Options:
  -d, --dbc <PATH>    DBC file to load on startup
  -m, --mdf4 <PATH>   MDF4 file to load on startup
  -h, --help          Print help
```

### Environment

| Variable | Purpose |
| --- | --- |
| `SIGMA_UPDATES_URL` | Base URL of sigma-updates (default `http://updates.sigma.localtest.me:30080`) |

Downloaded schemas are cached under `~/.config/can-viewer/dbc-cache/`.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

Licensed under MIT OR Apache-2.0. See [LICENSING.md](LICENSING.md).
