---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-port-fledge-plugin
artifact: testing
---

# Testing

Local acceptance requires the five-step Fledge lane, all parser tests, strict 100% coverage, four integrations, healthy Trust doctor, and a clean diff.

The five parser tests, compiled platform branches, help smoke, and direct source inspection provide verification evidence for `REQ-port-001`, `REQ-port-002`, `REQ-port-003`, `REQ-port-004`, and `REQ-port-005`. Runtime termination is intentionally not exercised because the migration must not kill host processes.

Hosted acceptance requires the new `trust` job plus existing Linux/macOS/Windows build-test matrix and Linux lint job to pass. Release packaging and Pages remain independent.
