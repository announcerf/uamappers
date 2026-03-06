# UAMappers

- `apps/api` — Rust HTTP API
- `apps/web` — Astro + Vue + TypeScript frontend
- `apps/worker` — background crawler/enrichment worker

## TL;DR

```bash
cp infra/env/dev/app.env.example infra/env/dev/app.env
just preflight
just install
just up
just migrate
```

## Local Development

Docker-first:

```bash
just up
just migrate
just down
```

Local endpoints:

- API: `http://127.0.0.1:8080`
- Web: `http://127.0.0.1:3000`

Optional non-Docker runs:

```bash
just api-dev
just web-dev
```

## Worker

- Local manual run: `just worker-run`
- Local stop: `just worker-stop`
- Production schedule: weekly (Monday) via CI workflow

## Commands

```bash
just help
```

## License

Apache License 2.0. See `LICENSE` and `NOTICE`.
