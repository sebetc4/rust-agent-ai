use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🎨 Custom Configuration Example\n");
    
    // Create custom configuration
    let mut config = LLMConfig::default();
    
    // Adjust generation parameters
    config.temperature = 0.9;      // More creative (0.0 = deterministic, 1.0 = very creative)
    config.top_p = 0.95;          // Nucleus sampling threshold
    config.top_k = 50;            // Top-K sampling
    config.repeat_penalty = 1.2;  // Higher = less repetition
    config.max_tokens = 150;      // Longer responses
    
    println!("Configuration:");
    println!("  Temperature: {}", config.temperature);
    println!("  Top-P: {}", config.top_p);
    println!("  Top-K: {}", config.top_k);
    println!("  Repeat Penalty: {}", config.repeat_penalty);
    println!("  Max Tokens: {}\n", config.max_tokens);
    
    // Initialize and load
    let engine = LLMEngine::new(config)?;
    engine.load_model().await?;
    
    // Test with creative prompt
    let prompts = vec![
        "Write a creative story about a robot learning to paint.",
        "Describe a futuristic city in poetic language.",
    ];
    
    for prompt in prompts {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Prompt: {}\n", prompt);
        
        let response = engine.generate(prompt).await?;
        println!("Response: {}\n", response.text);
        println!("Tokens: {}", response.tokens_generated);
        println!();
    }
    
    Ok(())
}
