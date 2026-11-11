# AGENTS.md

Guidelines for AI agents and humans contributing to this codebase.

## Engineering conventions

- Commit after each coherent unit of work. Never bundle unrelated changes.
- Never push without explicit consent.

## Project notes

- `make build` builds the frontend (`web/`, Bun + Vite) before the Rust binary, because `web/dist` is embedded at compile time.
- `make test` runs `cargo test` and `svelte-check`. `make lint` runs rustfmt, clippy and Biome as CI does; `make fmt` fixes formatting.
- JSON shapes in `src/collect/mod.rs` are mirrored in `web/src/lib/types.ts`. Theme presets are listed in `src/config.rs`, `web/src/lib/theme.ts` and `web/src/app.css`. Change them together.
- Releases are cut by release-please (`.github/workflows/release.yml`) from conventional commit messages, so commit types decide the version bump and changelog. Merging its release PR tags `vX.Y.Z` and attaches static binaries built with `cross`.
