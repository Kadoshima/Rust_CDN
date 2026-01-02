# Repository Guidelines

This repository hosts a learning-focused Rust mini CDN (cache-enabled reverse proxy). Keep changes small, readable, and aligned with the incremental phases in `TODO.md`.

## Project Structure & Module Organization

- `README.md` describes scope, architecture, and planned modules.
- `TODO.md` tracks phased milestones.
- `mini-cdn-rs/` is the Rust crate.
  - `mini-cdn-rs/Cargo.toml` defines dependencies.
  - `mini-cdn-rs/src/main.rs` is the current entry point.
- Future modules (e.g., `cache.rs`, `proxy.rs`, `metrics.rs`) should live under `mini-cdn-rs/src/` as the project grows.

## Build, Test, and Development Commands

Run commands from `mini-cdn-rs/` (the crate root):

- `cargo build` builds the binary.
- `cargo run` runs the proxy locally.
- `cargo test` runs tests (none are defined yet).

## Coding Style & Naming Conventions

- Use standard Rust style: 4-space indentation, `snake_case` for functions/modules, `SCREAMING_SNAKE_CASE` for constants, `UpperCamelCase` for types.
- Prefer small, focused modules under `mini-cdn-rs/src/`.
- Formatting/linting: use `cargo fmt` and `cargo clippy` if available; no custom config is defined yet.

## Testing Guidelines

- Use Rust’s built-in test framework (`#[test]`).
- Unit tests can live in `src/` modules; integration tests can go in `mini-cdn-rs/tests/`.
- Name tests descriptively (e.g., `cache_key_includes_query`).

## Commit & Pull Request Guidelines

- Git history currently includes only “Initial commit”; no established convention exists yet.
- Prefer concise, imperative commit messages (e.g., “Add env config loader”).
- PRs should include: a short summary, what was tested (`cargo test` or “not run”), and a link to a related `TODO.md` item when applicable.

## Learning Mode (Teacher-Style Collaboration)

- Don’t make changes without explicit permission; confirm intent before editing.
- Avoid dumping full solutions at once; provide step-by-step guidance and check understanding.
- Ask clarifying questions when requirements are ambiguous, and explain trade-offs briefly.
- When you propose a design direction, don’t just agree; provide critique plus clear pros/cons.
- Add brief real-world best practices as optional context when relevant.
- Prefer small, incremental changes for learnability.
- Offer 1–2 alternative approaches when useful.
- Call out likely edge cases or pitfalls early.
- Treat Single Responsibility as a must; propose other SOLID principles only when they add clear value.

## Configuration & Environment

- Local config is expected via `.env`. Required keys are described in `README.md`/`TODO.md`:
  - `ORIGIN_BASE`, `LISTEN_ADDR`, `CACHE_ENABLED`, `DEFAULT_TTL_SECS`
- Ensure the `.env` file is in the current working directory when running the binary. If you run from `mini-cdn-rs/`, place `.env` there or copy it from the repo root.
- Example:

```env
ORIGIN_BASE=http://localhost:8080
LISTEN_ADDR=127.0.0.1:3000
CACHE_ENABLED=true
DEFAULT_TTL_SECS=60
```
