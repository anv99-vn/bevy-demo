---
name: fix-ci
description: Use when a GitHub Actions workflow runs on push to the main branch and a run fails, or when asked to create/set up a CI cron job that runs on push to main, or when asked to continuously watch/monitor CI and fix each failing run forever. Reads the failing run's error messages and fixes the underlying code or workflow until the run passes.
---

# Fix CI

Ensures the repository's CI passes on every push to `main`. Used when a
workflow run on `main` fails, or when setting up a new workflow that must run
on push to `main`.

## Setup a workflow that runs on push to main

If the workflow does not exist yet, create `.github/workflows/ci.yml` that
triggers on push to `main` (plus optional `pull_request` and
`workflow_dispatch`):

```yaml
name: check

on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:

concurrency:
  group: ci
  cancel-in-progress: true

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo check -j 4
```

Adapt steps to the project's build/lint/check commands. See the `Commands`
section of `AGENTS.md` for the exact commands to run.

## On a failed run: read the error, then fix

1. **Find the failing run.**
   ```
   gh run list --branch main --limit 5
   gh run view <run-id>
   ```
   Identify the failed job and step.

2. **Read the error messages.**
   ```
   gh run view <run-id> --log-failed
   ```
   Focus on the first real error: the compiler/clippy/test message, not the
   surrounding noise. Distinguish error types:
   - **Build/type errors** — `cargo check`/`cargo build` failures. Fix the
     Rust code, then reproduce locally with `cargo check -j 4` and
     `cargo clippy --all-targets -- -D warnings`.
   - **Format errors** — `cargo fmt --check` failures. Run `cargo fmt`.
   - **Workflow/action errors** — missing files, wrong paths, invalid YAML,
     failed actions. Fix the `.github/workflows/*.yml` config.
   - **Dependency errors** — add the missing Linux dependency to the `apt-get
     install` step, or fix `Cargo.toml`.

3. **Fix the root cause**, not the symptom. Do not disable checks or add
   `#[allow]` just to make the run green.

4. **Verify locally before pushing.** Run the same commands the workflow runs
   (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo check -j 4`). Fix any warnings — clippy warnings are hard failures.

5. **Commit and push to `main`.** The workflow runs on push; after pushing,
   confirm the new run:
   ```
   gh run list --branch main --limit 3
   gh run watch <run-id>
   ```

6. If it still fails, repeat from step 1 — read the new error, fix, push.
   Never push speculative fixes without reproducing the error locally.

## Watch mode: watch forever and fix each failing run

When asked to "watch", "run forever", "keep monitoring", or "fix CI and
continue" the CI, loop until the user stops you:

1. **Sync and note the latest commit.**
   ```
   git fetch origin main
   git rev-parse origin/main
   ```
   Record the current `origin/main` head SHA. A run may not exist yet for a
   very fresh push.

2. **Wait for the run for that commit.**
   ```
   gh run list --branch main --limit 5
   ```
   Find the `check` run whose `HEAD SHA` matches the commit. If it is not
   there yet (run still starting), poll `gh run list` every ~15 seconds until
   it appears.

3. **Watch it to completion.**
   ```
   gh run watch <run-id> --exit-status
   ```

4. **If it failed:** fix the root cause (see "On a failed run: read the
   error, then fix"), verify locally, commit and push to `main`. The push
   creates a new head and triggers a new run — go back to step 1 and repeat.
   Never push speculative fixes without reproducing the error locally.

5. **If it passed:** the commit is green, but keep going. Poll for the next
   commit on `main`:
   ```
   git fetch origin main
   git rev-parse origin/main
   ```
   Check periodically (e.g. every 30 seconds) until the head changes, then go
   back to step 2 and watch that run.

Run this loop indefinitely until the user stops you or changes the task.
Summarize each failure you fix as you go; do not re-report commits that
already passed.

## Guidelines

- Commands must use only 4 cores: append `-j 4` to `cargo` build/run/check.
- Run `cargo clippy` before building/running (see `AGENTS.md`).
- Never use `cargo clean` without asking the user first.
- Do not commit secrets; never edit workflow files to embed credentials.
