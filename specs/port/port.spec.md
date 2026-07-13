---
module: port
version: 4
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Port

## Purpose

Show processes listening on TCP ports, list all detected listeners, and terminate processes bound to a selected port using platform-native inspection tools.

## Public API

| Surface | Behavior |
|---------|----------|
| show | Display every listener bound to the requested TCP port. |
| shorthand port | Treat a bare numeric port as show. |
| list | Display all detected listening TCP ports. |
| kill | Send SIGTERM by default or an immediate forced termination when requested. |

## Invariants

1. Show and kill restrict discovery to the requested port.
2. Unix discovery considers listening TCP sockets from structured lsof fields.
3. Windows discovery considers only TCP rows in LISTENING state from netstat.
4. Empty discovery is a successful no-op with an explicit message.
5. Default kill attempts graceful termination; force selects immediate termination.
6. Every discovered process is reported before termination is attempted.
7. Inspection-command failures with diagnostics are surfaced rather than treated as an empty result.

## Behavioral Examples

```
Given one or more processes listening on a selected TCP port
When the developer runs show or kill
Then the plugin reports the matching process records and only kill sends the requested termination signal
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Inspector unavailable | lsof or netstat cannot be invoked | Report the missing platform prerequisite and exit non-zero. |
| Inspector failure | The platform tool fails with diagnostics | Surface the diagnostic rather than report no listeners. |
| Termination denied | The process cannot be signaled | Surface the platform error and exit non-zero. |
| Invalid port | CLI input is outside the u16 port range | Clap rejects the argument before inspection. |

## Dependencies

- Rust 1.89 or later
- `lsof` on Unix platforms
- `netstat` and process termination tools on Windows
- `clap` and `anyhow`

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing cross-platform listener discovery and termination behavior for SpecSync 5 adoption. |
| 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-port-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Port Fledge plugin |
| 2026-07-13 | CHG-0002-correct-rollout-review-findings-in-port-governance-and-canonical-contract: Correct rollout review findings in Port governance and canonical contract |
| 4 | 2026-07-13 | CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance: Complete Port requirement evidence and generated integration guidance |
