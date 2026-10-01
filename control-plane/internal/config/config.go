package config

import (
	"os"
	"strconv"
)

type Config struct {
	ServerHost  string
	ServerPort  int
	DatabaseURL string
	APIKey      string
}

func Load() *Config {
	portStr := os.Getenv("JOCKY_SERVER_PORT")
	port, err := strconv.Atoi(portStr)
	if err != nil {
		port = 8080
	}

	host := os.Getenv("JOCKY_SERVER_HOST")
	if host == "" {
		host = "127.0.0.1"
	}

	dbUrl := os.Getenv("JOCKY_DATABASE_URL")
	if dbUrl == "" {
		dbUrl = "postgres://postgres:postgres@localhost:5432/jocky?sslmode=disable"
	}

	apiKey := os.Getenv("JOCKY_API_KEY")
	if apiKey == "" {
		apiKey = "dev_secret_key_123" // Safe default for local dev only
	}

	return &Config{
		ServerHost:  host,
		ServerPort:  port,
		DatabaseURL: dbUrl,
		APIKey:      apiKey,
	}
}
