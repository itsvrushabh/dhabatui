# DhabaTUI — Developer & Contribution Guide

This guide covers building, testing, linting, code structure, and continuous integration for developers working on `dhabatui`.

---

## 1. Project Structure

```
dhabatui/
├── .github/
│   └── workflows/
│       └── ci.yml          # GitHub Actions CI matrix (Linux, macOS, Windows)
├── docs/
│   ├── ARCHITECTURE.md     # System design & redb storage engine
│   ├── FINANCIALS.md       # Integer paise math & GST specification
│   └── DEVELOPMENT.md      # This file
├── src/
│   ├── main.rs             # Application entrypoint, terminal lifecycle & event loop
│   ├── app.rs              # App state machine, active pane & input modes
│   ├── db.rs               # redb engine, seeds, transaction commits & receipt export
│   ├── models.rs           # Domain models, paise formatting & math logic
│   ├── logger.rs           # Tracing subscriber & non-blocking file appender
│   └── ui.rs               # Ratatui rendering layouts, widgets & modal popups
├── Cargo.toml              # Dependencies & package manifest
├── README.md               # User & cashier operational guide
└── dhabatui.redb           # Embedded database file (created on launch)
```

---

## 2. Prerequisites & Building

Requires a working Rust toolchain (edition 2021, Rust 1.75+):

```bash
# Debug build
cargo build

# Optimized release binary
cargo build --release
```

---

## 3. Running Tests

The test suite covers paise precision math, GST rounding, in-memory `redb` database initialization, transactional checkouts, and complete end-to-end user workflows:

```bash
# Run all tests
cargo test --all-targets
```

---

## 4. Linting & Formatting

DhabaTUI adheres strictly to standard Rust formatting and compiler hygiene (zero warnings):

```bash
# Format check
cargo fmt --all -- --check

# Clippy with all warnings denied
cargo clippy --all-targets -- -D warnings
```

---

## 5. Continuous Integration (CI)

Every push and pull request to `main` triggers [`.github/workflows/ci.yml`](file:///home/cachyos/Work/dhabatui/.github/workflows/ci.yml), which:
1. Runs `cargo fmt -- --check`.
2. Runs `cargo clippy --all-targets -- -D warnings`.
3. Executes all unit and integration tests.
4. Builds release binaries across Linux, macOS, and Windows.
