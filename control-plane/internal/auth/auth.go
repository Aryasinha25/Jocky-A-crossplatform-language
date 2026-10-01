package auth

import (
	"net/http"
	"strings"
)

func Middleware(apiKey string, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if authHeader == "" {
			http.Error(w, `{"error":{"code":"UNAUTHORIZED","message":"Missing Authorization header"}}`, http.StatusUnauthorized)
			return
		}

		parts := strings.Split(authHeader, " ")
		if len(parts) != 2 || parts[0] != "Bearer" {
			http.Error(w, `{"error":{"code":"UNAUTHORIZED","message":"Invalid Authorization header format"}}`, http.StatusUnauthorized)
			return
		}

		if parts[1] != apiKey {
			http.Error(w, `{"error":{"code":"UNAUTHORIZED","message":"Invalid API key"}}`, http.StatusUnauthorized)
			return
		}

		next.ServeHTTP(w, r)
	})
}
