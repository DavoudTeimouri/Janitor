# Competitors / Reverse-engineering findings

## Scope

Compared Janitor against BleachBit, ncdu, Baobab, Stacer, Filelight, QDirStat, Storage Sifter. Sources: vendor feature pages, docs, third-party comparisons (2026). No binary reverse engineering performed yet — all findings are from public documentation and feature lists.

## Comparison matrix

| Capability | Janitor | BleachBit | ncdu | Baobab | Stacer | Filelight | QDirStat | Storage Sifter |
|------------|---------|-----------|------|--------|--------|-----------|----------|----------------|
| Interface | CLI (TUI planned RC2) | GTK GUI + CLI | TUI (ncurses) | GTK GUI | Electron GUI | Qt GUI | Qt GUI | GUI (Rust) |
| Works over SSH | ✅ | ⚠️ CLI only | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Zero external crates | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Directory-size-first report | ✅ | ❌ | ✅ (size sorted) | ✅ (visual) | ❌ | ✅ (visual) | ✅ (visual) | ✅ (visual) |
| Drill-down to files | ✅ | ❌ | ✅ | ⚠️ (opens Files) | ❌ | ✅ (zoom) | ✅ | ✅ |
| Safe deletion (Trash) | ✅ (trash-put/gio) | ✅ | ✅ | ✅ (via Files) | ✅ | ✅ | ✅ | ✅ |
| Confirmation per item | ✅ | ❌ (preview batch) | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Broken symlinks | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Candidate classification (temp/dump/cache/log/large) | ✅ | ✅ (cleaner rules) | ❌ | ❌ | ✅ (categories) | ❌ | ❌ | ✅ |
| Safe-to-delete guidance | ⬜ (SAFETY.md) | ⚠️ (warnings on some) | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Secure shred / overwrite | ⬜ | ✅ (shred, overwrite, wipe free space) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| App-specific cleaners (Chrome, Firefox, apt, etc.) | ⬜ | ✅ (CleanerML) | ❌ | ❌ | ✅ (categories) | ❌ | ❌ | ⚠️ (distro-specific) |
| Package cache cleaning (apt/dnf/pacman) | ⬜ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ⚠️ |
| Docker/Podman prune integration | ⬜ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| RC release model | ✅ | stable releases | stable | stable | stable | stable | stable | stable |

Key: ✅ done, ⬜ planned/future, ⚠️ partial, ❌ not present.

## BleachBit — strongest competitor to learn from

**What it does well:**
- `CleanerML` XML rules for per-application cleaning (Chrome cache, Firefox history, VLC, system logs, APT/Pacman/Snap/Yum/DNF caches, Thumbs.db, .DS_Store).
- CLI: `--list-cleaners`, `--preview <cleaner>`, `--clean <cleaner>`, `--overwrite`, `--shred <file>`, `--wipe-empty-space <dir>`.
- Preview-then-delete model: user sees what will be removed before confirming.
- Runs as normal user or `sudo` for system-wide cleaning.
- No undo — files deleted without shred are recoverable; shred overwrites before delete.

**Gaps Janitor can exploit:**
- BleachBit is GUI-first; CLI is secondary. Janitor is CLI-first, works over SSH, no GUI stack needed.
- BleachBit pulls in GTK/Python deps. Janitor is stdlib-only Rust — single static binary.
- BleachBit has no directory-size-first report with drill-down. Janitor leads with directory size.

**Ideas to port (post-stable):**
- CleanerML-style rule files for app-specific caches (browser, package manager).
- `--shred` secure overwrite option behind a flag.
- `--wipe-empty-space` option (optional, opt-in).

## ncdu — TUI reference for RC2

**What it does well:**
- Interactive ncurses browser over `du` results. Arrow navigation, drill into directories, `d` to delete with confirmation.
- Works over SSH, no GPU, single C binary.
- JSON export `-o` for scan comparison.
- `--confirm-delete` enabled by default.

**Gaps:**
- Disk-usage browser, not cleanup-focused. No candidate classification (temp/cache/log).
- No trash backend — deletes directly.
- No safety knowledge base for protected paths.

**Ideas to port (RC2 TUI):**
- Arrow-key directory tree, size-sorted, drill-down.
- `d`-style delete confirmation per item.
- Scan progress feedback while walking.

## Baobab / Filelight / QDirStat — visual references

- Visual ring chart / treemap. Fast at spotting large directories.
- Baobab does not delete inside the app (opens external file manager) — cautious by design.
- Janitor's CLI approach is complementary: no GPU needed, works headless.

## Janitor positioning

- **CLI-first, stdlib-only, zero-deps** — unlike BleachBit (GTK/Python), Stacer (Electron), Storage Sifter (GUI deps).
- **Directory-size-first + drill-down** — unlike ncdu (pure disk usage) and BleachBit (cleaner rules without size-led browsing).
- **Trash-safe + confirmation-per-item** — safer default than ncdu's direct delete or BleachBit's batch clean.
- **SAFETY.md knowledge base** — explicit protected-path rules, which none of the competitors publish as a standalone auditable file.

## Next steps from reverse engineering

1. Add CleanerML-style app-specific rule files (browser cache, package manager cache) — post-stable.
2. Add optional `--shred` secure overwrite — post-stable.
3. RC2 TUI: arrow-key tree + progress bar, modeled on ncdu's interaction but with Janitor's safety layer.
4. Publish `SAFETY.md` as a differentiator — auditable protected-path list.
