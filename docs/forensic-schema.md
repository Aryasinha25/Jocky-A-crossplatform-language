# Forensic Schema

All collected evidence is normalized into a common JSON-compatible structure to enable cross-platform correlation, mapped fundamentally via Graph Nodes.

## Graph Representation
```json
{
  "type": "evidence",
  "id": "String (UUID for this evidence)",
  "host": "String (Target identifier)",
  "timestamp": "String (ISO8601)",
  "evidence_type": "String (e.g., system, process, network, file)",
  "source": "String (e.g., windows, linux)",
  "attributes": {
    "//": "Dynamic JSON object containing type-specific fields"
  }
}
```

## Relationships Model (Edges)
Relationships between evidence components and findings are mapped through strictly-typed Enums:

```json
{
  "source": "process-4210",
  "relationship": "connected_to",
  "target": "network-203"
}
```

Accepted values include `spawned`, `connected_to`, `created`, `modified`, `associated_with`, `triggered`, `generated_finding`, and `related_to`.

## Specific Types

### Process
Attributes might include:
- `pid`: Integer
- `name`: String
- `parent_pid`: Integer (Optional)
- `path`: String (Optional)

### Network (Synthetic Demonstration Limitation)
Note: The Network payload in Phase 3/4 represents a deterministic/synthetic experimental trace intended as a placeholder until an appropriate, cross-platform socket implementation replaces it.
Attributes might include:
- `pid`: Integer (Process ID)
- `local_ip`: String
- `local_port`: Integer
- `remote_ip`: String
- `remote_port`: Integer
- `state`: String
- `protocol`: String
