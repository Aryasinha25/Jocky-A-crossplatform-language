# JOCKY Analyst Console

Phase 6 introduces the JOCKY Analyst Console, a Tauri + React + TypeScript desktop application for visualizing forensic metadata.

## Overview
- **Read-Only**: The UI is explicitly read-only, fetching JSON investigations from the Phase 5 Go Control Plane.
- **Tauri Integration**: Leverages native WebView bindings rather than shipping Electron, saving memory and preventing arbitrary shell breakout via locked-down APIs.
- **React Flow**: Generates investigation graphs mapping topological evidence patterns.

## Building
Run these commands in the `ui` directory:
```bash
npm install
npm run dev
```

*Note: Requires Node.js and Rust installed. Tauri commands use standard Vite workflows under the hood.*
