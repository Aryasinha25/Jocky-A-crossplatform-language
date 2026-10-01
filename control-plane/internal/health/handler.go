package health

import (
	"database/sql"
	"encoding/json"
	"net/http"
)

func HealthHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"status":  "ok",
			"service": "jocky-control-plane",
			"version": "0.1.0",
		})
	}
}

func ReadyHandler(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		if err := db.Ping(); err != nil {
			w.WriteHeader(http.StatusServiceUnavailable)
			json.NewEncoder(w).Encode(map[string]string{
				"status":  "error",
				"service": "postgres",
				"message": err.Error(),
			})
			return
		}
		json.NewEncoder(w).Encode(map[string]string{
			"status":  "ready",
			"service": "jocky-control-plane",
		})
	}
}
