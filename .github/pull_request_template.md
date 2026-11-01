## Summary
<!-- Copilot:summary_start -->
<!-- Copilot:summary_end -->

## Proposed Changes
<!-- Copilot:walkthrough_start -->
<!-- Copilot:walkthrough_end -->

## Related Issues
Closes #

## Contributor & Copilot Verification Checklist
- [ ] **Formatting**: \`cargo fmt --all -- --check\` passes without diffs.
- [ ] **Linter**: \`cargo clippy --all-targets -- -D warnings\` passes with zero warnings.
- [ ] **Tests**: \`cargo test --all-targets\` passes (all unit and integration tests).
- [ ] **Financials**: No floating-point types used for monetary values (stored as integer paise \`u32\` / \`u64\`).
- [ ] **Terminal Safety**: No direct stdout/stderr printing inside TUI loop (use \`tracing\` to \`dhabatui.log\`).
- [ ] **CLI Commands**: Documentation uses \`dti\` or \`dhabatui\` (never \`cargo run --release\`).
- [ ] **Guidelines**: Conforms to repository conventions in [\`.github/copilot-instructions.md\`](.github/copilot-instructions.md).
