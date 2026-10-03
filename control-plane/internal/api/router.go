package api

import (
	"database/sql"
	"net/http"

	"jocky-control-plane/internal/auth"
	"jocky-control-plane/internal/endpoint"
	"jocky-control-plane/internal/health"
	"jocky-control-plane/internal/investigation"
)

func NewRouter(db *sql.DB, apiKey string) *http.ServeMux {
	mux := http.NewServeMux()

	// Public health endpoints
	mux.HandleFunc("/api/v1/health", health.HealthHandler())
	mux.HandleFunc("/api/v1/ready", health.ReadyHandler(db))

	// Authenticated routes
	authMux := http.NewServeMux()

	authMux.HandleFunc("/api/v1/endpoints/register", endpoint.RegisterHandler(db))
	authMux.HandleFunc("/api/v1/endpoints/heartbeat", endpoint.HeartbeatHandler(db)) // Requires ?id=uuid
	authMux.HandleFunc("/api/v1/endpoints", endpoint.ListHandler(db))

	authMux.HandleFunc("/api/v1/investigations/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodGet {
			investigation.GetHandler(db)(w, r)
		} else {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		}
	})

	authMux.HandleFunc("/api/v1/investigations", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			investigation.IngestHandler(db)(w, r)
		} else if r.Method == http.MethodGet {
			investigation.ListHandler(db)(w, r)
		} else {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		}
	})

	// Wrap auth routes with middleware
	mux.Handle("/api/v1/endpoints/", auth.Middleware(apiKey, authMux))
	mux.Handle("/api/v1/endpoints", auth.Middleware(apiKey, authMux))
	mux.Handle("/api/v1/investigations", auth.Middleware(apiKey, authMux))
	mux.Handle("/api/v1/investigations/", auth.Middleware(apiKey, authMux))

	return mux
}
