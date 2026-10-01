# API Contract

JOCKY provides a minimal, read-only HTTP server to transport serialized Investigation objects to future Central Control Plane infrastructures.

## Starting the Server
```bash
jocky serve --port 8787
```
*Note: Binds strictly to `127.0.0.1`. Do not expose publicly without a proxy implementing TLS and Authentication.*

## Endpoints

### `GET /api/v1/health`
Health check endpoint.
```json
{
  "status": "ok",
  "service": "jocky"
}
```

### `GET /api/v1/investigations`
Returns lightweight `InvestigationSummary` objects.
```json
{
  "investigations": [
    {
      "id": "inv-001",
      "target": "LOCAL",
      "platform": "windows",
      "start_time": "2026-10-01T10:20:00Z",
      "end_time": "2026-10-01T10:20:05Z",
      "finding_count": 2,
      "risk_score": 35,
      "priority": "MEDIUM"
    }
  ]
}
```

### `GET /api/v1/investigations/:id`
Returns the complete `InvestigationResult`.

### `GET /api/v1/investigations/:id/graph`
Returns nodes and edges.
Supports query filtering:
- `?type=evidence`
- `?type=finding`

### `GET /api/v1/investigations/:id/timeline`
Returns the chronological timeline.
Supports query parameters:
- `?limit=N`
- `?event_type=process`

### `GET /api/v1/investigations/:id/findings`
Returns a list of findings.
Supports query filters:
- `?severity=high`

## Error Handling
All errors use a structured format:
```json
{
  "error": {
    "code": "INVESTIGATION_NOT_FOUND",
    "message": "Investigation 'inv-999' was not found."
  }
}
```
