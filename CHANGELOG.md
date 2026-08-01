# Changelog

## v0.5.1 - 2026-08-02

- Fix Android APK signing in CI build.
- Add login scene with text input and game state transitions.
- Replace custom login UI with bevy_egui.
- Free-orbit camera with spherical coordinates for full 360 rotation.
- Extract settings button into its own module.
- Add key input logging.

## v0.5.0 - 2026-08-01

- Rename release workflow and add platform names to assets.
- Skip windows-build job on push, require manual run.
- Use all cores in CI and ignore Windows executables.
- Add opencode agent and command ask configs.
- Build Windows natively and rename lib to bevy_demo_app.
- Move sensitivity HUD behind a Settings button toggle.
- Add Windows-only build workflow.
- Block camera orbit while dragging sensitivity slider.

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
