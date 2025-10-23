#!/bin/bash

# Quick test script with predefined prompts

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Quick Model Test - Predefined Prompts${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Create quick test example
cat > "$SCRIPT_DIR/src-tauri/examples/quick_test.rs" << 'EOF'
use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("warn")
        .init();

    // Use default config with model path override
    let mut config = LLMConfig::default();
    config.max_tokens = 50; // Shorter for quick tests

    println!("🚀 Loading model...");
    let engine = LLMEngine::new(config)?;
    engine.load_model().await?;
    println!("✅ Model loaded!\n");

    // Predefined test prompts
    let prompts = vec![
        "What is Rust?",
        "Explain AI in simple terms.",
        "Write a haiku about programming.",
    ];

    for (i, prompt) in prompts.iter().enumerate() {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Test #{}: {}", i + 1, prompt);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        let start = std::time::Instant::now();
        
        match engine.generate(prompt).await {
            Ok(response) => {
                let duration = start.elapsed();
                println!("Response: {}", response.text);
                println!("\n📊 Stats:");
                println!("  • Tokens: {}", response.tokens_generated);
                println!("  • Time: {:.2}s", duration.as_secs_f64());
                if response.tokens_generated > 0 {
                    println!("  • Speed: {:.1} tokens/s", 
                        response.tokens_generated as f64 / duration.as_secs_f64());
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
        println!();
    }

    println!("✅ All tests completed!");
    Ok(())
}
EOF

echo -e "${BLUE}Building quick test...${NC}"
cargo build --manifest-path "$SCRIPT_DIR/src-tauri/Cargo.toml" --example quick_test 2>&1 | grep -E "(Compiling|Finished)" || true

echo -e "\n${GREEN}Running tests...${NC}\n"
cargo run --manifest-path "$SCRIPT_DIR/src-tauri/Cargo.toml" --example quick_test 2>&1
