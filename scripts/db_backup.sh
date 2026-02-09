#!/usr/bin/env bash
set -euo pipefail

project="${1:-uamappers}"

if [[ ! -f ".env" ]]; then
  echo "Missing .env in repo root" >&2
  exit 1
fi

# shellcheck disable=SC1091
set -a
. ./.env
set +a

mkdir -p backups

ts="$(date -u +"%Y%m%d-%H%M%S")"
out="backups/${POSTGRES_DB}-${ts}.sql.gz"

if ! docker compose -p "${project}" ps -q postgres >/dev/null 2>&1; then
  echo "docker compose project '${project}' not found (is postgres running?)" >&2
  exit 1
fi

cid="$(docker compose -p "${project}" ps -q postgres || true)"
if [[ -z "${cid}" ]]; then
  echo "postgres container is not running (run: just up)" >&2
  exit 1
fi

echo "Writing ${out}"
docker compose -p "${project}" exec -T postgres pg_dump -U "${POSTGRES_USER}" -d "${POSTGRES_DB}" --no-owner --no-privileges \
  | gzip -9 > "${out}"
