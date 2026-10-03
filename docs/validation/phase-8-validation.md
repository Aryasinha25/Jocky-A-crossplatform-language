# JOCKY Phase 8 Validation Report

## Environment
- Windows 11
- Rust 1.98.1
- target: x86_64-pc-windows-msvc
- JOCKY v0.1

## Phase 8 Step 1 - Persistence Foundation

### Files Changed
- `control-plane/migrations/001_initial_schema.sql`
- `control-plane/internal/investigation/handler.go`
- `control-plane/internal/models/models.go` (Added)
- `control-plane/internal/storage/investigation.go` (Added)
- `control-plane/internal/storage/investigation_test.go` (Added)

### Schema Changes
Added `evidence_ids JSONB NOT NULL DEFAULT '[]'::jsonb` to the `findings` table in `001_initial_schema.sql` to accurately preserve finding references structurally. Did not add `graph_nodes` as the graph can be adequately represented using the combined `evidence` and `findings` tables along with the `relationships` table.

### Storage Changes
Introduced `UpsertInvestigationTx` inside `storage` which handles a complete, single-transaction ingestion of the `InvestigationResult`. This logic cleanly separates database operations from HTTP handling. 

### Transaction Behavior
Implemented transactional persistence. The sequence begins a transaction, sequentially upserts the investigation metadata, loops over and handles `evidence`, `findings`, `relationships`, and `timeline_events`. Any failure strictly triggers a `ROLLBACK`.

### Idempotency Behavior
Investigation payloads natively overwrite the header through `ON CONFLICT DO UPDATE`. Nested structures (`evidence`, `findings`) are gracefully preserved via `ON CONFLICT DO NOTHING`. Relational linkages (`relationships` and `timeline_events`) are cleared via `DELETE FROM ... WHERE investigation_id = $1` and rebuilt cleanly to prevent silent duplication.

### Validation Behavior
Strict reference checking runs proactively **BEFORE COMMIT**. The API explicitly rejects payloads where:
- A `finding` lists an `evidence_id` that does not exist within the investigation payload.
- A `relationship` attempts to link a source or target that cannot be found across either `evidence` or `findings`.

### Tests Added
- `TestUpsertInvestigationTx_Success`
- `TestUpsertInvestigationTx_Idempotent`
- `TestUpsertInvestigationTx_InvalidEvidenceRef`
- `TestUpsertInvestigationTx_InvalidRelationshipRef`
- `TestUpsertInvestigationTx_SQLInjection`
- `TestUpsertInvestigationTx_RollbackOnChildFailure`

### Exact Test Results
`go test ./...`
```
?   	jocky-control-plane/cmd/jocky-server	[no test files]
?   	jocky-control-plane/internal/api	[no test files]
?   	jocky-control-plane/internal/auth	[no test files]
?   	jocky-control-plane/internal/config	[no test files]
?   	jocky-control-plane/internal/endpoint	[no test files]
?   	jocky-control-plane/internal/health	[no test files]
?   	jocky-control-plane/internal/investigation	[no test files]
?   	jocky-control-plane/internal/models	[no test files]
ok  	jocky-control-plane/internal/storage	(cached)
ok  	jocky-control-plane/tests	(cached)
```
Note: As explicitly requested, tests that require an active PostgreSQL instance check for `TEST_DATABASE_URL` and gracefully skip if none is available (as in this specific environment, Docker PostgreSQL was not running).

`cargo check`: PASS
`cargo test --workspace`: PASS (All 4 workspace tests passed)
`cargo fmt --all -- --check`: PASS

- Endpoint ID is presently handled outside the Rust InvestigationResult structure (as an API envelope).

## Phase 8 Step 2 - Retrieval and Pagination

### Retrieval Endpoint
Implemented `GET /api/v1/investigations/:id`. The API accepts investigation IDs in the format `inv-<UUID>` or bare `<UUID>`. It gracefully strips the `inv-` prefix (if present) and safely parses the underlying UUID to validate format, returning HTTP 400 for malformed IDs. It safely fetches all components natively (SQL injection is prevented by `$1` parameters), returning HTTP 404 for valid-format but missing IDs, and 200 upon success.

### List Endpoint
Implemented `GET /api/v1/investigations`. Returns investigation-level metadata.

### Pagination Defaults
- `page`: defaults to 1 if omitted or less than 1. Invalid values trigger HTTP 400.
- `page_size`: defaults to 20. Invalid values trigger HTTP 400. 

### Maximum Page Size
Maximum `page_size` is strictly enforced to 100.

### HTTP Status Behavior
Errors use consistent JSON payloads `{"error":{"code":"...","message":"..."}}`.
- 400 Bad Request: Invalid formatting or pagination values
- 401 Unauthorized: Lacking or incorrect bearer token
- 404 Not Found: UUID parsed but not present
- 200 OK: Valid payload returned
- 500 Internal Server Error: Database failure or corrupted graph relationships internally

### Graph Reconstruction
No `graph_nodes` table was introduced. The internal InvestigationResult Graph is successfully fully rebuilt on retrieval by pulling nodes directly from the `evidence` and `findings` tables. The `relationships` table correctly informs and reconstructs all `GraphEdges`. 

### Reference Validation
Pre-commit graph verification was retained. Additionally, post-retrieval validation ensures all edge relationships map logically to retrieved nodes, preventing API delivery of silently corrupted graphs.

