# Linting & checks (API + worker)
clippy:
	(cd apps/api && cargo clippy --all-targets --all-features -- -D warnings)
	(cd apps/worker && cargo clippy --all-targets --all-features -- -D warnings)

test:
	(cd apps/api && cargo test)
	(cd apps/worker && cargo test)

check:
	(cd apps/api && cargo check)
	(cd apps/worker && cargo check)

# Per-app shortcuts
api-clippy:
	(cd apps/api && cargo clippy --all-targets --all-features -- -D warnings)

worker-clippy:
	(cd apps/worker && cargo clippy --all-targets --all-features -- -D warnings)

api-test:
	(cd apps/api && cargo test)

worker-test:
	(cd apps/worker && cargo test)

api-check:
	(cd apps/api && cargo check)

worker-check:
	(cd apps/worker && cargo check)

# Docker
up:
	docker compose -p uamappers up -d

down:
	docker compose -p uamappers down

rebuild:
	docker compose -p uamappers build --no-cache

# DB
migrate:
	@for f in apps/api/migrations/*.sql; do \
		echo "Applying $$f"; \
		docker compose -p uamappers exec -T postgres psql -U uamappers -d uamappers < $$f; \
	done
