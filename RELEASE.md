# Release process

Janitor follows a pre-release-first model. Every version ships as `vX.Y.Z-rc.N` first,
is tested, and only goes stable after user sign-off.

## Cutting an RC

1. Update `CHANGELOG.md` with a new `## [vX.Y.Z-rc.N] — YYYY-MM-DD` section.
2. Bump version in `Cargo.toml`, `packaging/janitor.spec`, `packaging/debian/control`.
3. Commit: `git commit -m "chore: prepare vX.Y.Z-rc.N"`.
4. Tag: `git tag vX.Y.Z-rc.N` and push: `git push origin vX.Y.Z-rc.N`.
5. Build RPM + DEB + tarball from the tag.
6. Create GitHub Release (pre-release) with assets and the changelog section as body.
7. Test on non-critical systems.
8. After sign-off, retag as stable `vX.Y.Z` and promote the GitHub Release.

## After each release (local)

Run `scripts/release-cleanup.sh` to remove `target/`, test scratch dirs, and other
build traces from the local checkout.

## Version locations

- `Cargo.toml` — `version = "X.Y.Z"`
- `packaging/janitor.spec` — `Version: X.Y.Z`
- `packaging/debian/control` — `Version: X.Y.Z`
