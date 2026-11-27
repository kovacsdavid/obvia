# Repository Guidance

## Project Layout

- `backend/` is one Rust package. Shared infrastructure lives in `src/common/`, system-wide functionality in `src/manager/`, and tenant business modules in `src/tenant/`.
- `frontend/` is a React/TypeScript application built with Vite. Feature modules live under `src/components/modules/`; shared hooks, services, and Redux state live under `src/hooks/`, `src/services/`, and `src/store/`.
- Backend migrations are split between `backend/migrations/main/` and `backend/migrations/tenant/`.

## Working Conventions

- For a feature change, inspect its owning module and a nearby equivalent, including tests. Adapt the current pattern rather than copying it blindly, and keep changes within the requested scope.
- Preserve the manager-versus-tenant boundary in backend changes and the module-versus-shared boundary in frontend changes.
- Add or update focused tests for behavior changes, following the local test style. Run the narrowest relevant check first, then broader checks when the change warrants them.
- Treat current source and package manifests as authoritative when documentation disagrees. In particular, backend binaries are `obvia_server` and `obvia_cli`; some older docs still refer to `main.rs` or `obvia_backend`.
- Backend runtime and database workflows require configuration from `backend/config/`; check the example config and Compose setup before assuming local services are available.

## Validation Commands

Run from the repository root:

```sh
cargo fmt --manifest-path backend/Cargo.toml -- --check
cargo test --manifest-path backend/Cargo.toml
cargo clippy --manifest-path backend/Cargo.toml --all-targets
cargo build --manifest-path backend/Cargo.toml

npm --prefix frontend test -- --run
npm --prefix frontend run lint
npm --prefix frontend run fmt-check
npm --prefix frontend run build
```

## Further Reading

- [README.md](README.md) for setup and project overview.
- [GUIDELINES.md](GUIDELINES.md) for development conventions; verify commands and paths against current manifests and source.
- [CONTRIBUTING.md](CONTRIBUTING.md) for contribution, Conventional Commit, and DCO requirements.