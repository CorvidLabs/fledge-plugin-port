## MODIFIED

### REQUIREMENT REQ-port-001

Show SHALL report every process listening on the selected TCP port and a clear successful message when none exists.

Acceptance Criteria
- Parser tests cover occupied listener records, while source inspection confirms the explicit successful empty result.

### REQUIREMENT REQ-port-002

List SHALL report all listening TCP ports discovered by the platform inspector.

Acceptance Criteria

- Source inspection confirms `list` performs an unfiltered platform scan and prints every returned record or the explicit successful empty result.

### REQUIREMENT REQ-port-003

Kill SHALL target only processes bound to the selected port. Unix SHALL use graceful termination by default and force only when requested; Windows SHALL use the platform `taskkill /F` behavior for both modes.

Acceptance Criteria

- Source review confirms the Unix signal selection and the unconditional Windows `/F` argument; the platform branches compile in the existing Linux, macOS, and Windows CI matrix.

### REQUIREMENT REQ-port-004

Unix lsof and Windows netstat output SHALL be parsed deterministically into port, PID, command, and user records where available.

Acceptance Criteria

- Unit tests cover single, multiple, IPv6, UID-fallback, and incomplete `lsof` records.
- Source inspection and the Windows build confirm the `netstat` parser accepts only listening TCP records and applies the optional port filter.

### REQUIREMENT REQ-port-005

Missing inspectors, inspection failures, invalid ports, and termination failures SHALL fail explicitly.

Acceptance Criteria

- Source inspection confirms contextual errors for inspector launch/failure and non-zero termination commands.
- Clap rejects ports outside the `u16` range before dispatch, and the help smoke completes successfully.
