# AGENTS.md

Guidelines for AI agents and humans contributing to this codebase.

## Engineering conventions

- Commit after each coherent unit of work. Never bundle unrelated changes.
- Never push without explicit consent.

## Project notes

- `make build` builds the frontend (`web/`, Bun + Vite) before the Rust binary, because `web/dist` is embedded at compile time.
- `make test` runs `cargo test` and `svelte-check`.
- JSON shapes in `src/collect/mod.rs` are mirrored in `web/src/lib/types.ts`. Theme presets are listed in `src/config.rs`, `web/src/lib/theme.ts` and `web/src/app.css`. Change them together.
