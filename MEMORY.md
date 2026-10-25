# MEMORY.md — Environment facts

## Host (docker backend)

- OS: Linux 7.0.0-1013-oracle
- Architecture: aarch64-unknown-linux-gnu
- Terminal backend: docker (`terminal`, `read_file`, `write_file`, `patch`, `search_files` all inside docker)
- Working directory: /workspace

## Rust toolchain

- System rustc: /usr/local/bin/rustc -> /opt/rust-1.85.1/bin/rustc (1.85.1) — BROKEN: std rlibs are zero-length stubs, cannot compile.
- Working rustc: /opt/rust-tools/rustc-1.85.1-aarch64-unknown-linux-gnu/rustc/bin/rustc (1.85.1)
- Working std: /opt/rust-tools/rust-std-1.85.1-aarch64-unknown-linux-gnu/rust-std-aarch64-unknown-linux-gnu
- Build command:
  export PATH="/opt/rust-tools/rustc-1.85.1-aarch64-unknown-linux-gnu/rustc/bin:$PATH"
  export RUSTFLAGS="--sysroot /opt/rust-tools/rust-std-1.85.1-aarch64-unknown-linux-gnu/rust-std-aarch64-unknown-linux-gnu"
  cargo build --release
- No Windows target available (Windows code must compile on Linux to be type-checked).
- No systemd (systemd paths untestable here).
- No rustup.

## Janitor build artifacts

- Source: /workspace/project/github/public-repository/Janitor/src/
- Binary: /workspace/project/github/public-repository/Janitor/target/release/janitor
- Tests: 3 pass (scan finds temp/dump/large, skips other, directory size aggregates)

## GitHub

- Repo: github.com/DavoudTeimouri/Janitor
- Branch: main
- Tag: v0.1.0-rc.2
- SSH key: /root/.ssh/id_ed25519 (authenticates as DavoudTeimouri)
- `gh` CLI not installed.