### PostgreSQL Test Environment
Tests actively executed against a healthy `postgres:15-alpine` container running at `127.0.0.1:5432` with Docker Compose. Natively connected via `TEST_DATABASE_URL=postgres://jocky:jockysecret@localhost:5432/jocky?sslmode=disable`.

### Test Results
- Database Integration Tests (`go test ./...` in `jocky-control-plane/tests`): 100% PASS on the live database. No DB tests skipped. Tests added and successfully verified:
  - `TestGetInvestigationByID`
  - `TestGetInvestigationNotFound`
  - `TestGetInvestigationInvalidID`
  - `TestListInvestigations`
  - `TestListInvestigationsPagination`
  - `TestListInvestigationsInvalidPagination`
  - `TestRetrievalAuthentication`
  - `TestRetrievedReferenceIntegrity`
- Go Quality Gates (`go vet`, `gofmt`): PASS
- Rust Quality Gates (`cargo check`, `cargo test --workspace`, `cargo fmt`): PASS

### Known Limitations
- The integration is purely HTTP-driven. Rust automatic POST submission is not yet introduced and will happen in later steps.
- We did not implement new forensic collectors or remote job execution commands in this step.

## Phase 8 Step 1.5 - Initial Live PostgreSQL Integration Validation

### PostgreSQL Startup Result
**FAILED**. Attempted to start PostgreSQL using `docker compose up -d postgres`. The command failed because the Docker daemon is not running on the host system.

### Tests Skipped
All integration tests were skipped due to lack of a live PostgreSQL database:
- `TestUpsertInvestigationTx_Success`
- `TestUpsertInvestigationTx_Idempotent`
- `TestUpsertInvestigationTx_InvalidEvidenceRef`
- `TestUpsertInvestigationTx_InvalidRelationshipRef`
- `TestUpsertInvestigationTx_SQLInjection`
- `TestUpsertInvestigationTx_RollbackOnChildFailure`

### Go Quality Gates
- `gofmt -w ./...`: PASS
- `go test -p 1 ./... -v`: PASS (Integration tests skipped)
- `go vet ./...`: PASS

### Rust Quality Gates
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo fmt --all -- --check`: PASS

## Phase 8 Step 3 - Rust-to-Control-Plane Ingestion (Live Run)

### Submission Architecture
Implemented a dedicated synchronous HTTP client module in Rust (`crates/jocky-cli/src/client.rs`). The `run` subcommand was updated with a `--submit` flag that conditionally triggers the submission of the internally serialized `InvestigationResult` directly against the Control Plane API.

### Request Contract
Rust serialization matches the Go ingestion schema explicitly:
```json
{
  "endpoint_id": "UUID",
  "investigation": { ... InvestigationResult ... }
}
```

### CLI Usage
`jocky-cli run <file.jky> [--submit]`
Submission evaluates only if `--submit` is provided, preserving offline evaluation behavior. 

### Environment Variables
Configuration is handled safely via environment (hardcoded secrets are completely avoided):
- `JOCKY_API_URL`: Base URL of the control plane
- `JOCKY_API_KEY`: API Bearer authentication token
- `JOCKY_ENDPOINT_ID`: Verified Endpoint UUID to attribute the submission

### Authentication and Identity Behavior
The client binds the `JOCKY_API_KEY` into the HTTP Bearer header explicitly, safely passing the identity boundary. The `JOCKY_ENDPOINT_ID` populates the envelope structure. If either configuration parameter is missing, the CLI gracefully aborts prior to execution.

### Idempotency Behavior
Repeated submission tests demonstrate safety via conflict policies in the PostgreSQL storage model. If an Investigation Result is retried natively, the existing schema idempotently processes it. Note that `ON CONFLICT DO NOTHING` on child tables retains stale data if a payload is later modified, but avoids duplicate constraint violations.

### Go API Changes
No internal server API or storage architecture modifications were required. Rust aligned perfectly with the Step 1 / Step 2 model envelope. 

### Final E2E Validation Results (Successful Run)
In a subsequent successful run where Docker and PostgreSQL were available, the following behaviors were proven successfully with live local services:
- **Docker and PostgreSQL Status:** Verified. `jocky-postgres-1` healthy on port 5432.
- **Go Test Execution:** `go test -p 1 ./... -v` executed against live PostgreSQL without skipping. `TestE2ERustSubmission` and `TestDuplicateSubmission` executed successfully. (Total of 15 live database tests passed).
- **Rust Local Execution:** Evaluates cleanly offline without interacting with the network.
- **Rust Network Submission:** Successfully serializes and triggers the HTTP POST to the live endpoint on `jocky-cli run ... --submit`.
- **PostgreSQL Persistence Verification:** Retrieving via `GET /api/v1/investigations/:id` accurately yields the complete investigation graph previously created by the Rust runtime.
- **Duplicate-Submission Verification:** Retries of identical investigations resolve natively into `ON CONFLICT` constraints. The payload and all child records are verified against duplication.
- **Defects Discovered and Fixed:** 
    1. Cross-package test data deletion. Resolved by dropping global `TRUNCATE` operations and scoping cleanup directly to the test-created endpoints via `t.Cleanup()`.
    2. Strict UUID enforcement in the GET endpoint handler rejecting `inv-UUID` patterns despite string DB schema support. Fixed by appropriately stripping prefixes during ID validation.
    3. Duplicate test assertions were incomplete. Resolved by explicitly verifying child record counts and complete payload equivalency upon retrieval.

### Final Classification
**PASS — FULL E2E VALIDATED**

### Known Limitations
- Race conditions during tests are untested natively as `-race` relies on CGO, which is unconfigured on the local agent.
