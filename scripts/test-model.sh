#!/usr/bin/env bash
# Test script for model loading and generation

set -e

echo "🔍 Testing agents-rs model loading and generation..."
echo ""

# Test 1: Model loading
echo "📦 Test 1: Model loading"
cargo test --manifest-path src-tauri/Cargo.toml test_model_loading -- --nocapture
echo ""

# Test 2: Generation
echo "💬 Test 2: Text generation"
cargo test --manifest-path src-tauri/Cargo.toml test_generate_with_model -- --nocapture
echo ""

# Test 3: All LLM tests
echo "🧪 Test 3: All LLM module tests"
cargo test --manifest-path src-tauri/Cargo.toml llm::tests -- --nocapture
echo ""

echo "✅ All tests completed successfully!"
