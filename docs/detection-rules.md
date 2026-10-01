# Detection Rules

JOCKY Phase 3 relies on deterministic logic structured as YAML/JSON configuration files rather than hard-coded checks.

## Rule Schema
Rules evaluate normalized `Evidence` components to create `Finding` objects.

```yaml
- id: PROC-SUSPICIOUS-NAME-001
  name: Suspicious Process Name
  description: Detects process names commonly associated with test or suspicious execution.
  severity: medium
  evidence_type: process
  conditions:
    - Equals:
        field: name
        value: "example.exe"
```

## Supported Conditions
Currently, JOCKY supports the following deterministic matchers:
- `Equals { field, value }`
- `Contains { field, value }`
- `StartsWith { field, value }`
- `EndsWith { field, value }`
- `GreaterThan { field, value (numeric) }`
- `LessThan { field, value (numeric) }`

## Severity and Scoring
Severities carry weighted risk score heuristics used to assign priority to an investigation:
- `Info` (0)
- `Low` (10)
- `Medium` (25)
- `High` (50)
- `Critical` (80)
