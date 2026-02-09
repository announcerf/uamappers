set dotenv-load := true

project := "uamappers"

# Aliases
alias backups := backup-list
alias docker-clean := nuke

# Run clippy for the whole workspace (fail on warnings).
clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all tests in the workspace.
test:
	cargo test --workspace --all-features --all-targets

# Fast compile check for the workspace.
check:
	cargo check --workspace --all-features --all-targets

# API-only cargo check.
api-check:
	(cd apps/api && cargo check --all-features --all-targets)

# API-only clippy (fail on warnings).
api-clippy:
	(cd apps/api && cargo clippy --all-targets --all-features -- -D warnings)

# API-only tests.
api-test:
	(cd apps/api && cargo test --all-features --all-targets)

# Worker-only cargo check.
worker-check:
	(cd apps/worker && cargo check --all-features --all-targets)

# Worker-only clippy (fail on warnings).
worker-clippy:
	(cd apps/worker && cargo clippy --all-targets --all-features -- -D warnings)

# Worker-only tests.
worker-test:
	(cd apps/worker && cargo test --all-features --all-targets)

# Start Postgres + API via docker compose (worker is not started by default).
up:
	docker compose -p {{project}} up -d

# Stop containers started by `just up`.
down:
	docker compose -p {{project}} down

# Rebuild images without cache.
rebuild:
	docker compose -p {{project}} build --no-cache

# Restart docker services (down -> rebuild -> up).
reup:
	docker compose -p {{project}} down
	docker compose -p {{project}} build --no-cache
	docker compose -p {{project}} up -d

# Delete all docker resources for this project (DB data included).
nuke:
	./scripts/docker_clean.sh {{project}}

# Apply SQL migrations from `apps/api/migrations` to the docker Postgres.
migrate:
	@i=0; \
	until docker compose -p {{project}} exec -T postgres pg_isready -U "$$POSTGRES_USER" -d "$$POSTGRES_DB" >/dev/null 2>&1; do \
		i=$$((i+1)); \
		if [ $$i -ge 30 ]; then \
			echo "postgres is not ready"; \
			exit 1; \
		fi; \
		sleep 1; \
	done
	@find apps/api/migrations -maxdepth 1 -name '*.sql' -print | sort | while read -r f; do \
		echo "Applying $$f"; \
		docker compose -p {{project}} exec -T postgres psql -v ON_ERROR_STOP=1 -U "$$POSTGRES_USER" -d "$$POSTGRES_DB" < "$$f"; \
	done

# Open a psql shell inside the postgres container.
psql:
	docker compose -p {{project}} exec postgres psql -U "$$POSTGRES_USER" -d "$$POSTGRES_DB"

# Start the worker container (manual profile).
worker-run: up migrate
	docker compose -p {{project}} --profile worker up -d worker

# Stop the worker container if it's running.
worker-stop:
	docker compose -p {{project}} --profile worker stop worker || true
	docker compose -p {{project}} --profile worker rm -f worker || true

# Tail worker logs.
worker-logs:
	docker compose -p {{project}} --profile worker logs -f worker

# Create a full DB dump into `./backups`.
backup:
	./scripts/db_backup.sh {{project}} full

# Create a schema-only dump into `./backups`.
backup-schema:
	./scripts/db_backup.sh {{project}} schema

# Create a data-only dump into `./backups`.
backup-data:
	./scripts/db_backup.sh {{project}} data

# List available DB backups.
backup-list:
	./scripts/db_backup_list.sh

# Restore a dump file into the DB.
restore dump:
	./scripts/db_restore.sh {{project}} {{dump}} inplace

# Restore a dump file after dropping existing objects (destructive).
restore-clean dump:
	./scripts/db_restore.sh {{project}} {{dump}} clean
