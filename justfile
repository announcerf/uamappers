project := "uamappers"
dev_env := "infra/env/dev/app.env"
dev_compose := "infra/compose/dev/compose.yml"

[doc("Show available commands")]
help:
	just --list --unsorted

[doc("Local preflight (env files, docker daemon, required tools)")]
preflight:
	@test -f {{dev_env}} || (echo "Missing {{dev_env}}. Copy infra/env/dev/app.env.example first." && exit 1)
	@command -v docker >/dev/null || (echo "docker not found" && exit 1)
	@docker compose version >/dev/null 2>&1 || (echo "docker compose plugin not available" && exit 1)
	@docker info >/dev/null 2>&1 || (echo "docker daemon is not running" && exit 1)
	@command -v cargo >/dev/null || (echo "cargo not found" && exit 1)
	@command -v bun >/dev/null || (echo "bun not found" && exit 1)
	@echo "Preflight OK"

[doc("Run clippy for the whole workspace (fail on warnings)")]
clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

[doc("Run all tests in the workspace")]
test:
	cargo test --workspace --all-features --all-targets
	bun run web:test

[doc("Run tests only for the API crate")]
test-api:
	cargo test -p uamappers-api --all-features --all-targets

[doc("Run tests only for the web workspace")]
test-web:
	bun run web:test

[doc("Run tests only for the worker crate")]
test-worker:
	cargo test -p uamappers-worker --all-features --all-targets

[doc("Fast compile check for the workspace")]
check:
	cargo check --workspace --all-features --all-targets
	bun run lint
	bun run typecheck
	bun run web:test
	bun run format:check

[doc("Install Bun workspace dependencies")]
install:
	bun install --frozen-lockfile

[doc("Run frontend lint checks (Astro + Vue)")]
lint:
	bun run lint

[doc("Run frontend type checks")]
typecheck:
	bun run typecheck

[doc("Run the API locally (no docker)")]
api-dev:
	set -a; . {{dev_env}}; set +a; cargo run -p uamappers-api

[doc("Run the web frontend locally with Bun (no docker)")]
web-dev:
	bun run --cwd apps/web dev

[doc("Start local dev stack via Docker (postgres + api + web)")]
up:
	docker compose -f {{dev_compose}} -p {{project}} up -d

[doc("Stop local dev stack")]
down:
	docker compose -f {{dev_compose}} -p {{project}} down

[doc("Rebuild local dev stack without cache")]
rebuild:
	docker compose -f {{dev_compose}} -p {{project}} down
	docker compose -f {{dev_compose}} -p {{project}} --profile worker --profile migrator build --no-cache
	docker compose -f {{dev_compose}} -p {{project}} up -d

[doc("Delete all local docker resources for this project (DB data included)")]
nuke:
	docker compose -f {{dev_compose}} -p {{project}} down --remove-orphans --volumes --rmi local || true

[doc("Apply SeaORM migrations to local docker Postgres")]
migrate:
	@set -a; . {{dev_env}}; set +a; \
	i=0; \
	until docker compose -f {{dev_compose}} -p {{project}} exec -T postgres pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" >/dev/null 2>&1; do \
		i=$((i+1)); \
		if [ $i -ge 30 ]; then \
			echo "postgres is not ready"; \
			exit 1; \
		fi; \
		sleep 1; \
	done
	docker compose -f {{dev_compose}} -p {{project}} --profile migrator build migrator
	DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB" \
	docker compose -f {{dev_compose}} -p {{project}} --profile migrator run --rm -e DATABASE_URL migrator up

[doc("Open a psql shell inside the postgres container")]
psql:
	@set -a; . {{dev_env}}; set +a; docker compose -f {{dev_compose}} -p {{project}} exec postgres psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"

[doc("Start the worker container (manual profile)")]
worker-run: up migrate
	docker compose -f {{dev_compose}} -p {{project}} --profile worker rm -f worker || true
	docker compose -f {{dev_compose}} -p {{project}} --profile worker up -d worker

[doc("Stop the worker container if it's running")]
worker-stop:
	docker compose -f {{dev_compose}} -p {{project}} --profile worker stop worker || true
	docker compose -f {{dev_compose}} -p {{project}} --profile worker rm -f worker || true

[doc("Create a DB dump into backups. Modes: full | schema | data")]
backup mode="full":
	./scripts/db_backup.sh {{project}} {{mode}}

[doc("List available DB backups")]
backups:
	@if [ ! -d backups ]; then \
		echo "No backups/ directory"; \
	else \
		ls -1 backups | sort; \
	fi

[doc("Restore a dump file. Modes: inplace | clean")]
restore dump mode="inplace":
	./scripts/db_restore.sh {{project}} {{dump}} {{mode}}
