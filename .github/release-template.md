# Release template

Copy this file to `.github/RELEASES/vX.Y.Z-rc.N.md` (or use as GitHub Release body) and fill in every section.

## [vX.Y.Z-rc.N] — YYYY-MM-DD

### Status
Pre-release candidate. Not for production. Test on non-critical systems only.

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

### Installation
```sh
# RPM
sudo rpm -ivh janitor-0.1.0-1.x86_64.rpm

# DEB
sudo dpkg -i janitor_0.1.0_amd64.deb

# Tarball
tar -xzf janitor-0.1.0-aarch64-unknown-linux-gnu.tar.gz
sudo install janitor /usr/local/bin/
```

### Verification checklist
- [ ] Builds on target arch
- [ ] `cargo test` passes
- [ ] Scan runs on `/tmp`, `~/.cache`, `/var/log`
- [ ] Cleanup uses trash-put/gio trash by default
- [ ] Protected paths are skipped
- [ ] `--yes` skips prompts
- [ ] CHANGELOG updated
- [ ] Git tag created
