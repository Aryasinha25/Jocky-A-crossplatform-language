# UI Architecture

The Analyst Console bridges the final gap in the JOCKY Phase 6 infrastructure.

## Stack
- **Framework**: React (Vite)
- **Desktop Runtime**: Tauri (Rust WebView wrapper)
- **State**: React Hooks (No Redux needed for read-only flows)
- **Visualizations**: `reactflow`

## API Flow
1. User configures API endpoints in the Settings tab, specifying the Go Control Plane IP and Bearer token.
2. `api/client.ts` uses the browser `fetch` API for all outbound authenticated REST calls.
3. React renders the Dashboard, Endpoint Inventories, and nested `InvestigationDetail` layouts.
4. If demo mode is active, API requests dynamically reroute to synthetic datasets generated purely for mock UI testing.
