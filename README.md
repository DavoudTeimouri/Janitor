# janitor

CCleaner-style Linux cleanup tool written in Rust. Finds and removes temp/dump/cache/log files,
oversized files, and broken symlinks under a target root. Interactive by default; `--yes` deletes
without prompting. Stdlib-only (no external crates).

## Build

```sh
cargo build --release
```

## Usage

```sh
# preview what would be cleaned under /var/log and ~/.cache
janitor scan /var/log
janitor scan ~/.cache

# interactive cleanup (prompts per file)
janitor clean ~

# non-interactive cleanup
janitor clean /tmp --yes

# list broken symlinks
janitor orphan ~
```

## Packaging

RPM spec: `packaging/janitor.spec`  
DEB control: `packaging/debian/control`

Build RPM/DEB outside this repo (no packaging crates bundled). See each file for notes.

## Scope notes (ponytail)

- Web UI: not included. Add behind a `--web` feature gate with a minimal HTTP server when needed.
- Orphaned systemd units / services: not detected yet. v1 covers broken symlinks + known file patterns.
- CLI parsing is hand-rolled (`std::env::args`). Switch to `clap` if subcommands grow.
