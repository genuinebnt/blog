package main

import (
	"fmt"
	"log/slog"
	"net/http"
	"time"
)

func Server(cfg config, logger *slog.Logger) *http.Server {
	return &http.Server{
		Addr:         fmt.Sprintf(":%d", cfg.port),
		Handler:      Router(),
		IdleTimeout:  time.Minute,
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
		ErrorLog:     slog.NewLogLogger(logger.Handler(), slog.LevelError),
	}
}
