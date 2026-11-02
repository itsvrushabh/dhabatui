# Release Automation Guide

DhabaTUI features automated GitHub Actions workflows that manage the entire release pipeline from semver calculation to binary artifact cross-compilation and crates.io publishing.

---

## ⚙️ Architecture of the Release Workflows

The release lifecycle consists of three decoupled GitHub Actions workflows:

```
┌──────────────────────────────────────┐
│ .github/workflows/release-cycle.yml  │  Runs weekly/monthly on cron OR manual workflow_dispatch
└──────────────────┬───────────────────┘
                   │ Bumps Cargo.toml, generates CHANGELOG.md, creates & pushes Git Tag `vX.Y.Z`
                   ▼
       ┌────────────────────────┐
       │   Git Tag Push `v*`    │
       └───────────┬────────────┘
                   │
         ┌─────────┴─────────┐
         ▼                   ▼
┌─────────────────┐ ┌─────────────────────┐
│  release.yml    │ │  cargo-publish.yml  │
├─────────────────┤ ├─────────────────────┤
│ • Linux x86_64  │ │ • Dry-run test      │
│ • macOS arm64   │ │ • Check secret      │
│ • macOS Intel   │ │ • cargo publish     │
│ • Windows MSVC  │ │   to crates.io      │
│ • SHA256SUMS    │ └─────────────────────┘
│ • GH Release    │
└─────────────────┘
```

---

## 🚀 Workflows Breakdown

### 1. `release-cycle.yml` (Cadence Scheduler)
* **File**: [`.github/workflows/release-cycle.yml`](file:///home/cachyos/Work/dhabatui/.github/workflows/release-cycle.yml)
* **Triggers**:
  * Weekly Sunday 00:00 UTC (`cron: '0 0 * * 0'`).
  * Monthly 1st of month 00:00 UTC (`cron: '0 0 1 * *'`).
  * Manual trigger (`workflow_dispatch`).
* **What it does**:
  1. Inspects latest git tag (`vX.Y.Z`).
  2. Increments version according to selected tier (`patch`, `minor`, `major_lts`).
  3. Checks for new commits since the last tag. If none, exits cleanly without creating empty releases.
  4. Automatically parses commit messages and prepends categorized release notes to `CHANGELOG.md`.
  5. Updates `version` in `Cargo.toml` and runs `cargo check` to synchronize `Cargo.lock`.
  6. Commits changes as `chore(release): vX.Y.Z [skip ci]`.
  7. Creates and pushes tag `vX.Y.Z` to `origin`.

### 2. `release.yml` (Binary Assets & GitHub Releases)
* **File**: [`.github/workflows/release.yml`](file:///home/cachyos/Work/dhabatui/.github/workflows/release.yml)
* **Triggers**: Push of tag `v*` or manual `workflow_dispatch`.
* **Targets**:
  * Linux x86_64 (`x86_64-unknown-linux-gnu`) on `ubuntu-latest`.
  * Windows x86_64 (`x86_64-pc-windows-msvc`) on `windows-latest`.
  * macOS Apple Silicon (`aarch64-apple-darwin`) on `macos-latest`.
  * macOS Intel (`x86_64-apple-darwin`) cross-compiled on `macos-latest`.
* **Artifacts Created**:
  * `dhabatui-vX.Y.Z-<target>.tar.gz` and `.zip` containing `dhabatui` and `dti`.
  * `SHA256SUMS.txt` cryptographic checksum manifest.
  * Attached to the GitHub Release.

### 3. `cargo-publish.yml` (Crates.io Publication)
* **File**: [`.github/workflows/cargo-publish.yml`](file:///home/cachyos/Work/dhabatui/.github/workflows/cargo-publish.yml)
* **Triggers**: Push of tag `v*` or manual `workflow_dispatch`.
* **Safety features**:
  * Checks if the package version is already uploaded to crates.io; skips gracefully instead of throwing fatal errors.
  * Supports `inputs.dry_run` for dry-run verification.
  * Validates presence of `CARGO_REGISTRY_TOKEN`.

---

## 🖱️ Triggering a Manual Release via GitHub UI

1. Go to the repository **Actions** tab.
2. Select **Release Cycle Automation** in the left sidebar.
3. Click **Run workflow**:
   * **Release Cadence Type**: Choose `patch` (Weekly), `minor` (Monthly), or `major_lts` (Yearly LTS).
   * **Dry Run**: Check `true` to preview the generated changelog without pushing a tag.
   * **Custom Version override**: Leave empty for auto-increment or provide custom semver.
4. Click **Run workflow**.

---

## 🔑 Required Repository Secrets

| Secret Name | Purpose | Where to Configure |
| :--- | :--- | :--- |
| `CARGO_REGISTRY_TOKEN` | Publishing package to crates.io | Repo **Settings** > **Secrets and variables** > **Actions** |
| `GITHUB_TOKEN` | Creating release notes, committing changelog, and pushing tags | Built-in (Automatic) |
