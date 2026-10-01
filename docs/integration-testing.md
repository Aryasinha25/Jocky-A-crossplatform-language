# Integration Testing

Testing across boundaries requires all services to be running.

## End to End Test Workflow
1. `docker-compose up -d` to start PostgreSQL.
2. `cd control-plane && go run ./cmd/jocky-server` to start the backend.
3. `cd ui && npm run dev` to start the frontend.
4. `cargo run -- run examples/basic.jky` to generate and ingest investigation data.
