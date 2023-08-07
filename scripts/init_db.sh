#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v migrate)" ]; then
  echo >&2 "Error: golang-migrator is not installed."
  echo >&2 "Consult README to install."
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'jetty'
DB_NAME="${POSTGRES_DB:=mcs}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a migrations folder root directory has been set, otherwise default to '.' (current directory)
MIGRATIONS_ROOT_DIR="${POSTGRES_MIGRATIONS_ROOT_DIR:=.}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
      --name tea-agent \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d postgres \
      postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "mcs" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

export POSTGRESQL_URL='postgres://postgres:password@localhost:5432/mcs?sslmode=disable'
migrate -database ${POSTGRESQL_URL} -path ./migrations up

>&2 echo "Postgres migrations successfuly run"
