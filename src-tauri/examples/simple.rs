use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Simple example: one-shot generation
    
    println!("🚀 Simple LLM Example");
    println!("━━━━━━━━━━━━━━━━━━━━\n");
    
    // Use default configuration
    let config = LLMConfig::default();
    
    // Initialize engine
    let engine = LLMEngine::new(config)?;
    engine.load_model().await?;
    
    // Generate response
    let prompt = "What is Rust programming language?";
    println!("Prompt: {}\n", prompt);
    
    let response = engine.generate(prompt).await?;
    
    println!("Response: {}", response.text);
    println!("\nTokens: {}", response.tokens_generated);
    
    Ok(())
}
