# LOGIC.md — Janitor application logic, features, and internals

## Overview

Janitor is a stdlib-first Rust CLI for Linux disk cleanup. It scans a target root, reports space usage grouped by directory, lets the user inspect large files, and removes candidates via safe trash commands (or permanent delete behind `--yes`).

## Core loop

```
scan(root) -> entries grouped by directory
  -> report: directory size, file count, top files
  -> user selects directory
  -> drill-down: list files inside, sorted by size
  -> user selects file(s)
  -> confirm: trash-put / gio trash / rm --yes
```

## Modules

- `src/main.rs` — CLI entry, subcommand dispatch (`scan`, `clean`, `orphan`, `report`, `web` future).
- `src/scan.rs` — filesystem walk, classification, directory-size aggregation, progress reporting.
- `src/clean.rs` — deletion via `trash-put` / `gio trash` / `rm`, confirmation prompts.
- `src/orphan.rs` — broken symlink detection.
- `src/report.rs` — human-readable output formatting.
- `src/tui.rs` (planned RC2) — directory tree explorer, select-to-scan.

## Classification (src/scan.rs)

| Kind | Match rule | Protected? |
|------|-----------|------------|
| temp | `.tmp`, `.temp` | No |
| dump | `.dmp`, `.dump`, `.crash`, `/var/crash/` | No |
| cache | `cache` in name or `/.cache/` in path | User cache — confirm |
| log | `.log`, `.log.old`, `/var/log/` | Conditional — archive first |
| large | size >= `--min-size` | No — user decides |
| orphan-link | broken symlink | No |

## Directory-size reporting

Scan walks recursively and accumulates size per directory. Report shows:
- Directory path, total size, file count, top 3 largest files inside.
- User selects a directory → drill-down lists its files sorted by size.

## Cleanup safety

- Default interactive: print candidate + size, ask `[y/N]`.
- `--yes`: skip prompts.
- Delete backend: `trash-put` (preferred), `gio trash` (fallback), `rm -rf` only with `--yes` and non-protected path.
- Every candidate checked against `SAFETY.md` rules before offering.

## Release model

- Pre-release `vX.Y.Z-rc.N` on `main` or `pre-release/vX.Y.Z`.
- Stable `vX.Y.Z` only after user sign-off on RC testing.
- Each release: CHANGELOG entry (date, features, enhancements, changes), tag, GitHub Release with assets.

## Future (post-stable)

- `--web` HTTP dashboard behind feature gate.
- Orphan systemd unit detection.
- Docker/Podman native prune integration (`docker system prune`).
- Scheduled cron cleanup (opt-in).
