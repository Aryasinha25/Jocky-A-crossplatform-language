# Schema Versioning

To ensure stability between the JOCKY Runtime payload generation and any external consumer (like the future Go Control Plane and Tauri/React Dashboard), JOCKY employs explicit API schema versioning.

The `InvestigationResult` struct contains the `schema_version` attribute:
```json
{
  "schema_version": "0.1",
  "investigation_id": "inv-12345",
  "target": "LOCAL"
}
```

This is decoupled from the JOCKY CLI binary version. When external consumers consume this API, they parse the `schema_version` to route decoding through the correct DTO pipeline.
