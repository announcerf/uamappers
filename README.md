# uamappers

esu nyan?

## Backend

- **API**: Rust HTTP API with OpenAPI/Swagger, backed by PostgreSQL.
- **Worker**: Rust crawler that discovers Ukrainian mappers and refreshes cached data.

### Local dev (Docker)

1. Copy `.env.example` to `.env` and fill in values (minimum: `OSU_CLIENT_ID`, `OSU_CLIENT_SECRET`, `POSTGRES_PASSWORD`).
2. Start Postgres + API:
   - `just up`
   - API: `http://localhost:8080`
   - Swagger UI: `http://localhost:8080/swagger-ui`
   - OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`
3. Apply DB schema:
   - `just migrate`
4. Run the worker manually (not auto-started by default):
   - `just worker-run`
   - `just worker-logs`
   - `just worker-stop`

### Local dev (No Docker)

Prereqs:
- PostgreSQL running locally
- `psql` available on PATH

1. Copy `.env.example` to `.env` and set:
   - `POSTGRES_HOST=localhost`
   - `POSTGRES_PORT=5432`
   - `POSTGRES_USER=...`
   - `POSTGRES_PASSWORD=...`
   - `POSTGRES_DB=uamappers`
2. Apply DB schema:
   - `just migrate-local`
3. Run API:
   - `just api-run`
   - Swagger UI: `http://localhost:8080/swagger-ui`
4. Run worker (foreground):
   - `just worker-run-local`

## License

Apache License 2.0. See `LICENSE` and `NOTICE`.
