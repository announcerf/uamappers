#!/usr/bin/env bash
set -euo pipefail

project="${1:-uamappers}"
dump_file="${2:-}"
mode="${3:-inplace}" # inplace | clean
compose_file="infra/compose/dev/compose.yml"

if [[ -z "${dump_file}" ]]; then
  echo "Usage: db_restore.sh <project> <dump_file> [inplace|clean]" >&2
  exit 2
fi

if [[ ! -f "${dump_file}" ]]; then
  echo "Dump file not found: ${dump_file}" >&2
  exit 1
fi

env_file="infra/env/dev/app.env"

if [[ ! -f "${env_file}" ]]; then
  echo "Missing ${env_file}" >&2
  exit 1
fi

# shellcheck disable=SC1091
set -a
. "${env_file}"
set +a

cid="$(docker compose -f "${compose_file}" -p "${project}" ps -q postgres || true)"
if [[ -z "${cid}" ]]; then
  echo "postgres container is not running (run: just up)" >&2
  exit 1
fi

case "${mode}" in
inplace) ;;
clean)
  echo "Dropping schema public (destructive)"
  docker compose -f "${compose_file}" -p "${project}" exec -T postgres psql -v ON_ERROR_STOP=1 -U "${POSTGRES_USER}" -d "${POSTGRES_DB}" -c "DROP SCHEMA IF EXISTS public CASCADE; CREATE SCHEMA public;"
  ;;
*)
  echo "Unknown mode: ${mode} (expected: inplace | clean)" >&2
  exit 2
  ;;
esac

echo "Restoring from ${dump_file}"
if [[ "${dump_file}" == *.gz ]]; then
  gzip -dc "${dump_file}" |
    docker compose -f "${compose_file}" -p "${project}" exec -T postgres psql -v ON_ERROR_STOP=1 -U "${POSTGRES_USER}" -d "${POSTGRES_DB}"
else
  cat "${dump_file}" |
    docker compose -f "${compose_file}" -p "${project}" exec -T postgres psql -v ON_ERROR_STOP=1 -U "${POSTGRES_USER}" -d "${POSTGRES_DB}"
fi
