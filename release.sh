#!/bin/bash
set -e

# ParadoxOS Release Script
VERSION="0.4.0"
RELEASE_NAME="paradoxos-kernel-v$VERSION"
OUTPUT_DIR="release"

echo "🌌 Packaging ParadoxOS Kernel v$VERSION..."

# Ensure cargo is in path
source ~/.cargo/env || true

# 1. Clean and Build Release
echo "👉 Building release binaries..."
cargo clean
cargo build --release

# 2. Prepare Release Director
rm -rf $OUTPUT_DIR
mkdir -p $OUTPUT_DIR
mkdir -p $OUTPUT_DIR/bin
mkdir -p $OUTPUT_DIR/docs

# 3. Copy Binaries
cp target/release/paradox-kernel $OUTPUT_DIR/bin/
cp target/release/libparadox_kernel.rlib $OUTPUT_DIR/bin/ 2>/dev/null || true

# 4. Copy Documentation
cp IMPLEMENTATION_COMPLETE.md $OUTPUT_DIR/docs/
cp PHASE5_STATUS.md $OUTPUT_DIR/docs/
cp PHASE6_STATUS.md $OUTPUT_DIR/docs/
cp README.md $OUTPUT_DIR/docs/

# 5. Create README for release
cat > $OUTPUT_DIR/README.txt <<EOF
ParadoxOS Kernel v$VERSION
==========================

This is the production release of the ParadoxOS Kernel, a physics-native operating system.

Contents:
- bin/: Executable kernel binaries
- docs/: Implementation details and status reports

Quick Start:
  ./bin/paradox-kernel

For more info, read docs/IMPLEMENTATION_COMPLETE.md

(c) 2026 ParadoxOS Team
EOF

# 6. Compress
echo "👉 Compressing archive..."
tar -czf $RELEASE_NAME.tar.gz $OUTPUT_DIR

echo "✅ Release ready: $RELEASE_NAME.tar.gz"
echo "📦 Output directory: $OUTPUT_DIR"
