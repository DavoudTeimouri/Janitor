# SAFETY.md — Unsafe-to-delete knowledge base

## Purpose

Janitor must never delete files that break the OS, package manager, containers, or user data. This file is the single source of truth for protected items. It is read before every scan; any path matching a rule below is excluded automatically.

## Protected categories

### 1. System boot & kernel
- `/boot/*` — kernels, initramfs, bootloader config. Deleting = unbootable system.
- `/efi/*`, `/boot/efi/*` — UEFI partition.

### 2. Package manager state
- `/var/lib/dpkg/*`, `/var/lib/apt/*`, `/var/cache/apt/archives/*.deb` (apt cache may be cleaned via `apt clean`, not `rm`).
- `/var/lib/pacman/*`, `/var/cache/pacman/pkg/*` (Arch).
- `/var/lib/rpm/*`, `/var/cache/yum/*`, `/var/cache/dnf/*` (RHEL/Fedora).
- `/var/lib/flatpak/*`, `/var/lib/snapd/*`.

### 3. System binaries & libraries
- `/bin/*`, `/sbin/*`, `/usr/bin/*`, `/usr/sbin/*`, `/usr/lib/*`, `/usr/share/*`, `/lib/*`, `/lib64/*`.
- `/etc/*` — system and application config.

### 4. Virtualization / containers
- Docker: `/var/lib/docker/*`, `/var/run/docker.sock` — images, containers, volumes. Clean via `docker system prune`, `docker volume prune`, `docker builder prune` — never `rm`.
- Podman: `/var/lib/containers/*` — clean via `podman system prune`.
- LXC/LXD: `/var/lib/lxc/*`, `/var/lib/lxd/*`.

### 5. Runtime kernels
- `/proc/*`, `/sys/*`, `/dev/*` — pseudo-filesystems. Never scan, never delete.

### 6. User home dangers
- `~/.ssh/*`, `~/.gnupg/*` — credentials.
- `~/.config/systemd/user/*` — user services.
- `~/.local/share/*` — app data (Flatpak, etc.). Clean per-app, not blanket.

### 7. Database & mail
- `/var/lib/mysql/*`, `/var/lib/postgresql/*`, `/var/lib/mongodb/*`.
- `/var/mail/*`, `/var/spool/mail/*`.

### 8. Logs (conditionally protected)
- `/var/log/*` — rotate via `logrotate`, `journalctl --vacuum-time=...`. Janitor may offer `.log` / `.log.old` as candidates but defaults to archive-not-delete.

## Cleanup command table (never raw `rm` for user-facing deletion)

| Target | Safe command | Notes |
|--------|-------------|-------|
| User files / directories | `trash-put <path>` | Moves to Trash; recoverable. Preferred. |
| User files / directories | `gio trash <path>` | GIO fallback if trash-cli absent. |
| Permanent erase (--yes only) | `rm -rf <path>` | Last resort, after explicit confirmation. |
| Docker images/containers | `docker system prune -a` | Never `rm -rf /var/lib/docker`. |
| Docker volumes | `docker volume prune` | |
| Docker build cache | `docker builder prune` | |
| Podman | `podman system prune -a` | |
| APT cache | `apt clean` / `apt autoclean` | |
| DNF/YUM cache | `dnf clean all` / `yum clean all` | |
| Pacman cache | `pacman -Sc` | |
| Journal logs | `journalctl --vacuum-time=3d` | |

## Decision rule

1. Is path under a protected prefix? → Skip entirely.
2. Is it a Docker/Podman artifact? → Suggest native prune command, not deletion.
3. Is it a user candidate file? → Offer `trash-put` / `gio trash`; permanent `rm` only behind `--yes`.

## Maintenance

Edit this file when new protected paths are discovered. Each rule must state the protected glob and the reason. Rules are additive; never remove a rule without a release note.
