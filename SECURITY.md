# Security policy

## Supported versions

| Version | Supported |
|---------|-----------|
| latest stable `vX.Y.Z` | ✅ |
| pre-release `vX.Y.Z-rc.N` | ⚠️ testing only |
| older | ❌ |

## Reporting a vulnerability

Email `security@davoudteimouri.dev` or open a private GitHub advisory. Do not post vulnerability details in public issues.

## What we consider safe

- Janitor never calls `rm` by default.
- Deletion goes to user Trash via `trash-put` or `gio trash`.
- Protected system paths (`/boot`, `/etc`, `/usr`, `/var/lib/*`, Docker, `/proc`, `/sys`, `/dev`, `~/.ssh`) are excluded by `SAFETY.md`.
- Permanent delete requires `--yes`.

## What we do NOT consider a vulnerability

- User running `janitor clean / --yes` and deleting their own files (user confirmed).
- Missing trash backend on a minimal host (error message guides install of `trash-cli` or `gvfs`).
