# Timeline Engine

The Timeline component normalizes, structures, and chronologically sorts `Evidence` and `Finding` timestamps into an aggregated incident response layout.

## Normalization
When `Runtime::build_timeline()` is executed:
1. `Evidence` events are processed. Each Evidence node contributes an event with type equal to its `evidence_type` and references its unique `evidence_id`.
2. `Finding` events are processed. When a finding is triggered by the Detection Engine, a timeline artifact of type `FINDING` is logged using the timestamp of generation.
3. The dataset is sorted via `O(n log n)` sort algorithms using `chrono` standard implementations to guarantee strict sequential integrity.

## Output Structure
The result is exported natively to `timeline.json` containing `TimelineEvent` records:

```json
{
  "events": [
    {
      "timestamp": "2026-10-01T10:20:00Z",
      "event_type": "PROCESS",
      "evidence_id": "8b512c1b...",
      "host": "LOCAL",
      "description": "Evidence collected: process"
    },
    {
      "timestamp": "2026-10-01T10:20:01Z",
      "event_type": "FINDING",
      "evidence_id": "8f39ab13...",
      "host": "LOCAL",
      "description": "Indicator detected: Suspicious Process Name"
    }
  ]
}
```
