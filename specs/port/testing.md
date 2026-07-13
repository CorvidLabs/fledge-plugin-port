---
spec: port.spec.md
---

## Test Plan

### Unit Tests

- Parse lsof records, multiple listeners, user fallbacks, and malformed fields.
- Parse Windows netstat listeners and apply port filters.

### Integration Tests

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`
- Verify the help surface.
