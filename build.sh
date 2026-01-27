#!/bin/bash
# ParadoxOS Build and Run Script

set -e

echo "🌌 ParadoxOS Build Script"
echo "=========================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed!"
else
    echo "✅ Rust already installed"
fi

echo ""
echo "🔨 Building ParadoxOS kernel..."
cargo build --release

echo ""
echo "🧪 Running tests..."
cargo test --workspace --release

echo ""
echo "🚀 Running ParadoxOS kernel..."
echo "================================"
echo ""

cargo run --release --bin paradox-kernel
