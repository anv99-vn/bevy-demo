---
name: trigger-build
description: Use when the user asks to trigger, start, kick off, or run the latest Windows build for this Bevy demo (e.g. "trigger build", "run windows build", "start the windows build"). Triggers the `windows-build` GitHub Actions workflow via `gh` and monitors the resulting run.
---

# trigger-build

Trigger the project's `windows-build` GitHub Actions workflow (defined in
`.github/workflows/`, `workflow_dispatch` trigger, no inputs) and report the
result. It builds a release binary for `x86_64-pc-windows-msvc` on
`windows-latest` and uploads `bevy-demo.exe` as an artifact.

## Steps

1. Trigger the workflow:

   ```powershell
   gh workflow run windows-build
   ```

   If `gh` reports that a branch is required, run:

   ```powershell
   gh workflow run windows-build --ref main
   ```

2. Confirm the run was queued. Capture the new run id by listing recent runs of
   the workflow (sorted newest first) and reading the topmost id:

   ```powershell
   gh run list --workflow windows-build --limit 1
   ```

   The first column of the output is the numeric run id.

3. Watch the run until it finishes, then report its conclusion:

   ```powershell
   gh run watch <run-id>
   gh run view <run-id>
   ```

   Do NOT block indefinitely; `gh run watch` streams status. If it fails, fetch
   the logs with `gh run view <run-id> --log-failed` and summarize the error for
   the user.

## Notes

- The workflow only actually builds on `workflow_dispatch` (its `if:` guards the
  job). Runs from `push` events show as skipped — that is expected.
- The artifact is named `bevy-demo-windows`; download with
  `gh run download <run-id>` if the user asks for the binary.
- Never commit changes as part of this skill. Only trigger and report the build.