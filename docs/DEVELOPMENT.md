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

Every push and pull request to `main` triggers [`.github/workflows/ci.yml`](../.github/workflows/ci.yml), which:
1. Runs `cargo fmt -- --check`.
2. Runs `cargo clippy --all-targets -- -D warnings`.
3. Executes all unit and integration tests.
4. Builds release binaries across Linux, macOS, and Windows.

---

## 6. Releasing & Binary Packaging

Automated releases are published via [`.github/workflows/release.yml`](../.github/workflows/release.yml) upon pushing a version tag:

```bash
# Tag a new release version
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

The release pipeline automatically:
1. Builds release binaries for **Linux (x86_64)**, **Windows (x86_64)**, **macOS Apple Silicon (aarch64)**, and **macOS Intel (x86_64)**.
2. Packages binaries (`dhabatui` + `dti`), `README.md`, and `LICENSE` into `.tar.gz` and `.zip` archives.
3. Computes SHA256 checksums (`SHA256SUMS.txt`).
4. Creates a new GitHub Release with attached download artifacts and auto-generated release notes.
