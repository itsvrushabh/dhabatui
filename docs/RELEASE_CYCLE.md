# DhabaTUI Release Cycle & Cadence Specification

This document outlines the official release cadence, versioning principles, automation triggers, and checklist for **DhabaTUI**.

---

## 📅 The Three Release Tiers

```
┌─────────────────┬──────────────────────┬────────────────────────┬──────────────────────────────────────────┐
│ Release Tier    │ SemVer Pattern       │ Cadence Schedule       │ Scope & Policy                           │
├─────────────────┼──────────────────────┼────────────────────────┼──────────────────────────────────────────┤
│ **Yearly LTS**  │ `vX.0.0`             │ Annually (January 1)   │ 12-Month Long Term Support, enterprise   │
│                 │                      │                        │ stability, database schema migrations.   │
├─────────────────┼──────────────────────┼────────────────────────┼──────────────────────────────────────────┤
│ **Monthly Minor**│ `vX.Y.0`            │ 1st of every month     │ New features, mobile clients (iOS/       │
│                 │                      │                        │ Android), reports, non-breaking changes. │
├─────────────────┼──────────────────────┼────────────────────────┼──────────────────────────────────────────┤
│ **Weekly Patch**│ `vX.Y.Z`             │ Every Sunday 00:00 UTC │ Bug fixes, performance optimizations,    │
│                 │                      │                        │ dependency security updates, UI polish.  │
└─────────────────┴──────────────────────┴────────────────────────┴──────────────────────────────────────────┘
```

---

## 🤖 Release Automation Pipeline

Releases are fully automated via GitHub Actions:

1. **Automation Trigger**:
   - [`.github/workflows/release-cycle.yml`](file:///.github/workflows/release-cycle.yml) executes on schedule (Sundays and 1st of month) or manually via `workflow_dispatch`.
2. **Version Bump & Changelog**:
   - The script [`scripts/prepare-release.sh`](file:///scripts/prepare-release.sh) calculates the next semver from Git tags.
   - Updates `Cargo.toml` and synchronizes `Cargo.lock`.
   - Parses non-merge Git commits and automatically formats and prepends notes to `CHANGELOG.md`.
3. **Commit & Tag**:
   - Creates a commit: `chore(release): vX.Y.Z [skip ci]`.
   - Pushes annotated tag `vX.Y.Z` to `main`.
4. **Downstream Pipeline Execution**:
   - **Cross-Platform Binaries**: [`.github/workflows/release.yml`](file:///.github/workflows/release.yml) builds binaries for Linux, macOS (arm64 & Intel), and Windows, generates `SHA256SUMS.txt`, and publishes a GitHub Release.
   - **Crates.io Publication**: [`.github/workflows/cargo-publish.yml`](file:///.github/workflows/cargo-publish.yml) publishes the package to crates.io using `CARGO_REGISTRY_TOKEN`.

---

## 📋 Pre-Release Verification Checklist

Before triggering a release (or merging PRs intended for an automated release):

- [ ] All unit and integration tests pass: `cargo test --all-targets`
- [ ] Zero clippy warnings: `cargo clippy --all-targets -- -D warnings`
- [ ] Formatting is compliant: `cargo fmt --all -- --check`
- [ ] Packaging dry-run succeeds: `cargo publish --dry-run`
- [ ] Zero floating-point numbers in financial code (strictly integer paise).

---

## 📖 Related Documentation
- [Project Wiki](https://github.com/itsvrushabh/dhabatui/wiki)
- [CHANGELOG.md](file:///CHANGELOG.md)
- [Financial Architecture & Zero-Float Invariant](file:///docs/FINANCIALS.md)
