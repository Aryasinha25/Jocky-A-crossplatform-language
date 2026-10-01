package main

import (
	"fmt"
	"log"
	"net/http"

	"jocky-control-plane/internal/api"
	"jocky-control-plane/internal/config"
	"jocky-control-plane/internal/storage"
)

func main() {
	cfg := config.Load()

	db, err := storage.InitDB(cfg.DatabaseURL)
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}
	defer db.Close()

	if err := storage.RunMigrations(db, "migrations"); err != nil {
		log.Fatalf("Failed to run migrations: %v", err)
	}

	router := api.NewRouter(db, cfg.APIKey)

	addr := fmt.Sprintf("%s:%d", cfg.ServerHost, cfg.ServerPort)
	log.Printf("JOCKY Control Plane listening on %s", addr)
	
	if err := http.ListenAndServe(addr, router); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
