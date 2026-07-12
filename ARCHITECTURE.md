# Diagnostics — Architecture

## Overview

Diagnostics is a native Slint desktop application. Domain logic lives in Rust; the UI binds to Rust-owned models via Slint properties and callbacks. There is no WebView, JavaScript frontend, or Tauri IPC layer.

```
┌──────────────────────────────────────────────────────────────┐
│                    SigmaDiagnostics (Slint)                     │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────┐  │
│  │  MDF4 tab  │  │  Live tab  │  │  DBC tab   │  │ About  │  │
│  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘  └────────┘  │
└────────┼───────────────┼───────────────┼────────────────────┘
         │               │               │
         ▼               ▼               ▼
┌──────────────────────────────────────────────────────────────┐
│                      src/app/ (controllers)                   │
│   mdf4.rs          live.rs           dbc.rs                   │
└────────┬───────────────┬───────────────┬────────────────────┘
         │               │               │
         ▼               ▼               ▼
┌──────────────────────────────────────────────────────────────┐
│                    src/services/ (domain)                     │
│   mdf.rs   dbc.rs   capture.rs   filter.rs   init.rs          │
└────────┬─────────────────────────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────────────────────────┐
│  state.rs · decode.rs · live_capture.rs · dbc_export.rs       │
│  dbc-rs · mdf4-rs · socketcan (Linux)                          │
└──────────────────────────────────────────────────────────────┘
```

## Directory structure

```
diagnostics/
├── build.rs                 # slint_build::compile("ui/app.slint")
├── ui/
│   ├── app.slint            # SigmaDiagnostics root window
│   ├── theme.slint          # Brand palette
│   ├── tabs/                # MDF4, Live, DBC tab components
│   └── widgets/             # Shared rows, toolbar, status chip
├── src/
│   ├── main.rs              # CLI + app::run()
│   ├── lib.rs               # slint::include_modules! + exports
│   ├── app/                 # Slint callback wiring
│   ├── services/            # Domain services (no IPC)
│   ├── state.rs             # AppState, DBC, capture handles
│   ├── dto.rs               # DTOs + Slint row types
│   ├── decode.rs            # Frame decode helpers
│   ├── live_capture.rs      # Live ring buffer + display generation
│   └── dbc_export.rs        # DBC serialize for editor save
└── tests/
    └── mdf4_integration.rs  # MDF4 load/decode integration tests
```

## Data flow

### MDF4 tab

1. User opens MDF4 via `rfd` file dialog.
2. `services::mdf::load_mdf4` parses ASAM `CAN_DataFrame` channels.
3. Frames stored in `Mdf4Controller`; filters run via `services::filter`.
4. Filtered rows bound to `mdf4-frames` Slint property (`VecModel`).
5. Frame selection triggers `services::dbc::decode_single_frame` for the signals panel.

### Live tab

1. `services::capture::list_can_interfaces` enumerates SocketCAN netdevs.
2. `start_capture` spawns socket + processor threads; updates sent on an `mpsc` channel.
3. Slint `Timer` polls `CaptureSession::poll_update()` at ~10 Hz.
4. `live_capture::generate_display()` builds structured rows (no HTML).
5. `stop_capture` finalizes MDF4 on disk; export copies the capture file.

### DBC tab

1. `services::dbc::get_dbc_info` maps `dbc-rs` types to `DbcInfo`.
2. `DbcController` owns editable `DbcInfo` in a `Mutex`.
3. Slint detail fields two-way bind to selected message/signal/node.
4. Save uses `dbc_export::export_dbc_info_to_string` then `services::dbc::save_dbc_info`.

## Key dependencies

| Crate | Role |
|-------|------|
| `slint` 1.13.1 | Native UI (`backend-winit`, `renderer-femtovg`) |
| `dbc-rs` 0.5 | DBC parse/decode/edit |
| `mdf4-rs` 0.2 | MDF4 read/write (ASAM bus logging) |
| `socketcan` 3 | Linux live capture |
| `rfd` | Native open/save dialogs |

## Session persistence

Configuration is stored under `~/.config/diagnostics` via `config::SessionConfig`.

## Extension point

`lib.rs` re-exports filter utilities, `parse_can_dataframe`, and DTO types for optional pro/extension crates that build on the same domain layer.
