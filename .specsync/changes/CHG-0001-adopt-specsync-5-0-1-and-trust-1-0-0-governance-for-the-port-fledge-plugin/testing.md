---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-port-fledge-plugin
artifact: testing
---

# Testing

Local acceptance requires the five-step Fledge lane, all parser tests, strict 100% coverage, four integrations, healthy Trust doctor, and a clean diff.

The five parser tests, compiled platform branches, help smoke, and direct source inspection provide evidence as follows:

- `REQ-port-001`: occupied-listener parser tests and inspection of the successful empty-result path.
- `REQ-port-002`: inspection of the unfiltered scan and table/empty output paths.
- `REQ-port-003`: inspection of Unix signal selection and unconditional Windows `/F`, plus all-platform compilation.
- `REQ-port-004`: five `lsof` parser tests plus inspection and compilation of the Windows `netstat` parser.
- `REQ-port-005`: inspection of contextual inspector/termination errors, Clap's `u16` validation, and the help smoke.

Runtime termination is intentionally not exercised because the migration must not kill host processes.

Hosted acceptance requires the new `trust` job plus existing Linux/macOS/Windows build-test matrix and Linux lint job to pass. Release packaging and Pages remain independent.
