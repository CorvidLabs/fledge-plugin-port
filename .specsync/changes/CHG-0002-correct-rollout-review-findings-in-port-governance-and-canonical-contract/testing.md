---
change: CHG-0002-correct-rollout-review-findings-in-port-governance-and-canonical-contract
artifact: testing
---

# Testing

- Run `fledge lanes run verify` for formatting, Clippy, tests, release build, and smoke execution.
- Run `specsync check --strict --force` at the repository's committed advisory threshold.
- Run `specsync agents status` and require Claude, Cursor, Codex, and Gemini.
- Run `fledge trust doctor` and `fledge trust verify`.
- Confirm the pull-request matrix runs on Linux, macOS, and Windows for this governance-only diff.

`REQ-port-003` is evidenced by source inspection of the `cfg(unix)` and `cfg(windows)` termination branches plus successful compilation in the existing three-platform hosted matrix. The change intentionally documents existing behavior without changing product code.
