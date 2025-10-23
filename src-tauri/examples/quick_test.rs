use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("warn")
        .init();

    let config = LLMConfig {
        model_path: "models/Qwen3-1.7B-IQ4_XS.gguf".to_string(),
        n_ctx: 2048,
        n_threads: 4,
        temperature: 0.7,
        top_p: 0.9,
        top_k: 40,
        repeat_penalty: 1.1,
        max_tokens: 50,
        context_size: 2048,  // Added missing field
        use_gpu: false,      // Set to false since GPU features are disabled
        n_gpu_layers: u32::MAX,  // Use maximum value for all GPU layers
        main_gpu: 0,
    };

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
