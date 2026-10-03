package storage

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"jocky-control-plane/internal/models"
)

// GetInvestigationByID fetches a complete investigation by its ID and validates its internal reference integrity natively.
func GetInvestigationByID(db *sql.DB, id string) (*models.InvestigationResult, error) {
	// Query the investigation metadata
	row := db.QueryRow(`
		SELECT id, endpoint_id, schema_version, target, platform, start_time, end_time, risk_score, priority
		FROM investigations
		WHERE id = $1
	`, id)

	var inv models.InvestigationResult
	var epID string
	var startT, endT time.Time

	err := row.Scan(&inv.InvestigationID, &epID, &inv.SchemaVersion, &inv.Target, &inv.Platform, &startT, &endT, &inv.RiskScore.Score, &inv.RiskScore.Priority)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // Indicate not found
		}
		return nil, fmt.Errorf("query investigation: %w", err)
	}

	inv.StartTime = startT.Format(time.RFC3339)
	inv.EndTime = endT.Format(time.RFC3339)

	// Fetch evidence
	evidenceRows, err := db.Query(`
		SELECT id, host, timestamp, evidence_type, source, attributes_json
		FROM evidence
		WHERE investigation_id = $1
	`, id)
	if err != nil {
		return nil, fmt.Errorf("query evidence: %w", err)
	}
	defer evidenceRows.Close()

	var nodes []models.GraphNode
	for evidenceRows.Next() {
		var n models.GraphNode
		n.Type = "evidence"
		var t time.Time
		var attrsJSON []byte
		if err := evidenceRows.Scan(&n.ID, &n.Host, &t, &n.EvidenceType, &n.Source, &attrsJSON); err != nil {
			return nil, fmt.Errorf("scan evidence: %w", err)
		}
		n.Timestamp = t.Format(time.RFC3339)
		if len(attrsJSON) > 0 {
			if err := json.Unmarshal(attrsJSON, &n.Attributes); err != nil {
				return nil, fmt.Errorf("unmarshal evidence attributes: %w", err)
			}
		}
		nodes = append(nodes, n)
	}

	// Fetch findings
	findingsRows, err := db.Query(`
		SELECT id, rule_id, title, description, severity, confidence, evidence_ids, timestamp
		FROM findings
		WHERE investigation_id = $1
	`, id)
	if err != nil {
		return nil, fmt.Errorf("query findings: %w", err)
	}
	defer findingsRows.Close()

	var findings []models.Finding
	for findingsRows.Next() {
		var f models.Finding
		var t time.Time
		var evIDsJSON []byte
		if err := findingsRows.Scan(&f.ID, &f.RuleID, &f.Title, &f.Description, &f.Severity, &f.Confidence, &evIDsJSON, &t); err != nil {
			return nil, fmt.Errorf("scan findings: %w", err)
		}
		f.Timestamp = t.Format(time.RFC3339)
		if len(evIDsJSON) > 0 {
			if err := json.Unmarshal(evIDsJSON, &f.EvidenceIDs); err != nil {
				return nil, fmt.Errorf("unmarshal finding evidence_ids: %w", err)
			}
		}
		findings = append(findings, f)

		// Reconstruct Finding node in the graph
		nodes = append(nodes, models.GraphNode{
			Type:       "finding",
			ID:         f.ID,
			RuleID:     f.RuleID,
			Title:      f.Title,
			Severity:   f.Severity,
			Confidence: f.Confidence,
			Timestamp:  f.Timestamp,
		})
	}

	// Fetch relationships
	relRows, err := db.Query(`
		SELECT source_id, relationship_type, target_id
		FROM relationships
		WHERE investigation_id = $1
	`, id)
	if err != nil {
		return nil, fmt.Errorf("query relationships: %w", err)
	}
	defer relRows.Close()

	var edges []models.GraphEdge
	for relRows.Next() {
		var e models.GraphEdge
		if err := relRows.Scan(&e.Source, &e.Relationship, &e.Target); err != nil {
			return nil, fmt.Errorf("scan relationship: %w", err)
		}
		edges = append(edges, e)
	}

	// Fetch timeline events
	tlRows, err := db.Query(`
		SELECT timestamp, event_type, evidence_id, host, description
		FROM timeline_events
		WHERE investigation_id = $1
		ORDER BY timestamp ASC
	`, id)
	if err != nil {
		return nil, fmt.Errorf("query timeline: %w", err)
	}
	defer tlRows.Close()

	var events []models.TimelineEvent
	for tlRows.Next() {
		var e models.TimelineEvent
		var t time.Time
		if err := tlRows.Scan(&t, &e.EventType, &e.EvidenceID, &e.Host, &e.Description); err != nil {
			return nil, fmt.Errorf("scan timeline: %w", err)
		}
		e.Timestamp = t.Format(time.RFC3339)
		events = append(events, e)
	}

	inv.Graph.Nodes = nodes
	if inv.Graph.Nodes == nil {
		inv.Graph.Nodes = []models.GraphNode{}
	}
	inv.Graph.Edges = edges
	if inv.Graph.Edges == nil {
		inv.Graph.Edges = []models.GraphEdge{}
	}
	inv.Timeline.Events = events
	if inv.Timeline.Events == nil {
		inv.Timeline.Events = []models.TimelineEvent{}
	}
	inv.Findings = findings
	if inv.Findings == nil {
		inv.Findings = []models.Finding{}
	}

	// 4. Validate reference integrity
	evidenceMap := make(map[string]bool)
	findingMap := make(map[string]bool)
	for _, n := range nodes {
		if n.Type == "evidence" {
			evidenceMap[n.ID] = true
		} else if n.Type == "finding" {
			findingMap[n.ID] = true
		}
	}

	for _, f := range findings {
		for _, eID := range f.EvidenceIDs {
			if !evidenceMap[eID] {
				return nil, fmt.Errorf("corrupted data: finding %s references missing evidence %s", f.ID, eID)
			}
		}
	}

	for _, e := range edges {
		if !evidenceMap[e.Source] && !findingMap[e.Source] {
			return nil, fmt.Errorf("corrupted data: edge references missing source %s", e.Source)
		}
		if !evidenceMap[e.Target] && !findingMap[e.Target] {
			return nil, fmt.Errorf("corrupted data: edge references missing target %s", e.Target)
		}
	}

	for _, t := range events {
		if t.EvidenceID != "" {
			if !evidenceMap[t.EvidenceID] && !findingMap[t.EvidenceID] {
				return nil, fmt.Errorf("corrupted data: timeline references missing ID %s", t.EvidenceID)
			}
		}
	}

	return &inv, nil
}

