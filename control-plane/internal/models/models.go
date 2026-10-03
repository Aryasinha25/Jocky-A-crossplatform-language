package models

type GraphNode struct {
	Type         string                 `json:"type"`
	ID           string                 `json:"id"`
	Host         string                 `json:"host,omitempty"`
	Timestamp    string                 `json:"timestamp"`
	EvidenceType string                 `json:"evidence_type,omitempty"`
	Source       string                 `json:"source,omitempty"`
	Attributes   map[string]interface{} `json:"attributes,omitempty"`
	RuleID       string                 `json:"rule_id,omitempty"`
	Title        string                 `json:"title,omitempty"`
	Severity     string                 `json:"severity,omitempty"`
	Confidence   float64                `json:"confidence,omitempty"`
}

type GraphEdge struct {
	Source       string `json:"source"`
	Relationship string `json:"relationship"`
	Target       string `json:"target"`
}

type TimelineEvent struct {
	Timestamp   string `json:"timestamp"`
	EventType   string `json:"event_type"`
	EvidenceID  string `json:"evidence_id"`
	Host        string `json:"host"`
	Description string `json:"description"`
}

type Finding struct {
	ID          string   `json:"id"`
	RuleID      string   `json:"rule_id"`
	Title       string   `json:"title"`
	Description string   `json:"description"`
	Severity    string   `json:"severity"`
	Confidence  float64  `json:"confidence"`
	EvidenceIDs []string `json:"evidence_ids"`
	Timestamp   string   `json:"timestamp"`
}

type InvestigationResult struct {
	SchemaVersion   string `json:"schema_version"`
	InvestigationID string `json:"investigation_id"`
	Target          string `json:"target"`
	Platform        string `json:"platform"`
	StartTime       string `json:"start_time"`
	EndTime         string `json:"end_time"`
	RiskScore       struct {
		Score    int    `json:"score"`
		Priority string `json:"priority"`
	} `json:"risk_score"`
	Graph struct {
		Nodes []GraphNode `json:"nodes"`
		Edges []GraphEdge `json:"edges"`
	} `json:"graph"`
	Timeline struct {
		Events []TimelineEvent `json:"events"`
	} `json:"timeline"`
	Findings []Finding `json:"findings"`
}

type IngestRequest struct {
	EndpointID    string              `json:"endpoint_id"`
	Investigation InvestigationResult `json:"investigation"`
}

type ListInvestigationItem struct {
	ID         string `json:"id"`
	EndpointID string `json:"endpoint_id"`
	Target     string `json:"target"`
	Platform   string `json:"platform"`
	StartTime  string `json:"start_time"`
	RiskScore  int    `json:"risk_score"`
	Priority   string `json:"priority"`
	CreatedAt  string `json:"created_at"`
}

type ListInvestigationsResponse struct {
	Items      []ListInvestigationItem `json:"items"`
	Page       int                     `json:"page"`
	PageSize   int                     `json:"page_size"`
	Total      int                     `json:"total"`
	TotalPages int                     `json:"total_pages"`
}
