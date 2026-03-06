#!/usr/bin/env sh
set -eu

STACK_DIR="${STACK_DIR:-/opt/uamappers/prod}"
PROJECT_NAME="${COMPOSE_PROJECT_NAME:-uamappers-prod}"

cd "$STACK_DIR"

if [ ! -f "compose.yml" ] || [ ! -f "compose.worker.yml" ]; then
  echo "compose files were not found in $STACK_DIR"
  exit 1
fi

if [ ! -f "app.env" ]; then
  echo "app.env was not found in $STACK_DIR"
  exit 1
fi

# shellcheck disable=SC1091
set -a
. ./app.env
set +a

if [ -z "${UAMAPPERS_WORKER_IMAGE:-}" ]; then
  echo "UAMAPPERS_WORKER_IMAGE is required"
  exit 1
fi

export COMPOSE_PROJECT_NAME="$PROJECT_NAME"

docker compose -f compose.yml up -d postgres
docker compose -f compose.yml -f compose.worker.yml pull worker
docker compose -f compose.yml -f compose.worker.yml run --rm worker

echo "weekly worker run finished successfully"
