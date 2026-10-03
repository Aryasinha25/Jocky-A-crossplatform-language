package api_test

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"os"
	"os/exec"
	"testing"

	"github.com/google/uuid"

	"jocky-control-plane/internal/models"
)

func TestE2ERustSubmission(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	server := setupTestServer(t, db)
	defer server.Close()

	epID := createTestEndpoint(t, db)

	// Create a dummy JOCKY script
	wd, _ := os.Getwd()
	scriptPath := wd + "/test_e2e.jky"
	scriptContent := "TARGET \"LOCAL\"\nGENERATE REPORT\n"
	err := os.WriteFile(scriptPath, []byte(scriptContent), 0644)
	if err != nil {
		t.Fatalf("failed to create script: %v", err)
	}
	defer os.Remove(scriptPath)
	defer os.Remove("investigation.json")
	defer os.Remove("timeline.json")

	// Ensure jocky-cli is built (we assume 'cargo build' was run beforehand by the pipeline, but we can use cargo run to be safe)
	cmd := exec.Command("cargo", "run", "--bin", "jocky-cli", "--", "run", scriptPath, "--submit")
	cmd.Dir = ".." // run from workspace root
	cmd.Env = append(os.Environ(),
		"JOCKY_API_URL="+server.URL,
		"JOCKY_API_KEY=test-secret",
		"JOCKY_ENDPOINT_ID="+epID,
	)

	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("cargo run failed: %v\nOutput:\n%s", err, output)
	}

	// Verify it submitted successfully
	if !contains(string(output), "Successfully submitted investigation") {
		t.Fatalf("Submission output not found in:\n%s", output)
	}

	// Wait, we need the investigation ID to fetch it.
	// Since we don't parse the CLI output for ID easily without regex, we can just List investigations.
	req, _ := http.NewRequest("GET", server.URL+"/api/v1/investigations", nil)
	req.Header.Set("Authorization", "Bearer test-secret")
	resp, err := http.DefaultClient.Do(req)
	if err != nil || resp.StatusCode != 200 {
		t.Fatalf("failed to list investigations")
	}
	defer resp.Body.Close()

	var list models.ListInvestigationsResponse
	if err := json.NewDecoder(resp.Body).Decode(&list); err != nil {
		t.Fatalf("decode failed: %v", err)
	}

	if list.Total == 0 {
		t.Fatalf("Investigation was not saved to DB")
	}

	// Fetch it by ID to verify retrieval
	invID := list.Items[0].ID
	req, _ = http.NewRequest("GET", server.URL+"/api/v1/investigations/"+invID, nil)
	req.Header.Set("Authorization", "Bearer test-secret")
	resp, err = http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("failed to do request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		body, _ := io.ReadAll(resp.Body)
		t.Fatalf("Failed to retrieve investigation by ID %s: %d\nBody: %s", invID, resp.StatusCode, string(body))
	}

	var retrieved models.InvestigationResult
	if err := json.NewDecoder(resp.Body).Decode(&retrieved); err != nil {
		t.Fatalf("decode retrieved failed: %v", err)
	}

	if retrieved.InvestigationID != invID {
		t.Errorf("expected ID %s, got %s", invID, retrieved.InvestigationID)
	}
}

// Check for sub-string in a string
func contains(s, substr string) bool {
	return len(s) >= len(substr) && func() bool {
		for i := 0; i <= len(s)-len(substr); i++ {
			if s[i:i+len(substr)] == substr {
				return true
			}
		}
		return false
	}()
}

func TestDuplicateSubmission(t *testing.T) {
	db, cleanup := getTestDB(t)
	defer cleanup()

	server := setupTestServer(t, db)
	defer server.Close()

	epID := createTestEndpoint(t, db)

	inv := buildValidInvestigation(uuid.New().String())
	payload := models.IngestRequest{
		EndpointID:    epID,
		Investigation: inv,
	}

	body, _ := json.Marshal(payload)

	// First submission
	req, _ := http.NewRequest("POST", server.URL+"/api/v1/investigations", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer test-secret")
	resp, err := http.DefaultClient.Do(req)
	if err != nil || resp.StatusCode != 200 {
		t.Fatalf("First submission failed: %v", resp)
	}
	resp.Body.Close()

	// Second submission (Duplicate)
	req2, _ := http.NewRequest("POST", server.URL+"/api/v1/investigations", bytes.NewReader(body))
	req2.Header.Set("Authorization", "Bearer test-secret")
	resp2, err := http.DefaultClient.Do(req2)
	if err != nil || resp2.StatusCode != 200 {
		t.Fatalf("Second submission failed (Idempotency violation): %v", resp2)
	}
	resp2.Body.Close()

	// Verify no duplicates created
	var count int
	db.QueryRow("SELECT COUNT(*) FROM investigations WHERE id = $1", inv.InvestigationID).Scan(&count)
	if count != 1 {
		t.Errorf("Expected exactly 1 investigation row, found %d", count)
	}

	// Verify child records are not duplicated
	var evCount int
	db.QueryRow("SELECT COUNT(*) FROM evidence WHERE investigation_id = $1", inv.InvestigationID).Scan(&evCount)
	if evCount != 1 {
		t.Errorf("Expected exactly 1 evidence row, found %d", evCount)
	}

	var findingsCount int
	db.QueryRow("SELECT COUNT(*) FROM findings WHERE investigation_id = $1", inv.InvestigationID).Scan(&findingsCount)
	if findingsCount != 1 {
		t.Errorf("Expected exactly 1 finding row, found %d", findingsCount)
	}

	// Fetch it by ID to verify retrieval is equivalent
	req3, _ := http.NewRequest("GET", server.URL+"/api/v1/investigations/"+inv.InvestigationID, nil)
	req3.Header.Set("Authorization", "Bearer test-secret")
	resp3, err := http.DefaultClient.Do(req3)
	if err != nil || resp3.StatusCode != 200 {
		t.Fatalf("Failed to retrieve investigation")
	}
	defer resp3.Body.Close()

	var retrieved models.InvestigationResult
	if err := json.NewDecoder(resp3.Body).Decode(&retrieved); err != nil {
		t.Fatalf("decode retrieved failed: %v", err)
	}

	if retrieved.InvestigationID != inv.InvestigationID {
		t.Errorf("Payload mismatch: expected ID %s, got %s", inv.InvestigationID, retrieved.InvestigationID)
	}
	if len(retrieved.Graph.Nodes) != 2 {
		t.Errorf("Payload mismatch: expected 2 nodes, got %d", len(retrieved.Graph.Nodes))
	}
}
