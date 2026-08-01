# Changelog

## v0.4.1 - 2026-08-01

- Fix cargo-xwin SDK cache misses on tag builds by warming the cache on main.

## v0.4.0 - 2026-08-01

- Never run `cargo build` on the local machine; use `cargo check` for verification instead.

## v0.3.0 - 2026-08-01

- Add on-screen mouse sensitivity HUD with finer adjustment.

## v0.2.0 - 2026-08-01

- Add configurable mouse sensitivity.
- Build Android APK in the release workflow.
- Cross-compile Windows build with cargo-xwin and check on Windows in CI.
- Check on a single OS in CI.
- Add watch mode to the fix-ci skill.
- Add tag push and CI/CD check rules to AGENTS.md.
- Install llvm-tools so cargo-xwin can find llvm-lib.

## v0.1.1 - 2026-08-01

- Split CI into check-on-main and build-on-tag workflows.
- Add concurrency control to cancel in-progress runs.
- Fix clippy build error and formatting.

## v0.1.0 - 2026-08-01

- Initial release: Bevy 0.15 rotating cube with orbit camera.
- Add GitHub Actions CI that builds Linux and Windows binaries and publishes them as a GitHub Release on tag.
