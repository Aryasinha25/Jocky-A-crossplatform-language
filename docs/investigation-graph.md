# Investigation Graph

JOCKY represents forensic data in a formal Investigation Graph consisting of Nodes and Edges.

## Graph Model
- **Nodes**: A tagged representation that wraps either an `Evidence` structure or a `FindingNode` structure.
- **Edges**: Links connecting two nodes indicating a definitive relationship.

### Evidence Nodes
Preserves raw collected data:
```json
{
  "type": "evidence",
  "id": "evidence-123",
  "host": "LOCAL",
  "timestamp": "2026-10-01T10:20:00Z",
  "evidence_type": "process",
  "source": "windows",
  "attributes": { ... }
}
```

### Finding Nodes
Preserves deterministic findings, omitting bulky evidence data, relying purely on edges (traceability) to resolve underlying evidence:
```json
{
  "type": "finding",
  "id": "finding-001",
  "rule_id": "PROC-SUSPICIOUS-NAME-001",
  "severity": "medium",
  "confidence": 0.9,
  "timestamp": "2026-10-01T10:20:05Z"
}
```

## Traceability & Edges
Every relationship uses a strongly typed `RelationshipType` enum:
- `Spawned`
- `ConnectedTo`
- `Created`
- `Modified`
- `AssociatedWith`
- `Triggered`
- `GeneratedFinding`
- `RelatedTo`

This enables explicit tracking of the origin of findings:
```json
{
  "source": "evidence-123",
  "relationship": "generated_finding",
  "target": "finding-001"
}
```

## Distinction Between Graph and Timeline
- **Graph**: Answers "What is related to what?" (Topological context)
- **Timeline**: Answers "What happened and when?" (Chronological context)
