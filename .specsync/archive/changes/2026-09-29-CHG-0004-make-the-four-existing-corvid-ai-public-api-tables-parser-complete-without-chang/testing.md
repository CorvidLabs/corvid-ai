---
change: CHG-0004-make-the-four-existing-corvid-ai-public-api-tables-parser-complete-without-chang
artifact: testing
---

# Testing

- REQ-config-001 remains covered by the existing configuration resolution tests.
- REQ-error-001 remains covered by compile-time enum use and provider/config error tests.
- REQ-provider-005 remains covered by pure request-builder and response-parser unit tests.
- REQ-registry-001 remains covered by case-insensitive lookup tests.

Released `specsync check --strict --require-coverage 100 --force` must report all exports documented and 6/6 source files covered.
