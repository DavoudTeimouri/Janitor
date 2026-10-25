# Changelog

All notable changes to Janitor are documented in this file.

## [Unreleased]

### New features
- 

### Enhancements
- 

### Changes
- 

### Bug fixes
- 

### Security
- 

## [v0.1.0-rc.1] — 2026-01-25

### New features
- Directory-size scan: reports every directory containing candidate files, sorted by total size
- Drill-down: `janitor drill <dir>` lists files inside a directory sorted by size
- Safe deletion: `trash-put` → `gio trash` → `rm --yes` fallback, never raw `rm` by default
- Confirmation prompts per candidate; `--yes` skips prompts
- Broken symlink detection via `orphan` subcommand
- Size threshold via `--min-size N` (default 100 MB)
- Stdlib-only: zero external crates

### Enhancements
- `scan` output shows DIRECTORY / SIZE / FILES / TOP TYPE
- `clean` collects candidates across all directories and deletes interactively

### Changes
- Replaced flat file list with per-directory aggregation
- Renamed `Entry` → `FileEntry`, added `DirEntry`
- Cleanup backend auto-detects `trash-put` then `gio trash`

### Bug fixes
- None (initial RC)

### Security
- `SAFETY.md`: protected paths knowledge base (boot, package manager, Docker, /proc, /sys, /dev, ~/.ssh, etc.)
