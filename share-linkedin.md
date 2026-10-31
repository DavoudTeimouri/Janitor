# Janitor v0.1.0-rc.13 — Linux disk cleanup CLI in Rust

Janitor is a stdlib-first, zero-dependency CLI for cleaning Linux systems. It reports directory sizes first, lets you drill down to see which files are large, then removes candidates safely via Trash (`trash-put` / `gio trash`) with per-item confirmation — never raw `rm` by default.

**What it does:**
- Directory-size scan, sorted descending
- Drill-down into any directory to find large files
- Broken symlink (orphan) detection
- Interactive TUI with arrow navigation and progress bars
- Safe deletion backends with confirmation prompts
- RPM, DEB, and tarball releases via GitHub Actions

**Install:**
```sh
# DEB
sudo dpkg -i janitor_0.1.0-rc.13_amd64.deb

# RPM
sudo rpm -ivh janitor-0.1.0-1.x86_64.rpm

# Tarball
tar -xzf janitor-0.1.0-rc.13-x86_64-unknown-linux-gnu.tar.gz
sudo install janitor-0.1.0-rc.13/janitor /usr/local/bin/
```

**Usage:**
```sh
janitor scan ~/.cache
janitor drill ~/.cache
janitor clean ~/.cache --yes
janitor orphan ~
```

github.com/DavoudTeimouri/Janitor

#Linux #Rust #CLI #Sysadmin #DiskCleanup #OpenSource #DevTools
