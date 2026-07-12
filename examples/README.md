# Diagnostics Examples

This directory contains sample files for testing Diagnostics.

## Files

- `sample.mf4` - Sample MDF4 file with CAN frames (Engine RPM, Vehicle Speed, Throttle Position)
- `sample.dbc` - Sample DBC file with message and signal definitions

## Usage

### Load MDF4 file on startup

```bash
diagnostics --mdf4 examples/sample.mf4
```

### Load DBC file on startup

```bash
diagnostics --dbc examples/sample.dbc
```

### Load both files on startup

```bash
diagnostics --dbc examples/sample.dbc --mdf4 examples/sample.mf4
```

### Short options

```bash
diagnostics -d examples/sample.dbc -m examples/sample.mf4
```

## Running from source

```bash
cargo run -- --dbc examples/sample.dbc --mdf4 examples/sample.mf4
```
