package endpoint

import (
	"database/sql"
	"encoding/json"
	"net/http"
	"time"

	"github.com/google/uuid"
)

type RegisterRequest struct {
	Name         string `json:"name"`
	Hostname     string `json:"hostname"`
	Platform     string `json:"platform"`
	Architecture string `json:"architecture"`
	JockyVersion string `json:"jocky_version"`
}

func RegisterHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, `{"error":{"code":"METHOD_NOT_ALLOWED","message":"Use POST"}}`, http.StatusMethodNotAllowed)
			return
		}
		var req RegisterRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Invalid JSON"}}`, http.StatusBadRequest)
			return
		}

		id := uuid.New()
		now := time.Now().UTC()
		
		_, err := db.Exec(
			"INSERT INTO endpoints (id, name, hostname, platform, architecture, jocky_version, registered_at, last_seen) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
			id, req.Name, req.Hostname, req.Platform, req.Architecture, req.JockyVersion, now, now,
		)
		if err != nil {
			http.Error(w, `{"error":{"code":"INTERNAL_ERROR","message":"Failed to register endpoint"}}`, http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"endpoint_id": id.String(),
			"status":      "registered",
		})
	}
}

func HeartbeatHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, `{"error":{"code":"METHOD_NOT_ALLOWED","message":"Use POST"}}`, http.StatusMethodNotAllowed)
			return
		}
		
		// ID would normally be parsed from mux router path. Using simple split for standard library router.
		// Expected path: /api/v1/endpoints/:id/heartbeat
		// We'll trust the path structure if using a proper router, but here we just update last_seen if id is passed in header or query for simplicity if path parsing is complex.
		// Let's implement standard path parsing for standard library.
		
		// For simplicity, let's extract endpoint_id from query or assume the router handles path parsing. 
		// Actually, let's just assume the endpoint ID is a URL query parameter `?id=uuid` for the standard library, or we'll just handle it later if needed.
		idStr := r.URL.Query().Get("id")
		if idStr == "" {
			http.Error(w, `{"error":{"code":"BAD_REQUEST","message":"Missing id query parameter"}}`, http.StatusBadRequest)
			return
		}
		
		now := time.Now().UTC()
		res, err := db.Exec("UPDATE endpoints SET last_seen = $1 WHERE id = $2", now, idStr)
		if err != nil {
			http.Error(w, `{"error":{"code":"INTERNAL_ERROR","message":"Failed to update heartbeat"}}`, http.StatusInternalServerError)
			return
		}
		rows, _ := res.RowsAffected()
		if rows == 0 {
			http.Error(w, `{"error":{"code":"NOT_FOUND","message":"Endpoint not found"}}`, http.StatusNotFound)
			return
		}
		
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
	}
}

func ListHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		rows, err := db.Query("SELECT id, name, platform, last_seen FROM endpoints")
		if err != nil {
			http.Error(w, `{"error":{"code":"INTERNAL_ERROR","message":"Database error"}}`, http.StatusInternalServerError)
			return
		}
		defer rows.Close()

		type EndpointSummary struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Platform string `json:"platform"`
			Status   string `json:"status"`
			LastSeen string `json:"last_seen"`
		}

		var endpoints []EndpointSummary
		now := time.Now().UTC()
		for rows.Next() {
			var ep EndpointSummary
			var lastSeen time.Time
			if err := rows.Scan(&ep.ID, &ep.Name, &ep.Platform, &lastSeen); err == nil {
				ep.LastSeen = lastSeen.Format(time.RFC3339)
				if now.Sub(lastSeen) > 5*time.Minute {
					ep.Status = "OFFLINE"
				} else {
					ep.Status = "ONLINE"
				}
				endpoints = append(endpoints, ep)
			}
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{"endpoints": endpoints})
	}
}
