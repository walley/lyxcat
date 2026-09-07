#!/bin/bash
# Build Debian package for lyxcat

set -e

echo "Building lyxcat Debian package..."

# Check if dpkg-buildpackage is available
if ! command -v dpkg-buildpackage &> /dev/null; then
    echo "Error: dpkg-buildpackage not found. Install build-essential and devscripts:"
    echo "  sudo apt-get install build-essential devscripts"
    exit 1
fi

# Build the package
dpkg-buildpackage -uc -us

echo ""
echo "Build complete! Check the parent directory for .deb and .changes files."
