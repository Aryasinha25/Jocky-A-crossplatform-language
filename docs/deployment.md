# Deployment

For local development, JOCKY Control Plane is deployed using Docker Compose.

```bash
docker-compose up -d
```

This starts:
1. `postgres:15-alpine` on port 5432 (bound to localhost).
2. `jocky-control-plane` on port 8080 (bound to 0.0.0.0 for container access, mapped to localhost).

**Do NOT expose these ports publicly.**
