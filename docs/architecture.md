# Architecture

JOCKY is built around a modular compilation and execution pipeline:

## Pipeline

1. **Source Code**: `.jky` text files containing the DSL.
2. **Lexer/Parser**: Breaks the source into semantic tokens and assembles them into an Abstract Syntax Tree (AST) representing operations.
3. **IR**: An intermediate representation that normalizes operations for cross-platform execution.
4. **Runtime**: Coordinates the collection and detection phases.
5. **Collector Registry**: A platform-detection layer routing collection to OS implementations.
6. **Platform Collectors**: Safely gather metadata using OS APIs or cross-platform libraries (e.g. `sysinfo`), outputting normalized evidence.
7. **Detection Engine**: Analyzes collected evidence against structured YAML deterministic rules.
8. **Correlation & Graph Engine**: Assembles Findings and Evidence into an `InvestigationGraph` using Nodes and explicit `RelationshipType` Edges.
9. **Timeline Engine**: Extrapolates chronologically sorted `TimelineEvent` records based on the graph.
10. **Go Control Plane (Phase 5)**: A standalone `net/http` backend responsible for identity registry, authentication, investigation ingestion, and PostgreSQL serialization for future external Analyst consumers.
11. **Tauri Analyst UI (Phase 6)**: A secure, read-only desktop interface querying the Go Control Plane to visualize complex incident response topologies.

## Security Constraints
The architecture is bounded to strictly defensive enumeration. Remote Command Execution, Payload Drop, EDR evasion, and Process Injection are completely prohibited by design, guaranteeing safe utilization for authorized analysts.
