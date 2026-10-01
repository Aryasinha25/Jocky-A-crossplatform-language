# JOCKY Phase 6

JOCKY is a cross-platform Domain-Specific Language (DSL) and forensic analysis framework for authorized computer and network investigations.

## Objective

JOCKY aims to provide a unified, programmatic way to target, collect, correlate, and detect artifacts across multiple operating systems. It acts as an abstraction layer above raw OS forensics, enabling analysts to write consistent investigative logic regardless of the target environment.

## Phase 6 Capabilities
- **Tauri Analyst UI**: A standalone React application connecting strictly via REST.
- **Topological Visualizations**: Graphical rendering using `reactflow` bridging Findings and Evidence.
- **Chronological Tracing**: Sequence-based event mapping tracking endpoint anomalies.
- **Secure Architecture**: Strictly read-only application preventing accidental or malicious reverse-execution capabilities.

## Architecture

```text
                         JOCKY Endpoint
                              │
                              ▼
                       Rust Investigation
                              │
                              │ HTTPS
                              ▼
                    ┌─────────────────────┐
                    │ Go Control Plane    │
                    │                     │
                    │ Endpoint Registry   │
                    │ Investigation API   │
                    │ Authentication      │
                    │ Audit Logging       │
                    └─────────┬───────────┘
                              │
                              ▼
                         PostgreSQL
                              │
                              │ REST / JSON
                              ▼
                    ┌─────────────────────┐
                    │ Tauri Analyst App   │
                    │                     │
                    │ React + TypeScript  │
                    ├─────────────────────┤
                    │ Dashboard           │
                    │ Endpoints           │
                    │ Investigations      │
                    │ Findings            │
                    │ Timeline            │
                    │ Graph               │
                    │ Evidence            │
                    └─────────────────────┘
```

## Security Posture
JOCKY is read-only and explicitly designed for **defensive** and authorized forensic enumeration. It does not perform process injection, kernel subversion, or antivirus bypass. The Go Control Plane and Tauri Desktop UI **cannot** remotely execute code.

## Roadmap

- **Phase 7**: LLVM backend
- **Phase 8**: Advanced defensive memory telemetry
