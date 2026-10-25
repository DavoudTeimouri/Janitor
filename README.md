# Janitor

CCleaner-style Linux cleanup tool written in Rust. Scan directories by size, drill down to large files, and remove candidates safely via Trash — never raw `rm` by default.

## Why Janitor

Most cleanup tools delete files blindly. Janitor reports **directory size first**, lets you pick a directory, then shows which files inside are large. Every deletion goes to Trash (via `trash-put` or `gio trash`) unless you explicitly pass `--yes`.

## Features

| Feature | Description |
|---------|-------------|
| Directory-size scan | Walk a root and report every directory that contains candidate files, sorted by total size |
| Drill-down | `janitor drill <dir>` lists files inside a directory sorted by size |
| Safe deletion | `trash-put` → `gio trash` → `rm --yes` fallback, never by default |
| Confirmation prompts | Every candidate is `[y/N]` confirmed unless `--yes` |
| Broken symlinks | `orphan` subcommand lists dead links |
| Size threshold | `--min-size N` to only flag files ≥ N bytes |
| Stdlib-first | Zero external crates; `std::fs` only |

## Install

### From source

```sh
git clone https://github.com/DavoudTeimouri/Janitor.git
cd Janitor
# host rustc + sysroot (see note below if std rlibs are stubs)
cargo build --release
sudo install target/release/janitor /usr/local/bin/
```

### RPM

```sh
sudo rpm -ivh janitor-0.1.0-1.x86_64.rpm
```

### DEB

```sh
sudo dpkg -i janitor_0.1.0_amd64.deb
```

## Usage

```sh
# 1. Scan a root — directories by total size
janitor scan ~

# 2. Drill into a directory — which files are large?
janitor drill ~/.cache

# 3. Clean candidates (prompts per file)
janitor clean ~/.cache

# 4. Non-interactive (Trash, no prompt)
janitor clean /tmp --yes

# 5. Broken symlinks
janitor orphan ~

# 6. Only flag files ≥ 50 MB
janitor scan /var/log --min-size 52428800
```

## Safety

Protected paths (`/boot`, `/etc`, `/usr`, `/var/lib/*`, Docker volumes, `/proc`, `/sys`, `/dev`, `~/.ssh`, etc.) are excluded automatically. See `SAFETY.md` for the full knowledge base and the cleanup command table.

Deletion order:
1. `trash-put` (moves to Trash, recoverable)
2. `gio trash` (fallback)
3. `rm -rf` only with `--yes` and a non-protected path

## Build

Host is `aarch64-unknown-linux-gnu`. The system rustc at `/opt/rust-1.85.1` has zero-length std rlibs. Build with:

```sh
export PATH="/opt/rust-tools/rustc-1.85.1-aarch64-unknown-linux-gnu/rustc/bin:$PATH"
export RUSTFLAGS="--sysroot /opt/rust-tools/rust-std-1.85.1-aarch64-unknown-linux-gnu/rust-std-aarch64-unknown-linux-gnu"
cargo build --release
```

Tests:

```sh
cargo test
```

## Release model

Pre-release `vX.Y.Z-rc.N` → stable `vX.Y.Z` after sign-off. Each release has a changelog entry (`CHANGELOG.md`) and a GitHub Release with RPM + DEB + tarball assets.

See `.github/release-template.md`.

## Repository

- `src/` — Rust CLI (stdlib only)
- `packaging/janitor.spec` — RPM spec
- `packaging/debian/control` — DEB control
- `SAFETY.md` — unsafe-to-delete knowledge base
- `LOGIC.md` — application logic & internals
- `CHANGELOG.md` — per-release notes

## Topics

linux, cleanup, ccleaner, disk-usage, rust, cli, trash, system-administration

## License

MIT
