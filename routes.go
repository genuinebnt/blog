package main

import (
	"net/http"

	"github.com/julienschmidt/httprouter"
)

func Router() http.Handler {
	router := httprouter.New()
	router.HandlerFunc(http.MethodGet, "/v1/healthcheck", healthCheckHandler)
	return router
}
