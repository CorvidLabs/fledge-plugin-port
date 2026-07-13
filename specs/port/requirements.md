---
spec: port.spec.md
---

## User Stories

- As a developer, I want to identify and stop the process occupying a TCP port without remembering platform-specific commands.

## Acceptance Criteria

### REQ-port-001

Show SHALL report every process listening on the selected TCP port and a clear successful message when none exists.

Acceptance Criteria
- Listener-reporting parser tests cover occupied and unoccupied ports.

### REQ-port-002

List SHALL report all listening TCP ports discovered by the platform inspector.

### REQ-port-003

Kill SHALL target only processes bound to the selected port, using graceful termination by default and force only when requested.

### REQ-port-004

Unix lsof and Windows netstat output SHALL be parsed deterministically into port, PID, command, and user records where available.

### REQ-port-005

Missing inspectors, inspection failures, invalid ports, and termination failures SHALL fail explicitly.

## Constraints

- Process metadata availability differs by platform and user permissions.

## Out of Scope

- UDP sockets, firewall management, port forwarding, and automatic privilege escalation.
