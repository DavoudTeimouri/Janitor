# USER.md — Communication & work style

## How the user prefers to communicate

- Terse, caveman-style responses. No articles (a/an/the), no filler (just/really/basically/actually/simply), no pleasantries, no hedging.
- Pattern: `[thing] [action] [reason]`. Then `[next step]`.
- Code blocks, file paths, commands, errors, URLs: keep exact.
- Security warnings, irreversible action confirmations, multi-step sequences: normal prose.
- No self-reference, no announcing the style, no decorative emoji, no narrating tool calls.

## Config fields with known value sets

Show a numbered selector list. Enter = default, number = select, custom = type value.

## Code & engineering preferences

- Stdlib-first, minimal dependencies, shortest working diff.
- Deletion over addition. Boring over clever. No unrequested abstractions.
- Non-trivial logic leaves ONE runnable assert-based self-check.
- Deliberate simplifications marked with `ponytail:` comment naming ceiling + upgrade path.
- Input validation at trust boundaries, error handling that prevents data loss, security: never simplify away.

## Project-specific (ObtainHub / CloudSeed)

- Version bumped in 4 places: Cargo.toml, installer/ObtainHub.nsi VERSION, installer/setup.wxs, CHANGELOG.md.
- GUID {A1B2C3D4-E5F6-7890-ABCD-EF1234567890} same in WiX UpgradeCode and NSIS PRODUCT_GUID.
- Releases publish only ObtainHub.msi + ObtainHub-Setup.exe + SHA256SUMS.
- Release body = CHANGELOG.md `## [vX.Y.Z]` section.
- Checkout shared with another session (author 'hertest'). Always `git log --oneline -3` + `git status` before committing/pushing.

## Release policy

- Every version ships as PRE-RELEASE first (vX.Y.Z-rc.N), tested, stable only after user confirms.

## Sandbox

- `rm -rf` on directories needs terminal approval (desktop app cannot answer) — give user a one-liner.
- `rm -f` on single files OK.
- Disk hogs: scratch cargo targets and `target/debug` (regenerable).
