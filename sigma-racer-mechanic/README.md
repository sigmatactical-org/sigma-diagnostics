# Sigma Racer Mechanic

Shop-side desktop tool for Sigma Racer: connect over SocketCAN, diagnose, maintenance
actions (protocol stubs), OTA catalog download, plus embedded CAN Viewer analysis tabs
(MDF4 / Live / DBC).

```bash
cargo run -p sigma-racer-mechanic
```

Session config: `~/.config/sigma-racer-mechanic/`. DBC cache is shared with
`~/.config/sigma-diagnostics/dbc-cache/`.
