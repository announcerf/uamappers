#!/usr/bin/env bash
set -euo pipefail

project="${1:-uamappers}"
mode="${2:-full}" # full | schema | data
compose_file="infra/compose/dev/compose.yml"

env_file="infra/env/dev/app.env"

if [[ ! -f "${env_file}" ]]; then
  echo "Missing ${env_file}" >&2
  exit 1
fi

# shellcheck disable=SC1091
set -a
. "${env_file}"
set +a

mkdir -p backups

ts="$(date -u +"%Y%m%d-%H%M%S")"
suffix="${mode}"
out="backups/${POSTGRES_DB}-${ts}-${suffix}.sql.gz"

if ! docker compose -f "${compose_file}" -p "${project}" ps -q postgres >/dev/null 2>&1; then
  echo "docker compose project '${project}' not found (is postgres running?)" >&2
  exit 1
fi

cid="$(docker compose -f "${compose_file}" -p "${project}" ps -q postgres || true)"
if [[ -z "${cid}" ]]; then
  echo "postgres container is not running (run: just up)" >&2
  exit 1
fi

echo "Writing ${out}"
pg_dump_flags=()
case "${mode}" in
full) ;;
schema)
  pg_dump_flags+=(--schema-only)
  ;;
data)
  pg_dump_flags+=(--data-only)
  ;;
*)
  echo "Unknown mode: ${mode} (expected: full | schema | data)" >&2
  exit 1
  ;;
esac

pg_dump_cmd=(
  docker compose -f "${compose_file}" -p "${project}" exec -T postgres
  pg_dump
  -U "${POSTGRES_USER}"
  -d "${POSTGRES_DB}"
  --no-owner
  --no-privileges
)

if ((${#pg_dump_flags[@]} > 0)); then
  pg_dump_cmd+=("${pg_dump_flags[@]}")
fi

"${pg_dump_cmd[@]}" | gzip -9 >"${out}"
