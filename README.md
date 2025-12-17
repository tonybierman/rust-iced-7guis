# 7GUIs in Rust with Iced

Implementation of the [7GUIs benchmark](https://7guis.github.io/7guis/) using Rust and the [Iced](https://github.com/iced-rs/iced) GUI framework.

## About 7GUIs

7GUIs is a benchmark for comparing GUI frameworks. It defines seven tasks that represent common GUI programming challenges:

1. [**Counter**](https://github.com/tonybierman/rust-iced-7guis/tree/main/counter) - Simple state management
2. [**Temperature Converter**](https://github.com/tonybierman/rust-iced-7guis/tree/main/temperature-converter) - Bidirectional data flow
3. [**Flight Booker**](https://github.com/tonybierman/rust-iced-7guis/tree/main/flight-booker) - Constraints and validation
4. [**Timer**](https://github.com/tonybierman/rust-iced-7guis/tree/main/timer) - Concurrency and timed events
5. [**CRUD**](https://github.com/tonybierman/rust-iced-7guis/tree/main/crud) - Create, Read, Update, Delete operations
6. [**Circle Drawer**](https://github.com/tonybierman/rust-iced-7guis/tree/main/circle-drawer) - Undo/Redo functionality
7. [**Cells**](https://github.com/tonybierman/rust-iced-7guis/tree/main/cells) - Complex state management (spreadsheet)

## Status 

[![CI](https://github.com/tonybierman/rust-iced-7guis/actions/workflows/CI.yml/badge.svg)](https://github.com/tonybierman/rust-iced-7guis/actions/workflows/CI.yml)


| Binary | Implemented | 
|--------|-------------|
| Counter |✅|
| Temperature Converter |✅|
| Flight Booker |✅|
| Timer |✅|
| CRUD |✅|
| Circle Drawer |✅|
| Cells |✅|

## Structure

This repository is organized as a Cargo workspace with each GUI as a separate binary crate:

```
rust-iced-7guis/
├── Cargo.toml              # Workspace configuration
├── counter/                # Task 1: Counter
├── temperature-converter/  # Task 2: Temperature Converter
├── flight-booker/          # Task 3: Flight Booker
├── timer/                  # Task 4: Timer
├── crud/                   # Task 5: CRUD
├── circle-drawer/          # Task 6: Circle Drawer
└── cells/                  # Task 7: Cells (Spreadsheet)
```

## Running

To run a specific GUI:

```bash
cargo run -p counter
cargo run -p temperature-converter
cargo run -p flight-booker
cargo run -p timer
cargo run -p crud
cargo run -p circle-drawer
cargo run -p cells
```

## Building

To build all GUIs:

```bash
cargo build --release
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Testing

```bash
cargo llvm-cov --no-cfg-coverage
```

or

```bash
cargo llvm-cov --lcov --no-cfg-coverage --output-path lcov.info
```

## License

This project is for educational purposes.

## About 7GUIs: A GUI Programming Benchmark

There are countless GUI toolkits in different languages and with diverse approaches to GUI development. Yet, diligent comparisons between them are rare. Whereas in a traditional benchmark competing implementations are compared in terms of their resource consumption, here implementations are compared in terms of their notation. To that end, [7GUIs](https://eugenkiss.github.io/7guis/) defines seven tasks that represent typical challenges in GUI programming. In addition, 7GUIs provides a recommended set of evaluation dimensions.
