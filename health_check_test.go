package main

import (
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"testing"
)

func spawnApp() config {
	cfg := config{
		port: 4000,
		env:  "development",
	}

	logger := slog.New(slog.NewTextHandler(os.Stdout, nil))

	svr := Server(cfg, logger)

	go func() {
		svr.ListenAndServe()
	}()

	return cfg
}

func TestHealthCheck(t *testing.T) {
	cfg := spawnApp()
	t.Run("returns 200 Ok for health check", func(t *testing.T) {
		resp, _ := http.Get(fmt.Sprintf("http://localhost:%d/v1/healthcheck", cfg.port))
		want := 200

		if resp.StatusCode != want {
			t.Errorf("got %v want %v", resp.StatusCode, want)
		}
	})
}
