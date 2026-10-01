use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use jocky_common::{InvestigationResult, InvestigationSummary, InvestigationTimeline};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ApiError,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

pub trait InvestigationStore: Send + Sync {
    fn list(&self) -> Result<Vec<InvestigationSummary>, String>;
    fn get(&self, id: &str) -> Result<Option<InvestigationResult>, String>;
    fn insert(&self, result: InvestigationResult) -> Result<(), String>;
}

pub struct InMemoryStore {
    investigations: RwLock<HashMap<String, InvestigationResult>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            investigations: RwLock::new(HashMap::new()),
        }
    }
}

impl InvestigationStore for InMemoryStore {
    fn list(&self) -> Result<Vec<InvestigationSummary>, String> {
        let guard = self.investigations.read().map_err(|_| "Lock error")?;
        let mut summaries = Vec::new();
        for (id, result) in guard.iter() {
            summaries.push(InvestigationSummary {
                id: id.clone(),
                target: result.target.clone(),
                platform: result.platform.clone(),
                start_time: result.start_time.clone(),
                end_time: result.end_time.clone(),
                finding_count: result.findings.len(),
                risk_score: result.risk_score.score,
                priority: result.risk_score.priority.clone(),
            });
        }
        Ok(summaries)
    }

    fn get(&self, id: &str) -> Result<Option<InvestigationResult>, String> {
        let guard = self.investigations.read().map_err(|_| "Lock error")?;
        Ok(guard.get(id).cloned())
    }

    fn insert(&self, result: InvestigationResult) -> Result<(), String> {
        let mut guard = self.investigations.write().map_err(|_| "Lock error")?;
        guard.insert(result.investigation_id.clone(), result);
        Ok(())
    }
}

pub struct AppState {
    pub store: Arc<dyn InvestigationStore>,
}

pub fn create_router(store: Arc<dyn InvestigationStore>) -> Router {
    let state = Arc::new(AppState { store });
    Router::new()
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/investigations", get(list_investigations_handler))
        .route("/api/v1/investigations/:id", get(get_investigation_handler))
        .route(
            "/api/v1/investigations/:id/graph",
            get(get_investigation_graph_handler),
        )
        .route(
            "/api/v1/investigations/:id/timeline",
            get(get_investigation_timeline_handler),
        )
        .route(
            "/api/v1/investigations/:id/findings",
            get(get_investigation_findings_handler),
        )
        .with_state(state)
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "jocky"
    }))
}

async fn list_investigations_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.store.list() {
        Ok(summaries) => (
            StatusCode::OK,
            Json(serde_json::json!({ "investigations": summaries })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INTERNAL_ERROR".to_string(),
                    message: "Failed to list investigations".to_string(),
                }
            })),
        ),
    }
}

async fn get_investigation_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.store.get(&id) {
        Ok(Some(inv)) => (StatusCode::OK, Json(serde_json::json!(inv))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INVESTIGATION_NOT_FOUND".to_string(),
                    message: format!("Investigation '{}' was not found.", id),
                }
            })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INTERNAL_ERROR".to_string(),
                    message: "Failed to get investigation".to_string(),
                }
            })),
        ),
    }
}

#[derive(Deserialize)]
struct GraphQuery {
    #[serde(rename = "type")]
    node_type: Option<String>,
}

async fn get_investigation_graph_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<GraphQuery>,
) -> impl IntoResponse {
    match state.store.get(&id) {
        Ok(Some(inv)) => {
            let mut graph = inv.graph.clone();
            if let Some(nt) = query.node_type {
                if nt == "evidence" {
                    graph
                        .nodes
                        .retain(|n| matches!(n, jocky_common::GraphNode::Evidence(_)));
                } else if nt == "finding" {
                    graph
                        .nodes
                        .retain(|n| matches!(n, jocky_common::GraphNode::Finding(_)));
                }
            }
            (StatusCode::OK, Json(serde_json::json!(graph)))
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INVESTIGATION_NOT_FOUND".to_string(),
                    message: format!("Investigation '{}' was not found.", id),
                }
            })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INTERNAL_ERROR".to_string(),
                    message: "Failed to get investigation graph".to_string(),
                }
            })),
        ),
    }
}

#[derive(Deserialize)]
struct TimelineQuery {
    limit: Option<usize>,
    event_type: Option<String>,
}

async fn get_investigation_timeline_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<TimelineQuery>,
) -> impl IntoResponse {
    match state.store.get(&id) {
        Ok(Some(inv)) => {
            let mut events = inv.timeline.events.clone();
            if let Some(et) = query.event_type {
                events.retain(|e| e.event_type.eq_ignore_ascii_case(&et));
            }
            if let Some(l) = query.limit {
                events.truncate(l);
            }
            (
                StatusCode::OK,
                Json(serde_json::json!(InvestigationTimeline { events })),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INVESTIGATION_NOT_FOUND".to_string(),
                    message: format!("Investigation '{}' was not found.", id),
                }
            })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INTERNAL_ERROR".to_string(),
                    message: "Failed to get investigation timeline".to_string(),
                }
            })),
        ),
    }
}

#[derive(Deserialize)]
struct FindingsQuery {
    severity: Option<String>,
}

async fn get_investigation_findings_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(query): Query<FindingsQuery>,
) -> impl IntoResponse {
    match state.store.get(&id) {
        Ok(Some(inv)) => {
            let mut findings = inv.findings.clone();
            if let Some(sev) = query.severity {
                findings.retain(|f| format!("{:?}", f.severity).eq_ignore_ascii_case(&sev));
            }
            (
                StatusCode::OK,
                Json(serde_json::json!({ "findings": findings })),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INVESTIGATION_NOT_FOUND".to_string(),
                    message: format!("Investigation '{}' was not found.", id),
                }
            })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!(ErrorResponse {
                error: ApiError {
                    code: "INTERNAL_ERROR".to_string(),
                    message: "Failed to get investigation findings".to_string(),
                }
            })),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_store() {
        let store = InMemoryStore::new();
        let inv = InvestigationResult {
            schema_version: "0.1".to_string(),
            investigation_id: "inv-123".to_string(),
            target: "LOCAL".to_string(),
            platform: "windows".to_string(),
            start_time: "".to_string(),
            end_time: "".to_string(),
            graph: jocky_common::InvestigationGraph {
                nodes: vec![],
                edges: vec![],
            },
            timeline: InvestigationTimeline { events: vec![] },
            findings: vec![],
            risk_score: jocky_common::RiskScore::new(0),
        };
        store.insert(inv).unwrap();
        let list = store.list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "inv-123");

        let fetched = store.get("inv-123").unwrap().unwrap();
        assert_eq!(fetched.investigation_id, "inv-123");
    }
}
