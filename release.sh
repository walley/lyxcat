#!/bin/bash
set -euo pipefail

# Simple release script for lyxcat
# Usage: ./release.sh [version]
#   If version is not provided, it uses the version from Cargo.toml

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Get version from Cargo.toml if not provided
VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    VERSION=$(awk -F\" '/^version[[:space:]]*=/ {print $2; exit}' Cargo.toml)
    if [ -z "$VERSION" ]; then
        echo "Error: Could not determine version from Cargo.toml" >&2
        exit 1
    fi
fi

# Validate version looks like a semver
if ! [[ "$VERSION" =~ ^v?[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
    echo "Error: Invalid version format: $VERSION" >&2
    echo "Expected format: v1.2.3 or 1.2.3" >&2
    exit 1
fi

# Remove leading 'v' if present for tag consistency
VERSION=${VERSION#v}

# Update changelog version
echo "lyxcat ($VERSION-1) unstable; urgency=medium

  * See GitHub releases for changes

 -- walley <walley@walley.org>  $(date -R)
" > debian/changelog

# Commit changelog
git add debian/changelog
git commit -m "Update changelog for v$VERSION"

# Create annotated tag
TAG="v$VERSION"
git tag -a "$TAG" -m "Release $TAG"

# Push tag to trigger release workflow
git push origin "$TAG"

echo "Release $TAG pushed!"
echo "The GitHub Actions workflow will build and upload the .deb file."
echo "Check: https://github.com/walley/lyxcat/releases/tag/$TAG"
