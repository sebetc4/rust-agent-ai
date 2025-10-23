#!/bin/bash

# Interactive LLM testing script for agents_rs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MODEL_PATH="$SCRIPT_DIR/models/Qwen3-1.7B-IQ4_XS.gguf"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  agents_rs - Interactive Model Test${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Check if model exists
if [ ! -f "$MODEL_PATH" ]; then
    echo -e "${YELLOW}⚠️  Model not found at: $MODEL_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Model found: Qwen3-1.7B-IQ4_XS.gguf"
echo -e "${GREEN}✓${NC} Size: $(du -h "$MODEL_PATH" | cut -f1)\n"

# Check if test_model.rs exists, create it if not
if [ ! -f "$SCRIPT_DIR/src-tauri/examples/test_model.rs" ]; then
    echo -e "${YELLOW}⚠️  Creating test_model.rs...${NC}"
    # Create a simple test binary
    cat > "$SCRIPT_DIR/src-tauri/examples/test_model.rs" << 'EOF'
use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Configure LLM with default settings
    let mut config = LLMConfig::default();
    config.max_tokens = 100; // Reasonable default for interactive

    println!("\n🤖 Initializing LLM engine...");
    let engine = LLMEngine::new(config)?;
    
    println!("📦 Loading model...");
    engine.load_model().await?;
    
    println!("✅ Model loaded successfully!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Interactive mode
    use std::io::{self, Write};
    
    loop {
        print!("💬 Your prompt (or 'quit' to exit): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let prompt = input.trim();
        
        if prompt.is_empty() {
            continue;
        }
        
        if prompt.eq_ignore_ascii_case("quit") || prompt.eq_ignore_ascii_case("exit") {
            println!("\n👋 Goodbye!");
            break;
        }
        
        println!("\n🔄 Generating response...\n");
        
        match engine.generate(prompt).await {
            Ok(response) => {
                println!("🤖 Response: {}", response.text);
                println!("\n📊 Tokens generated: {}", response.tokens_generated);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
    }
    
    Ok(())
}
EOF
else
    echo -e "${GREEN}✓${NC} Using existing test_model.rs"
fi

echo -e "${BLUE}Building test executable...${NC}"
cargo build --manifest-path "$SCRIPT_DIR/src-tauri/Cargo.toml" --example test_model

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Build successful!\n"
    echo -e "${BLUE}Starting interactive session...${NC}\n"
    
    # Run the example
    cargo run --manifest-path "$SCRIPT_DIR/src-tauri/Cargo.toml" --example test_model
else
    echo -e "${YELLOW}⚠️  Build failed${NC}"
    exit 1
fi
