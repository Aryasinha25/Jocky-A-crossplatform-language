package storage

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"jocky-control-plane/internal/models"
)

func UpsertInvestigationTx(db *sql.DB, epID string, inv models.InvestigationResult) error {
	// 1. Strict reference validation BEFORE COMMIT
	evidenceMap := make(map[string]bool)
	findingMap := make(map[string]bool)

	for _, n := range inv.Graph.Nodes {
		if n.Type == "evidence" || n.EvidenceType != "" { // Hacky way if type isn't perfectly set
			evidenceMap[n.ID] = true
		} else if n.Type == "finding" || n.RuleID != "" {
			findingMap[n.ID] = true
		}
	}
	// Trust findings slice as well
	for _, f := range inv.Findings {
		findingMap[f.ID] = true
		for _, eID := range f.EvidenceIDs {
			if !evidenceMap[eID] {
				return fmt.Errorf("finding %s references missing evidence %s", f.ID, eID)
			}
		}
	}

	for _, e := range inv.Graph.Edges {
		validSrc := evidenceMap[e.Source] || findingMap[e.Source]
		validTgt := evidenceMap[e.Target] || findingMap[e.Target]
		if !validSrc {
			return fmt.Errorf("edge references missing source %s", e.Source)
		}
		if !validTgt {
			return fmt.Errorf("edge references missing target %s", e.Target)
		}
	}

	for _, t := range inv.Timeline.Events {
		if t.EvidenceID != "" {
			if !evidenceMap[t.EvidenceID] && !findingMap[t.EvidenceID] {
				return fmt.Errorf("timeline references missing ID %s", t.EvidenceID)
			}
		}
	}

	tx, err := db.Begin()
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback() // Rollback on any error, no-op if committed

	// 2. Upsert Investigation Header
	_, err = tx.Exec(`
		INSERT INTO investigations (id, endpoint_id, schema_version, target, platform, start_time, end_time, risk_score, priority)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
		ON CONFLICT (endpoint_id, id) DO UPDATE 
		SET risk_score = EXCLUDED.risk_score, priority = EXCLUDED.priority
	`, inv.InvestigationID, epID, inv.SchemaVersion, inv.Target, inv.Platform,
		inv.StartTime, inv.EndTime, inv.RiskScore.Score, inv.RiskScore.Priority)
	if err != nil {
		return fmt.Errorf("upsert investigation: %w", err)
	}

	// 3. Upsert Evidence
	for _, n := range inv.Graph.Nodes {
		if n.Type == "evidence" || n.EvidenceType != "" {
			attrs, _ := json.Marshal(n.Attributes)
			_, err = tx.Exec(`
				INSERT INTO evidence (id, investigation_id, endpoint_id, host, timestamp, evidence_type, source, attributes_json)
				VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
				ON CONFLICT (id, investigation_id, endpoint_id) DO NOTHING
			`, n.ID, inv.InvestigationID, epID, n.Host, n.Timestamp, n.EvidenceType, n.Source, attrs)
			if err != nil {
				return fmt.Errorf("insert evidence %s: %w", n.ID, err)
			}
		}
	}

	// 4. Insert Findings
	for _, f := range inv.Findings {
		evIDs, _ := json.Marshal(f.EvidenceIDs)
		_, err = tx.Exec(`
			INSERT INTO findings (id, investigation_id, endpoint_id, rule_id, title, description, severity, confidence, evidence_ids, timestamp)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
			ON CONFLICT (id, investigation_id, endpoint_id) DO NOTHING
		`, f.ID, inv.InvestigationID, epID, f.RuleID, f.Title, f.Description, f.Severity, f.Confidence, evIDs, f.Timestamp)
		if err != nil {
			return fmt.Errorf("insert finding %s: %w", f.ID, err)
		}
	}

	// 5. Insert Relationships
	// clear existing for idempotency? Yes, or just avoid duplicates.
	// For simplicity, delete existing relationships for this investigation first.
	_, err = tx.Exec(`DELETE FROM relationships WHERE investigation_id = $1 AND endpoint_id = $2`, inv.InvestigationID, epID)
	if err != nil {
		return fmt.Errorf("clear relationships: %w", err)
	}

	for _, edge := range inv.Graph.Edges {
		_, err = tx.Exec(`
			INSERT INTO relationships (investigation_id, endpoint_id, source_id, relationship_type, target_id)
			VALUES ($1, $2, $3, $4, $5)
		`, inv.InvestigationID, epID, edge.Source, edge.Relationship, edge.Target)
		if err != nil {
			return fmt.Errorf("insert relationship: %w", err)
		}
	}

	// 6. Insert Timeline Events
	_, err = tx.Exec(`DELETE FROM timeline_events WHERE investigation_id = $1 AND endpoint_id = $2`, inv.InvestigationID, epID)
	if err != nil {
		return fmt.Errorf("clear timeline: %w", err)
	}

	for _, t := range inv.Timeline.Events {
		_, err = tx.Exec(`
			INSERT INTO timeline_events (investigation_id, endpoint_id, timestamp, event_type, evidence_id, host, description)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
		`, inv.InvestigationID, epID, t.Timestamp, t.EventType, t.EvidenceID, t.Host, t.Description)
		if err != nil {
			return fmt.Errorf("insert timeline event: %w", err)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}
	return nil
}
