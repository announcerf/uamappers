#!/usr/bin/env bash
set -euo pipefail

project="${1:-uamappers}"

# Best-effort cleanup. We prefer to be noisy/safe over clever.
docker compose -p "${project}" down --remove-orphans --volumes --rmi local || true
