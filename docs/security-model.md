# Security Model

The JOCKY Control Plane is explicitly designed for forensic data transport.

## Boundaries
- **No Remote Execution**: JOCKY Control Plane **cannot** remotely run `.jky` code or spawn shells.
- **Authentication**: A simple static Bearer token (`JOCKY_API_KEY`) is used for Phase 5 development.
- **Identity**: Endpoints are assigned a UUID upon registration.
- **Audit**: Security events (registration, ingestion) are routed through structured logging.
