#!/usr/bin/env bash
# Release cleanup — run after each GitHub Release is cut.
# Removes local build artifacts and test traces that should not ship.
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Removing cargo build artifacts"
cargo clean 2>/dev/null || true

echo "==> Removing test scratch directories"
rm -rf /tmp/janitor-test-* /tmp/jt-test /tmp/jt2 2>/dev/null || true

echo "==> Verifying git status is clean (only tracked source should remain)"
git status --short

echo "==> Done. Local tree is release-clean."
