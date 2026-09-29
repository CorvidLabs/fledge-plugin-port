---
change: CHG-0003-complete-port-requirement-evidence-and-generated-integration-guidance
artifact: requirements
---

# Requirements

- Each of `REQ-port-001` through `REQ-port-005` SHALL have concrete acceptance criteria that match the existing implementation and validation surface.
- The semantic delta SHALL enumerate all five requirement IDs so verification evidence binds to every corrected requirement.
- Claude, Cursor, and Gemini scaffold guidance SHALL preserve the complete free-text description before deriving a module name.
- Gemini change creation SHALL pass `{{args}}` rather than the unavailable shell variable `$ARGUMENTS`.
- The change SHALL not alter Rust product behavior.