// ListInvestigations returns a paginated list of investigations ordered by created_at DESC, id DESC.
func ListInvestigations(db *sql.DB, page, pageSize int) (*models.ListInvestigationsResponse, error) {
	if page < 1 {
		page = 1
	}
	if pageSize < 1 || pageSize > 100 {
		pageSize = 20
	}
	offset := (page - 1) * pageSize

	var total int
	err := db.QueryRow("SELECT COUNT(*) FROM investigations").Scan(&total)
	if err != nil {
		return nil, fmt.Errorf("count investigations: %w", err)
	}

	rows, err := db.Query(`
		SELECT id, endpoint_id, target, platform, start_time, risk_score, priority, created_at
		FROM investigations
		ORDER BY created_at DESC, id DESC
		LIMIT $1 OFFSET $2
	`, pageSize, offset)
	if err != nil {
		return nil, fmt.Errorf("query investigations: %w", err)
	}
	defer rows.Close()

	var items []models.ListInvestigationItem
	for rows.Next() {
		var i models.ListInvestigationItem
		var startT, createdT time.Time
		if err := rows.Scan(&i.ID, &i.EndpointID, &i.Target, &i.Platform, &startT, &i.RiskScore, &i.Priority, &createdT); err != nil {
			return nil, fmt.Errorf("scan investigation: %w", err)
		}
		i.StartTime = startT.Format(time.RFC3339)
		i.CreatedAt = createdT.Format(time.RFC3339)
		items = append(items, i)
	}

	if items == nil {
		items = []models.ListInvestigationItem{}
	}

	totalPages := total / pageSize
	if total%pageSize != 0 {
		totalPages++
	}

	return &models.ListInvestigationsResponse{
		Items:      items,
		Page:       page,
		PageSize:   pageSize,
		Total:      total,
		TotalPages: totalPages,
	}, nil
}
