---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-port-fledge-plugin
artifact: research
---

# Research

The implementation uses structured lsof fields on Unix and netstat rows on Windows. Five native tests cover Unix parsing, multiple listeners, IPv6, UID fallback, and malformed records. Existing CI builds and tests all three platforms and blocks Clippy and formatting.
