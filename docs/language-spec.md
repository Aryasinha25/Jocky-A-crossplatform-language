# JOCKY Language Specification v0.1

## Commands

- `TARGET "<target>"`: Defines the target environment (e.g., "LOCAL").
- `COLLECT <type>`: Collects a specific type of evidence. Supported types: `SYSTEM`, `PROCESS`, `FILE`, `NETWORK`, `USER`, `EVENT`, `PERSISTENCE`.
- `CORRELATE <type> WITH <type>`: Associates two evidence streams.
- `DETECT <rule_id>`: Applies a hardcoded or internal detection rule against collected evidence.
- `DETECT RULES "<path>"`: Evaluates a YAML/JSON detection rule configuration file against collected evidence.
- `SCORE FINDINGS`: Computes risk/confidence scores based on detections.
- `GENERATE REPORT` or `REPORT`: Outputs a final report of the investigation.

## Syntax Rules
- Commands are case-sensitive (currently uppercase preferred).
- Whitespace is ignored.
- String literals are enclosed in double quotes.
