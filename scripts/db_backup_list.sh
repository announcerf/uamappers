#!/usr/bin/env bash
set -euo pipefail

if [[ ! -d "backups" ]]; then
  echo "No backups/ directory" >&2
  exit 0
fi

ls -1 backups | sort

