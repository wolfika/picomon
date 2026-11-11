# Contributing

## Setup

Requirements: stable Rust and [Bun](https://bun.sh). Docker is needed only to build or test the ARM targets locally.

```sh
make dev     # backend on :7340, Vite with hot reload on :5173
make test    # Rust tests and Svelte type check
make lint    # rustfmt, clippy and Biome, as CI runs them
make fmt     # apply rustfmt and Biome fixes
make build   # release binary at target/release/picomon
```

The frontend in `web/` is built into `web/dist` and embedded in the binary at compile time, so `make build` builds it first. `make dev` serves it from Vite instead.

Some definitions exist on both sides and must change together:

- JSON shapes in `src/collect/mod.rs` and `web/src/lib/types.ts`
- Theme presets in `src/config.rs`, `web/src/lib/theme.ts` and `web/src/app.css`

## Commits

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org). Keep each commit to one coherent change.

The commit type decides what a change does to the next release:

| Type                                                        | Effect while below 1.0           |
| ----------------------------------------------------------- | -------------------------------- |
| `feat`                                                      | Minor version bump               |
| `fix`, `perf`                                               | Patch version bump               |
| `feat!`, `fix!` or a `BREAKING CHANGE:` footer              | Minor version bump               |
| `docs`, `style`, `refactor`, `test`, `build`, `ci`, `chore` | No release, not in the changelog |

From 1.0 on, breaking changes bump the major version.

## Pull requests

CI (`.github/workflows/ci.yml`) runs on every pull request:

- `rust`: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
- `web`: `biome ci`, `svelte-check`, `vite build`
- `test-armv7`: the Rust tests for the 32-bit armv7 release target, run under QEMU with [cross](https://github.com/cross-rs/cross)

`make lint` and `make test` cover the first two locally.

## Releases

Releases are automated with [release-please](https://github.com/googleapis/release-please) in `.github/workflows/release.yml`.

1. Every push to `main` runs the CI checks. If they pass, release-please opens or updates a release PR titled `chore(main): release X.Y.Z`. It contains the version bump in `Cargo.toml` and `.release-please-manifest.json`, and the new `CHANGELOG.md` entries generated from the commits since the last release.
2. The release PR collects changes until it is merged. Merging it is the decision to release.
3. Merging tags `vX.Y.Z` and creates the GitHub release with the changelog as its notes. The same workflow run then, for each target:
   - builds a static musl binary with cross (`x86_64`, `aarch64`, `armv7`),
   - runs `.github/scripts/smoke-test.sh`, which checks the version, the embedded dashboard and a live sample,
   - uploads it as `picomon-linux-<arch>`.

   A final job uploads `SHA256SUMS`. Asset names carry no version, so `releases/latest/download/<asset>` always points at the newest release.

To set a version explicitly, for example for 1.0, add a `Release-As: 1.0.0` footer to a commit message.

GitHub Actions needs permission to create pull requests: Settings → Actions → General → "Allow GitHub Actions to create and approve pull requests".

### When a release build fails

The tag and release exist before the binaries are built, so a failed build leaves a release without assets.

- For a transient failure, re-run the failed jobs (`gh run rerun <run-id> --failed`). Uploads overwrite existing assets.
- For a failure in the code, the build checks out the tag, so a re-run reproduces it. Fix it on `main` and release the fix as the next patch version.

To check a release build locally, build a target with cross and run the smoke test against it:

```sh
cross build --release --target aarch64-unknown-linux-musl
.github/scripts/smoke-test.sh target/aarch64-unknown-linux-musl/release/picomon
```

ARM binaries need QEMU registered with binfmt_misc on the host, or run the test inside an emulated container.
