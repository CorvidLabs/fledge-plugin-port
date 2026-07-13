---
spec: port.spec.md
---

## Context

This standalone Rust plugin replaces common manual lsof/netstat and kill workflows with one cross-platform Fledge command.

## Related Modules

- Platform process and socket inspection tools.
- Fledge plugin command registration.

## Design Decisions

- Parse structured lsof fields on Unix to avoid column-format ambiguity.
- Preserve graceful termination as the default safety posture.
