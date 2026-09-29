---
change: CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance
artifact: context
---

# Context

The rollout review corrected Port's Windows termination contract, but the canonical companion still lacked concrete acceptance criteria for four requirements. The generated scaffold commands also truncated free-text descriptions and used a shell variable that Gemini does not substitute. This change records the already-reviewed, migration-only corrections in a new workspace because SpecSync correctly forbids changing the definition of an already-applied change.

No Rust product code changes are in scope.
