# X-BRIDGE-RUST

Professional Axum API starter focused on modular structure, dynamic router registration, offline Swagger docs, and server-side templates.

## Highlights

- **Automatic API version/module prefixing**: every API module is mounted as `/api/v1/<module_name>`.
- **Offline Swagger UI** at `/docs` using vendored assets (no external runtime links).
- **OpenAPI filtering**: only paths starting with `/api` are kept in Swagger.
- **Static assets** served from `static/`.
- **Template system** with Jinja2-style syntax using MiniJinja:
  - `templates/base.html`
  - `templates/includes/header.html`
  - `templates/includes/footer.html`

## Project structure

```text
.
├── src
│   ├── api
│   │   ├── health.rs
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── web
│   │   ├── home.rs
│   │   ├── mod.rs
│   │   └── templates.rs
│   ├── app.rs
│   ├── docs.rs
│   ├── lib.rs
│   └── main.rs
├── static
│   └── css
│       └── app.css
└── templates
    ├── base.html
    ├── home.html
    └── includes
        ├── footer.html
        └── header.html
```

## How dynamic router registration works

- API modules are represented by `ApiModule` (`src/api/mod.rs`).
- Each module exposes `module() -> ApiModule` with only its local router.
- `register_modules(...)` automatically nests each module under:

```text
/api/v1/<module_name>
```

To add a new module:

1. Create `src/api/<new_module>.rs`
2. Return `ApiModule::new("<new_module>", Router::new()...)`
3. Add it once to `all_modules()`

No manual prefix wiring is required in the app setup.

## Run locally

```bash
cargo run
```

Server default: `127.0.0.1:3000`

Useful endpoints:

- `GET /` Home page (template rendered)
- `GET /docs` Swagger UI
- `GET /api-docs/openapi.json` OpenAPI JSON
- `GET /api/v1/health/ping`
- `GET /api/v1/users`

## Test

```bash
cargo test
```

Included focused tests:

- API module prefix behavior (`/api/v1/<module>`)
- Swagger path filtering (only `/api...`)
