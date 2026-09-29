## MODIFIED

### REQUIREMENT REQ-port-003

Kill SHALL target only processes bound to the selected port. Unix SHALL use graceful termination by default and force only when requested; Windows SHALL use the platform `taskkill /F` behavior for both modes.

Acceptance Criteria
- Source review confirms the Unix signal selection and the unconditional Windows `/F` argument; the platform branches compile in the existing Linux, macOS, and Windows CI matrix.
