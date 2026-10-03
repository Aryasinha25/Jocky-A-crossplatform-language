# JOCKY Local Prototype Demo Runbook

This guide covers testing the full Phase 8 Control Plane integration, demonstrating the Rust CLI's ability to natively generate, correlate, and persist investigations safely within the PostgreSQL backend.

## Prerequisites
- **Docker & Docker Compose** (Ensure Docker Desktop is running)
- **Rust** (1.98+) & Cargo
- **Go** (1.26+)
- Node.js (for the Analyst UI)
- PowerShell (Windows environment)

## 1. Startup Order

1. **Start PostgreSQL Database**
   ```powershell
   docker compose up -d postgres
   ```
   *Verify it's healthy on port 5432 using `docker compose ps`.*

2. **Start the Go Control Plane**
   Open a dedicated PowerShell terminal in the `control-plane` directory:
   ```powershell
   cd control-plane
   $env:JOCKY_DATABASE_URL="postgres://jocky:jockysecret@localhost:5432/jocky?sslmode=disable"
   $env:JOCKY_API_KEY="demo-secret"
   go run ./cmd/jocky-server
   ```
   *Wait until you see: `JOCKY Control Plane listening on 127.0.0.1:8080`.*

## 2. API Health & Endpoint Registration

Open a new PowerShell terminal to register a test endpoint. This generates an ID required for attribution.

**Check Health:**
```powershell
Invoke-RestMethod -Uri "http://127.0.0.1:8080/api/v1/health" | ConvertTo-Json
```

**Register Endpoint:**
```powershell
$headers = @{ "Authorization" = "Bearer demo-secret"; "Content-Type" = "application/json" }
$body = @{ name = "demo-ep"; hostname = "demo-host"; platform = "windows"; architecture = "x86_64"; jocky_version = "0.1" } | ConvertTo-Json
$response = Invoke-RestMethod -Uri "http://127.0.0.1:8080/api/v1/endpoints/register" -Method Post -Headers $headers -Body $body
$response | ConvertTo-Json
```
> **IMPORTANT:** Copy the `endpoint_id` from the output. It will be required for the next steps.

## 3. Offline Execution Test

Verify the JOCKY CLI processes rules natively without making network calls.
```powershell
# From the repository root
cargo run --bin jocky-cli -- run examples/basic.jky
```
**Expected Output:**
- 345+ evidence records collected
- Timeline populated
- Generates `investigation.json` and `timeline.json` locally.
- *Notice: No "Submitting to Control Plane" text is printed.*

## 4. Live Submission Test

Submit the investigation to the Control Plane. Replace `<ENDPOINT_ID>` with the ID generated in Step 2.

```powershell
$env:JOCKY_API_URL="http://127.0.0.1:8080"
$env:JOCKY_API_KEY="demo-secret"
$env:JOCKY_ENDPOINT_ID="<ENDPOINT_ID>"
cargo run --bin jocky-cli -- run examples/basic.jky --submit
```
**Expected Output:**
- Runs the same offline collection.
- Prints `Submitting to Control Plane...`
- Prints `[✓] Successfully submitted investigation inv-<UUID>`

> **IMPORTANT:** Copy the generated `inv-<UUID>`.

## 5. Investigation Retrieval Verification

Use the `inv-<UUID>` to query the Control Plane directly and verify the data was stored natively.

```powershell
$headers = @{ "Authorization" = "Bearer demo-secret" }
$inv = Invoke-RestMethod -Uri "http://127.0.0.1:8080/api/v1/investigations/inv-<UUID>" -Headers $headers
Write-Output "Nodes: $($inv.graph.nodes.Count) | Events: $($inv.timeline.events.Count)"
```
**Expected Output:**
- `Nodes: 350 | Events: 350` (or identical counts to the CLI output).

## 6. Testing the Analyst UI

The UI project (React/Tauri) connects to the API to display the graphs visually.

1. Open a new terminal in the `ui` folder.
2. Ensure you have Node installed, then run:
   ```powershell
   npm install
   npm run dev
   ```
3. Open `http://localhost:5173` in your browser.
4. Open the Developer Console (F12) -> Application -> Local Storage.
5. Set `jocky_api_token` to `demo-secret` and `jocky_api_url` to `http://127.0.0.1:8080`.
6. Remove `jocky_demo_mode` if it exists.
7. Reload the page. You should see the Control Plane Connected indicator and be able to navigate to Investigations.

### Known Limitations
- Modifying a previously submitted finding payload and re-submitting with the same ID will result in the new changes being ignored (`ON CONFLICT DO NOTHING`), though the root investigation will still process cleanly without causing constraints violation.

## Shutdown Instructions
- Close all terminal tabs running `go run` and `npm run dev`.
- Stop the database: `docker compose down`
