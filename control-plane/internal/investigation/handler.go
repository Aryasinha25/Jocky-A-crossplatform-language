package investigation

import (
	"database/sql"
	"encoding/json"
	"net/http"
	"strconv"
	"strings"

	"github.com/google/uuid"

	"jocky-control-plane/internal/models"
	"jocky-control-plane/internal/storage"
)

func IngestHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, `{"error":{"code":"METHOD_NOT_ALLOWED","message":"Use POST"}}`, http.StatusMethodNotAllowed)
			return
		}

		var req models.IngestRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Invalid JSON"}}`, http.StatusBadRequest)
			return
		}

		// Ensure endpoint exists
		var epId string
		if err := db.QueryRow("SELECT id FROM endpoints WHERE id = $1", req.EndpointID).Scan(&epId); err != nil {
			http.Error(w, `{"error":{"code":"UNAUTHORIZED","message":"Invalid endpoint_id"}}`, http.StatusUnauthorized)
			return
		}

		// Upsert Investigation using storage layer (handles validation, tx, inserts)
		if err := storage.UpsertInvestigationTx(db, req.EndpointID, req.Investigation); err != nil {
			http.Error(w, `{"error":{"code":"INTERNAL_ERROR","message":"Failed to ingest investigation: `+err.Error()+`"}}`, http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{"status": "ingested"})
	}
}

func ListHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, `{"error":{"code":"METHOD_NOT_ALLOWED","message":"Use GET"}}`, http.StatusMethodNotAllowed)
			return
		}

		pageStr := r.URL.Query().Get("page")
		pageSizeStr := r.URL.Query().Get("page_size")

		page := 1
		if pageStr != "" {
			p, err := strconv.Atoi(pageStr)
			if err != nil || p < 1 {
				http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Invalid page value"}}`, http.StatusBadRequest)
				return
			}
			page = p
		}

		pageSize := 20
		if pageSizeStr != "" {
			ps, err := strconv.Atoi(pageSizeStr)
			if err != nil || ps < 1 || ps > 100 {
				http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Invalid page_size value (must be 1-100)"}}`, http.StatusBadRequest)
				return
			}
			pageSize = ps
		}

		resp, err := storage.ListInvestigations(db, page, pageSize)
		if err != nil {
			http.Error(w, `{"error":{"code":"INTERNAL_ERROR","message":"Failed to list investigations"}}`, http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}
}

func GetHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, `{"error":{"code":"METHOD_NOT_ALLOWED","message":"Use GET"}}`, http.StatusMethodNotAllowed)
			return
		}

		// Parse ID from URL path: /api/v1/investigations/{id}
		parts := strings.Split(r.URL.Path, "/")
		if len(parts) < 5 || parts[4] == "" {
			http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Missing investigation ID"}}`, http.StatusBadRequest)
			return
		}
		id := parts[4]

		if id == "" {
			http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Invalid investigation ID format"}}`, http.StatusBadRequest)
			return
		}

		// Ensure it's either a raw UUID or "inv-" + UUID
		idToParse := strings.TrimPrefix(id, "inv-")
		if _, err := uuid.Parse(idToParse); err != nil {
			http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Invalid investigation ID format"}}`, http.StatusBadRequest)
			return
		}

		inv, err := storage.GetInvestigationByID(db, id)
		if err != nil {
			http.Error(w, `{"error":{"code":"INTERNAL_ERROR","message":"Failed to retrieve investigation: `+err.Error()+`"}}`, http.StatusInternalServerError)
			return
		}
		if inv == nil {
			http.Error(w, `{"error":{"code":"NOT_FOUND","message":"Investigation not found"}}`, http.StatusNotFound)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(inv)
	}
}
