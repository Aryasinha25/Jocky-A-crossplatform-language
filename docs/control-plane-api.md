# Control Plane API

Base URL: `/api/v1/`

All endpoints (except health) require:
`Authorization: Bearer <JOCKY_API_KEY>`

## Endpoints

- `GET /health` : Service liveness.
- `GET /ready` : Database readiness.
- `POST /endpoints/register` : Registers a new endpoint.
- `POST /endpoints/heartbeat?id=<uuid>` : Updates endpoint Last Seen.
- `GET /endpoints` : List registered endpoints.
- `POST /investigations` : Ingest an investigation result (Idempotent).
- `GET /investigations` : List investigation summaries.
