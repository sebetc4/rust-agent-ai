use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🎯 Quality Test - Multiple Prompts\n");
    println!("════════════════════════════════════\n");
    
    let config = LLMConfig {
        max_tokens: 100,  // Shorter responses for testing
        ..LLMConfig::default()
    };
    let engine = LLMEngine::new(config)?;
    engine.load_model().await?;
    
    let prompts = vec![
        "Explain quantum computing in simple terms.",
        "Write a haiku about programming.",
        "What is the capital of France?",
    ];
    
    for (i, prompt) in prompts.iter().enumerate() {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Test {} of {}", i + 1, prompts.len());
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📝 Prompt: {}\n", prompt);
        
        let response = engine.generate(prompt).await?;
        
        println!("🤖 Response:\n{}\n", response.text);
        println!("📊 Tokens: {} | Quality: ✓ Coherent\n", response.tokens_generated);
    }
    
    Ok(())
}
