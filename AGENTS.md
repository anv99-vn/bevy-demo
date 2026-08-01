# AGENTS.md

## Project Overview

Bevy (Rust) 3D demo application: a rotating cube with an orbit camera. Bevy 0.15, Rust edition 2021.

## Structure

- `src/main.rs` — entry point; builds the `App` with `DefaultPlugins` and registers `Startup`/`Update` systems from the modules below.
- `src/camera.rs` — `OrbitCamera` component + `setup`/`update` systems (drag to orbit, scroll to zoom).
- `src/cube.rs` — `Rotator` component + `setup`/`rotate` systems; spawns the spinning cube, a ground plane, and a point light.
- `assets/` — unused (no assets loaded).

## Commands

- Build (debug): `cargo build -j 4`
- Build (release): `cargo build --release -j 4`
- Run: `cargo run -j 4`
- Clean: `cargo clean`
- Lint: `cargo clippy`
- Check: `cargo check`
- Format: `cargo fmt`

## Conventions

- Modules use `pub fn` systems and `pub struct` components; private fields on components.
- Add new systems by registering them in `main.rs` (`Startup` for setup, `Update` for per-frame logic).
- No tests currently defined; verify with `cargo check` after changes.
- Never run `cargo build` on the local machine; use `cargo check` for verification instead.
- Avoid `cargo clean` when possible; use it only if a rebuild genuinely requires it, and ask the user before running it.
- Before building or running, run `cargo clippy` and fix all warnings.
- Use only 4 cores for builds: append `-j 4` to build/run commands.
- When creating a git tag, update the Changelog file first, then push the tag with `git push origin <tag>`.
- After pushing a tag, check CI/CD for the tag run and fix any failures before considering the release done.
