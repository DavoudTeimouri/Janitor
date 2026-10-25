# Contributing to Janitor

## How to contribute

1. Fork `DavoudTeimouri/Janitor`.
2. Create a branch from `main`: `git checkout -b feat/your-feature`.
3. Make changes, run `cargo test` and `cargo build --release`.
4. Commit with conventional message: `feat:`, `fix:`, `docs:`, `chore:`.
5. Open a Pull Request against `main`.

## Code style

- Stdlib-first. No new external crates without discussion.
- Shortest working diff. No boilerplate "for later".
- Deliberate simplifications marked with `ponytail:` comment naming ceiling + upgrade path.
- Non-trivial logic needs one assert-based self-check test.

## Commit convention

```
feat: add drill subcommand
fix: skip protected paths in scan
docs: update README usage examples
```

## Release process

Maintainer cuts RC tags (`v0.1.0-rc.N`) on `main`. Stable tag after user sign-off. Each release needs:
- CHANGELOG entry (date, features, enhancements, changes)
- Git tag
- GitHub Release with RPM + DEB + tarball
