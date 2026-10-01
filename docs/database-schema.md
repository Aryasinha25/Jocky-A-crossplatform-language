# Database Schema

The database uses PostgreSQL to map JOCKY outputs into structured queries without over-normalizing flexible attributes.

### Tables
- `endpoints`: Tracks device lifecycles.
- `investigations`: The core entity, linking a forensic scan to an endpoint.
- `evidence`: Raw artifacts represented with JSONB `attributes_json`.
- `findings`: Detection engine results mapped to investigation contexts.
- `relationships`: Traceability edges between nodes.
- `timeline_events`: Chronological steps.
- `audit_logs`: Tracks control plane security events.
