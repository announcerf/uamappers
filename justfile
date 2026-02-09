set dotenv-load := true

project := "uamappers"

# Run clippy for the whole workspace (fail on warnings).
clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all tests in the workspace.
test:
	cargo test --workspace --all-features --all-targets

# Fast compile check for the workspace.
check:
	cargo check --workspace --all-features --all-targets

# Start Postgres + API via docker compose (worker is not started by default).
up:
	docker compose -p {{project}} up -d

# Stop containers started by `just up`.
down:
	docker compose -p {{project}} down

# Rebuild images without cache.
rebuild:
	docker compose -p {{project}} build --no-cache

# Remove containers/images/volumes for this project (clean slate).
docker-clean:
	./scripts/docker_clean.sh {{project}}

# Apply SQL migrations from `apps/api/migrations` to the docker Postgres.
migrate:
	@find apps/api/migrations -maxdepth 1 -name '*.sql' -print | sort | while read -r f; do \
		echo "Applying $$f"; \
		docker compose -p {{project}} exec -T postgres psql -v ON_ERROR_STOP=1 -U "$$POSTGRES_USER" -d "$$POSTGRES_DB" < "$$f"; \
	done

# Start the worker in docker (manual profile).
worker:
	docker compose -p {{project}} --profile worker up -d worker

# Stop the worker container if it's running.
worker-stop:
	docker compose -p {{project}} --profile worker stop worker || true
	docker compose -p {{project}} --profile worker rm -f worker || true

# Tail worker logs.
worker-logs:
	docker compose -p {{project}} --profile worker logs -f worker

# Create a compressed DB dump in `backups/`.
backup:
	./scripts/db_backup.sh {{project}}

# List available DB backups.
backup-list:
	./scripts/db_backup_list.sh

