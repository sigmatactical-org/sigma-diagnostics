# Sigma Diagnostics

[![CI](https://github.com/sigmatactical-org/sigma-diagnostics/actions/workflows/ci.yml/badge.svg)](https://github.com/sigmatactical-org/sigma-diagnostics/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="can-viewer/ui/assets/sigma-mark.svg">
  <img src="can-viewer/ui/assets/sigma-mark.svg" alt="Sigma" width="40" height="40" />
</picture>

Cargo workspace for Sigma Racer diagnostics: reusable CAN/DBC/MDF4 domain library, CAN Viewer, and Sigma Racer Mechanic.

| Crate | Role |
| --- | --- |
| [`sigma-diagnostics`](sigma-diagnostics/) | Shared domain (decode, filter, capture, DBC/MDF4, updates, vehicle session) |
| [`can-viewer`](can-viewer/) | Slint desktop UI (MDF4, live SocketCAN, DBC editor) |
| [`sigma-racer-mechanic`](sigma-racer-mechanic/) | Shop tool over SocketCAN + embedded analysis tabs |

Maintained by [Sigma Tactical Group](https://github.com/sigmatactical-org). See [CONTRIBUTORS.md](CONTRIBUTORS.md) for lineage.

## Features

- **MDF4 inspection** — load, filter, decode, and export CAN log files
- **DBC decoding** — decode frames with DBC databases via [dbc-rs](https://github.com/sigmatactical-org/dbc-rs)
- **DBC catalog** — fetch Sigma Racer schemas from [sigma-updates](https://github.com/sigmatactical-org/updates) (`Catalog` button on the DBC tab)
- **Live capture** — SocketCAN capture on Linux with ring-buffer display and MDF4 export
- **DBC editor** — create and edit messages, signals, nodes, and header metadata
- **Native UI** — Slint desktop UI (no WebView or Node.js toolchain)
- **Reusable library** — `sigma-diagnostics` for other crates (e.g. cluster) without Slint

## From source

```bash
git clone https://github.com/sigmatactical-org/sigma-diagnostics.git
cd sigma-diagnostics
cargo run -p can-viewer
# or shop tool:
cargo run -p sigma-racer-mechanic
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

Downloaded schemas are cached under `~/.config/sigma-diagnostics/dbc-cache/`.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

Licensed under MIT OR Apache-2.0. See [LICENSING.md](LICENSING.md).
