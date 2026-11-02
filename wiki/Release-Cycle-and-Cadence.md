# Release Cycle & Cadence

DhabaTUI employs a predictable, automated release cadence inspired by modern operating system and enterprise software lifecycles. This ensures both rock-solid stability for operational food businesses and continuous delivery of improvements.

---

## 🗓️ Release Cadence Overview

```
                      ┌──────────────────────────────────────────────┐
                      │             DhabaTUI Release Tiers           │
                      └──────────────────────────────────────────────┘
                                              │
                ┌─────────────────────────────┼─────────────────────────────┐
                ▼                             ▼                             ▼
   ┌───────────────────────────┐ ┌───────────────────────────┐ ┌───────────────────────────┐
   │        Yearly LTS         │ │       Monthly Minor       │ │       Weekly Patch        │
   │         `vX.0.0`          │ │         `vX.Y.0`          │ │         `vX.Y.Z`          │
   ├───────────────────────────┤ ├───────────────────────────┤ ├───────────────────────────┤
   │ • Schedule: Annual (Jan)  │ │ • Schedule: 1st of month  │ │ • Schedule: Sundays       │
   │ • Support: 12 Months      │ │ • Support: Current Minor  │ │ • Support: Current Patch  │
   │ • Focus: Major stability, │ │ • Focus: New features,    │ │ • Focus: Bug fixes, UI    │
   │   breaking changes & schema│ │   mobile sync, reports    │ │   polish, security bumps  │
   └───────────────────────────┘ └───────────────────────────┘ └───────────────────────────┘
```

### 1. Yearly LTS Release (`vX.0.0`)
* **Cadence**: Released annually on **January 1st**.
* **Designation**: Long-Term Support (LTS).
* **Support Window**: Supported for **12 consecutive months** with backported critical security and data-integrity patches.
* **Target Audience**: High-volume production dhabas, multi-location restaurants, and franchises that require guaranteed operational stability without frequent UI or database migration adjustments.
* **Scope**: Major architectural enhancements, core database schema evolution, and major new platform targets.

### 2. Monthly Feature / Minor Release (`vX.Y.0`)
* **Cadence**: Released on the **1st day of every month at 00:00 UTC**.
* **Designation**: Feature / Minor release.
* **Support Window**: Supported until the release of the subsequent minor version.
* **Target Audience**: Establishments seeking the latest capabilities (e.g., expanded mobile integrations, new reporting charts, updated GST compliance rules).
* **Scope**: Backwards-compatible features, new TUI tabs/screens, enhanced HTTP endpoints, performance optimizations.

### 3. Weekly Patch / Bugfix Release (`vX.Y.Z`)
* **Cadence**: Released weekly on **Sunday at 00:00 UTC**.
* **Designation**: Patch / Maintenance release.
* **Automation Gate**: Automatically skips execution if no new commits exist since the previous tag.
* **Scope**: Bug fixes, keyboard navigation refinements, terminal resize glitch fixes, receipt layout tweaks, dependency security updates.

---

## 🔢 Semantic Versioning Specification

DhabaTUI strictly follows [Semantic Versioning 2.0.0](https://semver.org/):

$$\text{Format: } \mathbf{v\text{MAJOR}.\text{MINOR}.\text{PATCH}}$$

* **MAJOR**: Incremented for incompatible API, database format changes, or yearly LTS iterations.
* **MINOR**: Incremented monthly for backwards-compatible feature additions.
* **PATCH**: Incremented weekly for backwards-compatible bug fixes and stability improvements.

---

## 🛡️ Support Matrix & Maintenance Windows

| Release Track | Active Maintenance | Security Backports | Recommended Deployment |
| :--- | :--- | :--- | :--- |
| **LTS (v1.0.0)** | 12 Months | Yes (Critical only) | Production cash registers, multi-counter terminals |
| **Minor (vX.Y.0)** | 1 Month | Yes (via weekly patch) | Standard establishments seeking latest features |
| **Patch (vX.Y.Z)** | 1 Week | Rolled forward | Rolling updates |

---

## 🔀 Branching & Git Strategy

```
  main ───●───────●───────●───────●───────●───────● (Rolling development)
          │               │               │
      Tag v0.1.0      Tag v0.1.1      Tag v0.1.2 (Weekly)
                          │
                  (Monthly Minor v0.2.0)
                          │
                  (Yearly LTS v1.0.0)
```

1. **`main` Branch**: Active trunk for all feature and bugfix PR merges. All CI checks, tests, and formatting checks must pass before merging.
2. **Release Tags (`v*`)**: Pushed tags trigger the release pipelines:
   * Multi-platform cross-compilation binaries ([`.github/workflows/release.yml`](https://github.com/itsvrushabh/dhabatui/blob/main/.github/workflows/release.yml)).
   * Crates.io package publication ([`.github/workflows/cargo-publish.yml`](https://github.com/itsvrushabh/dhabatui/blob/main/.github/workflows/cargo-publish.yml)).
3. **LTS Maintenance Branches (`lts/vX`)**: Created for yearly LTS releases if patch backports are required while `main` is developing the next major version.
