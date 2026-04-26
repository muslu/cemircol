#!/bin/bash
set -e
clear
echo ""
echo "=== CemirCol Build Script (Ubuntu) ==="
echo ""

echo "[*] Installing system dependencies..."
sudo apt update
sudo apt install -y curl build-essential python3 python3-pip python3-venv
sudo systemctl daemon-reload

# 1. Check for Rust toolchain
if ! command -v cargo &> /dev/null; then
    echo "[!] Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# 2. Set up Python virtual environment
if [ ! -d ".venv" ]; then
    echo "[*] Creating Python virtual environment in .venv..."
    python3 -m venv .venv
fi

echo "[*] Activating virtual environment..."
source .venv/bin/activate
clear

# 3. Install Python dependencies
echo "[*] Installing Python dependencies in virtual environment..."
pip install -r requirements.txt

# 4. Build the Rust library and install into Python environment
echo "[*] Building CemirCol (release mode)..."
maturin develop --release
clear
echo ""
echo "=== Build Complete ==="
echo "Please remember to activate the virtual environment before using:"
echo "  source .venv/bin/activate"
echo "  python3 -c \"from cemircol import CemircolWriter, CemircolReader\""
echo ""
echo "To publish to PyPI:"
echo "  maturin build --release"
echo "  twine upload target/wheels/*"