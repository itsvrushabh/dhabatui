# Changelog

All notable changes to DhabaTUI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.2] - 2026-11-02

### Added
- Automated release cycle workflow (`.github/workflows/release-cycle.yml`) supporting Yearly LTS, Monthly Minor, and Weekly Patch releases.
- Release preparation automation script (`scripts/prepare-release.sh`) with dry-run support.
- OpenCode automated pull request review workflow (`.github/workflows/opencode-review.yml`) with OIDC authentication.
- Comprehensive Project Wiki (`wiki/`) covering Architecture, Financials, Release Cycle, Developer Guide, and Cashier Operations.
- Wiki synchronization script (`scripts/sync-wiki.sh`).
- Release cycle specification document (`docs/RELEASE_CYCLE.md`) and OpenCode guide (`docs/OPENCODE_GUIDE.md`).

### Fixed
- Fixed GitHub Actions release artifact download in `.github/workflows/release.yml` using `merge-multiple: true` to prevent `sha256sum: dhabatui-*: No such file or directory` errors.
- Added duplicate publication check in `.github/workflows/cargo-publish.yml` to gracefully skip crates already published to crates.io.

---

## [0.1.1] - 2026-11-02

### Added
- **Android Mobile Application**: Lightweight native Android client for viewing live dhaba table occupancy, menu catalog, and billing totals over local Wi-Fi.
- **iOS Mobile Application**: Lightweight native SwiftUI iOS client for iPhone and iPad table monitoring.
- **Automated PR-Agent Review**: Codium PR-Agent integration with automated `/describe` summaries and `/review` feedback.

### Fixed
- Cross-platform Swift build support on macOS runners.
- AGP 8.2 compatibility with Gradle 8.6 for Android CI builds.

---

## [0.1.0] - 2026-11-02

### Added
- **Official Crates.io Deployment**: First published release on crates.io (`cargo install dhabatui`).
- **Terminal POS User Interface**: Ratatui-based keyboard-first interface for fast order punching.
- **Zero-Float Financial Math**: Pure integer arithmetic in paise (`100 paise = ₹1.00`) with zero rounding errors.
- **Multi-Table Management**: Track up to 10 dining tables concurrently with status indicators (Vacant, Occupied, Billed).
- **Kitchen Order Tickets (KOT)**: Print and track kitchen tickets with incremental item counts.
- **Bill History & Receipts**: View, search, and reprint past finalized receipts.
- **Day-End Shift Closure**: Daily sales summary, GST reports, and cash vs UPI breakdown.
- **Local LAN HTTP Server**: Built-in HTTP server (`tiny_http`) serving mobile-friendly web dashboard at `http://<LAN-IP>:8080`.
- **Rapid Command Alias (`dti`)**: Fast 3-letter binary alias for instant terminal POS launching.
- **Cross-Platform Release Binaries**: Pre-compiled binaries for Linux, macOS (Apple Silicon & Intel), and Windows.

---

## [0.0.1] - 2026-11-01

### Added
- Initial project prototype and proof-of-concept terminal billing application.
