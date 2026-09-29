---
change: CHG-0002-correct-rollout-review-findings-in-port-governance-and-canonical-contract
artifact: context
---

# Context

The rollout review found that the canonical kill requirement described Unix behavior as universal even though the existing Windows implementation always uses `taskkill /F`. It also found that public documentation and newly installed governance files were absent from meaningful-path enforcement, and that governance-only pull requests did not trigger the existing three-platform CI matrix.

This change corrects only governance, workflow coverage, and canonical documentation. It does not alter plugin product code.
