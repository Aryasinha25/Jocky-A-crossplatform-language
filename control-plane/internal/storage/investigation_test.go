package storage_test

import (
	"database/sql"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/google/uuid"
	_ "github.com/lib/pq"

	"jocky-control-plane/internal/models"
	"jocky-control-plane/internal/storage"
)

func getTestDB(t *testing.T) (*sql.DB, func()) {
	dbUrl := os.Getenv("TEST_DATABASE_URL")
	if dbUrl == "" {
		t.Skip("TEST_DATABASE_URL not set. Skipping integration tests.")
	}
	db, err := sql.Open("postgres", dbUrl)
	if err != nil {
		t.Fatalf("failed to connect to db: %v", err)
	}

	// Make sure schema is created
	storage.RunMigrations(db, "../../migrations")

	return db, func() {
		db.Close()
	}
}

func createTestEndpoint(t *testing.T, db *sql.DB) string {
	id := uuid.New().String()
	_, err := db.Exec("INSERT INTO endpoints (id, name, hostname, platform, architecture, jocky_version, registered_at, last_seen) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
		id, "test-ep", "test-host", "windows", "x86_64", "0.1", time.Now().UTC(), time.Now().UTC())
	if err != nil {
		t.Fatalf("failed to create test endpoint: %v", err)
	}
	t.Cleanup(func() {
		_, _ = db.Exec("DELETE FROM endpoints WHERE id = $1", id)
	})

	return id
}

func buildValidInvestigation() models.InvestigationResult {
	return models.InvestigationResult{
		SchemaVersion:   "0.1",
		InvestigationID: uuid.New().String(),
		Target:          "TEST",
		Platform:        "windows",
		StartTime:       time.Now().UTC().Format(time.RFC3339),
		EndTime:         time.Now().UTC().Format(time.RFC3339),
		RiskScore: struct {
			Score    int    `json:"score"`
			Priority string `json:"priority"`
		}{
			Score:    25,
			Priority: "MEDIUM",
		},
		Graph: struct {
			Nodes []models.GraphNode `json:"nodes"`
			Edges []models.GraphEdge `json:"edges"`
		}{
			Nodes: []models.GraphNode{
				{
					Type:         "evidence",
					ID:           "ev-1",
					Host:         "host-1",
					Timestamp:    time.Now().UTC().Format(time.RFC3339),
					EvidenceType: "process",
					Source:       "windows",
				},
				{
					Type:       "finding",
					ID:         "find-1",
					RuleID:     "rule-1",
					Title:      "Test Finding",
					Severity:   "High",
					Confidence: 0.9,
					Timestamp:  time.Now().UTC().Format(time.RFC3339),
				},
			},
			Edges: []models.GraphEdge{
				{
					Source:       "find-1",
					Relationship: "GeneratedFinding",
					Target:       "ev-1",
				},
			},
		},
		Findings: []models.Finding{
			{
				ID:          "find-1",
				RuleID:      "rule-1",
				Title:       "Test Finding",
				Description: "Desc",
				Severity:    "High",
				Confidence:  0.9,
				EvidenceIDs: []string{"ev-1"},
				Timestamp:   time.Now().UTC().Format(time.RFC3339),
			},
		},
		Timeline: struct {
			Events []models.TimelineEvent `json:"events"`
		}{
			Events: []models.TimelineEvent{
				{
					Timestamp:   time.Now().UTC().Format(time.RFC3339),
					EventType:   "PROCESS",
					EvidenceID:  "ev-1",
					Host:        "host-1",
					Description: "Started",
				},
			},
		},
	}
}

func TestUpsertInvestigationTx_Success(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	epID := createTestEndpoint(t, db)
	inv := buildValidInvestigation()

	err := storage.UpsertInvestigationTx(db, epID, inv)
	if err != nil {
		t.Fatalf("expected success, got %v", err)
	}
}

func TestUpsertInvestigationTx_Idempotent(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	epID := createTestEndpoint(t, db)
	inv := buildValidInvestigation()

	err := storage.UpsertInvestigationTx(db, epID, inv)
	if err != nil {
		t.Fatalf("expected success on first insert, got %v", err)
	}

	// Second insert should succeed and not duplicate
	err = storage.UpsertInvestigationTx(db, epID, inv)
	if err != nil {
		t.Fatalf("expected success on duplicate insert, got %v", err)
	}

	var count int
	db.QueryRow("SELECT COUNT(*) FROM investigations WHERE id=$1", inv.InvestigationID).Scan(&count)
	if count != 1 {
		t.Fatalf("expected 1 investigation, got %d", count)
	}
}

func TestUpsertInvestigationTx_InvalidEvidenceRef(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	epID := createTestEndpoint(t, db)
	inv := buildValidInvestigation()

	// Corrupt the finding's evidence ID
	inv.Findings[0].EvidenceIDs = []string{"non-existent-ev"}

	err := storage.UpsertInvestigationTx(db, epID, inv)
	if err == nil {
		t.Fatalf("expected error due to invalid evidence reference, got nil")
	}
	if !strings.Contains(err.Error(), "references missing evidence") {
		t.Fatalf("unexpected error message: %v", err)
	}
}

func TestUpsertInvestigationTx_InvalidRelationshipRef(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	epID := createTestEndpoint(t, db)
	inv := buildValidInvestigation()

	// Corrupt the edge's target
	inv.Graph.Edges[0].Target = "non-existent-node"

	err := storage.UpsertInvestigationTx(db, epID, inv)
	if err == nil {
		t.Fatalf("expected error due to invalid edge reference, got nil")
	}
	if !strings.Contains(err.Error(), "references missing target") {
		t.Fatalf("unexpected error message: %v", err)
	}
}

func TestUpsertInvestigationTx_SQLInjection(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	epID := createTestEndpoint(t, db)
	inv := buildValidInvestigation()

	// Insert malicious SQL in a string field. If concatenated, this might cause syntax errors or truncate tables.
	// We verify that it gets treated as a literal string by ensuring the insert succeeds and a rollback doesn't randomly happen due to syntax errors.
	inv.Target = "'; DROP TABLE investigations; --"

	err := storage.UpsertInvestigationTx(db, epID, inv)
	if err != nil {
		t.Fatalf("expected success despite malicious string (should be parameterized), got %v", err)
	}

	// Verify the table still exists
	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM investigations").Scan(&count)
	if err != nil {
		t.Fatalf("table might have been dropped or query failed: %v", err)
	}
}

func TestUpsertInvestigationTx_RollbackOnChildFailure(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	epID := createTestEndpoint(t, db)
	inv := buildValidInvestigation()

	// We pass a valid Investigation to validation, but cause a DB-level failure
	// To cause a DB-level failure, we can corrupt the JSON string in timeline?
	// Or we can corrupt a timestamp format that Postgres rejects, but Go might pass.
	inv.Timeline.Events[0].Timestamp = "NOT A TIMESTAMP"

	err := storage.UpsertInvestigationTx(db, epID, inv)
	if err == nil {
		t.Fatalf("expected db insert error, got nil")
	}

	// Verify the investigation was rolled back (0 rows in investigations)
	var count int
	db.QueryRow("SELECT COUNT(*) FROM investigations WHERE id=$1", inv.InvestigationID).Scan(&count)
	if count != 0 {
		t.Fatalf("expected rollback to leave 0 investigations, got %d", count)
	}
}
