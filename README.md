# uamappers

esu nyan?

## Backend

- **API**: Rust HTTP API with OpenAPI/Swagger, backed by PostgreSQL.
- **Worker**: Rust crawler that discovers Ukrainian mappers and refreshes cached data.

### Local dev (docker)

1. Copy `.env.example` to `.env` and fill in the values (at minimum: `OSU_CLIENT_ID`, `OSU_CLIENT_SECRET`, `POSTGRES_PASSWORD`).
2. Start services:
   - `just up`
3. Apply DB schema:
   - `just migrate`
4. Run the worker manually (does not auto-start in docker):
   - `just worker`
   - `just worker-logs`
   - `just worker-stop`

## License

Apache License 2.0. See `LICENSE` and `NOTICE`.
