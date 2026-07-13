# Sigma Diagnostics — Architecture

## Overview

The **sigma-diagnostics** repository is a Cargo workspace with three members:

- **`sigma-diagnostics`** — reusable domain library (DBC/MDF4/decode/filter/capture/updates/**vehicle**)
- **`can-viewer`** — slim Slint desktop app for MDF4 / Live / DBC analysis
- **`sigma-racer-mechanic`** — Sigma Racer Mechanic shop tool (SocketCAN vehicle link + embedded analysis tabs)

Domain logic has no WebView or Tauri layer. Firmware crates can depend on `sigma-diagnostics` without Slint or session config.

```
┌─────────────────────────────────────────────────────────────────────────┐
│  sigma-racer-mechanic · SigmaRacerMechanic (Slint)                        │
│  Vehicle: Connect · Diagnosis · Maintenance · Settings · Updates · Logs │
│  Analysis: MDF4 · Live · DBC · About  (reuses can-viewer Slint tabs)      │
└───────────────────────────────┬─────────────────────────────────────────┘
                                │
┌───────────────────────────────┼─────────────────────────────────────────┐
│  can-viewer · SigmaDiagnostics│ (Slint)  AnalysisControllers::wire()    │
│  MDF4 · Live · DBC · About    │                                         │
└───────────────┬───────────────┴─────────────────────────────────────────┘
                │
                ▼
┌─────────────────────────────────────────────────────────────────────────┐
│  sigma-diagnostics                                                        │
│  dto · decode · filter · mdf · dbc · capture · updates · vehicle/*      │
└─────────────────────────────────────────────────────────────────────────┘
```

## Directory structure

```
sigma-diagnostics/
├── Cargo.toml                 # workspace: can-viewer, sigma-diagnostics, sigma-racer-mechanic
├── sigma-diagnostics/         # shared domain
│   ├── data/m7-draft.dbc      # embedded M7 draft (synced from wingman schemas)
│   └── src/vehicle/           # VehicleSession, diagnosis, maintenance/settings/ota stubs
├── can-viewer/
│   ├── ui/                    # theme, tabs, widgets (also included by Mechanic)
│   └── src/app/analysis.rs    # AnalysisControllers + wire_analysis_tabs()
└── sigma-racer-mechanic/
    ├── ui/app.slint           # Mechanic shell; imports can-viewer tabs via include path
    └── src/app/               # vehicle + analysis controllers
```

## Vehicle / Mechanic

Shop PC talks to the bike over **SocketCAN** (USB-CAN on the diagnostic/OBD port).  
`VehicleSession` wraps capture + optional embedded `m7-draft.dbc`. Maintenance/settings writes are stubbed until Wingman defines PDUs.

## Session configuration

| App | Config dir |
|-----|------------|
| can-viewer | `~/.config/sigma-diagnostics` |
| sigma-racer-mechanic | `~/.config/sigma-racer-mechanic` |

DBC catalog cache is shared at `~/.config/sigma-diagnostics/dbc-cache/`.
