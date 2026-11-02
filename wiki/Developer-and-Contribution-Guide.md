# Developer & Contribution Guide

This guide covers everything you need to know to build, test, and contribute to **DhabaTUI**.

---

## 🛠️ Prerequisites

* Rust 1.80+ (`rustup default stable`)
* Git
* Terminal emulator supporting ANSI / truecolor

---

## 🚀 Setting Up the Development Environment

1. Clone the repository:
   ```bash
   git clone https://github.com/itsvrushabh/dhabatui.git
   cd dhabatui
   ```

2. Run the application locally in debug mode:
   ```bash
   cargo run
   ```

3. Run the companion CLI diagnostic tool:
   ```bash
   cargo run --bin dti -- inspect
   ```

---

## 🧪 Testing & Code Verification

Before opening a pull request or submitting code changes, execute the full test and lint suite locally:

```bash
# 1. Run all unit and integration tests
cargo test --all-targets

# 2. Check for linter warnings (warnings are treated as errors in CI)
cargo clippy --all-targets -- -D warnings

# 3. Verify code formatting
cargo fmt --all -- --check
```

---

## 📐 Architecture Invariants to Maintain

When writing new features or fixing bugs, adhere strictly to these engineering invariants:

1. **Paise-Only Math**: Never use `f32` or `f64` for money. Use `u64` representing paise.
2. **Keyboard-First**: Every action in the TUI must have a single-key shortcut (`[Tab]`, `[Enter]`, `[K]`, `[B]`, `[Esc]`, `[Q]`).
3. **Offline-First**: Never introduce external cloud HTTP calls in core billing pathways.
4. **Resilient Error Handling**: Use `thiserror` for library error types; never panic (`unwrap()` / `expect()`) inside the TUI render loop.

---

## 🤖 Automated PR Review & CI Pipeline

When a Pull Request is opened against `main`:
1. **GitHub Copilot Reviewer**: Reviews code changes for architecture and security issues.
2. **PR-Agent**: Automatically generates an itemized `/describe` walkthrough and runs automated `/review`.
3. **OpenCode Agent**: Authenticates via OIDC with OpenCode Console to review PRs and respond to `/oc` interactive comments.
4. **Cross-Platform Matrix CI**:
   - `ubuntu-latest`: Runs `cargo test`, `cargo clippy`, and `cargo fmt`.
   - `macos-latest`: Builds for macOS.
   - `windows-latest`: Builds for Windows.
