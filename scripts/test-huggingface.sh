#!/bin/bash
# Quick test script for Hugging Face integration

set -e

echo "=== Testing Hugging Face Integration ==="
echo ""

# Test 1: Search models
echo "Test 1: Searching for 'qwen' models..."
cat > /tmp/hf_test_search.json << 'EOF'
{
  "search": "qwen",
  "limit": 5
}
EOF

echo "Search params: $(cat /tmp/hf_test_search.json)"
echo ""

# Test 2: Get model info
echo "Test 2: Getting info for 'Qwen/Qwen2.5-0.5B-Instruct-GGUF'..."
echo ""

# Note: These tests require the app to be running
# Run with: cargo run --release
# Then use the web interface or invoke Tauri commands

echo "=== Manual Testing Instructions ==="
echo ""
echo "1. Start the application:"
echo "   cd src-tauri && cargo run --release"
echo ""
echo "2. In the UI:"
echo "   - Click 'Download Models' button"
echo "   - Search for 'qwen' or 'llama'"
echo "   - Click on a model to see details"
echo "   - Select a GGUF file and download"
echo ""
echo "3. Expected behavior:"
echo "   - Search returns results with model info"
echo "   - Model details show files, tags, stats"
echo "   - Download saves to models/ directory"
echo ""
echo "4. Check downloaded files:"
echo "   ls -lh ../models/"
echo ""

echo "=== API Testing with curl ==="
echo ""
echo "You can also test the HF API directly:"
echo ""
echo "Search models:"
echo "curl 'https://huggingface.co/api/models?search=qwen&limit=5'"
echo ""
echo "Get model info:"
echo "curl 'https://huggingface.co/api/models/Qwen/Qwen2.5-0.5B-Instruct-GGUF'"
echo ""
echo "Download file (example):"
echo "curl -L 'https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf' -o test.gguf"
echo ""
