---
change: CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance
artifact: testing
---

# Testing

Local verification runs `fledge lanes run verify`, which requires formatting, Clippy with warnings denied, five parser tests, a release build, and the portable help smoke. Strict SpecSync must report non-vacuous 100% file and LOC coverage, and `specsync agents status` must report all four integrations installed.

- `REQ-port-001`: occupied-listener parser tests and inspection of the successful empty-result path.
- `REQ-port-002`: inspection of the unfiltered scan and table/empty output paths.
- `REQ-port-003`: inspection of Unix signal selection and unconditional Windows `/F`, plus all-platform compilation.
- `REQ-port-004`: five `lsof` parser tests plus inspection and compilation of the Windows `netstat` parser.
- `REQ-port-005`: inspection of contextual inspector/termination errors, Clap's `u16` validation, and the help smoke.

Runtime termination is intentionally not exercised because governance verification must not kill host processes. Hosted acceptance remains pending until the exact-head Linux, macOS, Windows, lint, Trust, and CodeQL checks pass.
