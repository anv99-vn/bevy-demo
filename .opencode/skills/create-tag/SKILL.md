---
name: create-tag
description: Use when the user asks to create a new release tag, prepare a new version, or cut a release. Updates CHANGELOG.md with a new version entry, creates a git tag, and pushes it. Optionally monitors the CI/CD release workflow run.
---

# create-tag

Creates a new release tag for the Bevy demo project. Updates `CHANGELOG.md`,
creates a git tag, and pushes it to origin. The CI/CD workflow will then
build and publish the release.

## Steps

1. **Determine the new version.**

   Read the latest tag and ask the user for the new version if not provided:
   ```
   git tag --sort=-v:refname | Select-Object -First 1
   ```
   The new version should follow semver (e.g. `v0.5.0`).

2. **Get today's date.**
   Use `Get-Date -Format "yyyy-MM-dd"` for the changelog date.

3. **Update CHANGELOG.md.**

   Insert a new section at the top of `CHANGELOG.md` (after the `# Changelog`
   heading) following this format:

   ```markdown
   ## v<VERSION> - <YYYY-MM-DD>

   - <change description>
   ```

   Ask the user what changes to include, or infer from recent commits:
   ```
   git log <previous_tag>..HEAD --oneline
   ```

   Example after editing:
   ```markdown
   # Changelog

   ## v0.5.0 - 2026-08-01

   - Add new feature X.
   - Fix bug Y.

   ## v0.4.1 - 2026-08-01
   ...
   ```

4. **Commit the changelog update.**
   ```
   git add CHANGELOG.md
   git commit -m "Update CHANGELOG.md for v<VERSION>"
   ```

5. **Create and push the tag.**
   ```
   git tag v<VERSION>
   git push origin v<VERSION>
   ```

6. **Monitor the release workflow.**

   After pushing the tag, watch for the release CI run:
   ```
   gh run list --branch v<VERSION> --limit 3
   gh run watch <run-id>
   ```

   If it fails, fetch logs and report:
   ```
   gh run view <run-id> --log-failed
   ```

   Do NOT attempt to fix failures automatically — report the error and let
   the user decide.

## Notes

- Never skip the changelog update — it must always reflect the new version.
- Never force-push tags or overwrite existing tags.
- If the tag already exists, abort and inform the user.
- Use only 4 cores for any cargo commands: append `-j 4`.
