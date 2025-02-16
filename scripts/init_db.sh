#!/bin/bash

#set -x
#set -eo pipefail

export DB_USER=${POSTGRES_USER:="postgres"}
export DB_PASSWORD=${POSTGRES_PASSWORD:="password"}
export DB_HOST=${POSTGRES_HOST:="localhost"}
export DB_NAME=${POSTGRES_DB:="blog-db"}
export DB_PORT=${POSTGRES_PORT:=5432}

if [[ -z $SKIP_DOCKER ]]; then
  docker compose up -d
fi

export PGPASSWORD=${DB_PASSWORD}
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d postgres -c '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port: ${DB_PORT}"
