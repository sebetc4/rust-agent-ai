use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Configure LLM with default settings
    let config = LLMConfig::default();

    println!("\n🤖 Initializing LLM engine...");
    println!("📋 Configuration:");
    println!("   - Temperature: {}", config.temperature);
    println!("   - Top-P: {}", config.top_p);
    println!("   - Top-K: {}", config.top_k);
    println!("   - Repeat Penalty: {}", config.repeat_penalty);
    println!("   - Max Tokens: {}", config.max_tokens);
    println!("   - Context Size: {}", config.n_ctx);
    println!("   - Threads: {}\n", config.n_threads);
    let engine = LLMEngine::new(config)?;
    
    println!("📦 Loading model...");
    engine.load_model().await?;
    
    println!("✅ Model loaded successfully!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Interactive mode
    use std::io::{self, Write};
    
    loop {
        print!("💬 Your prompt (or 'quit'/'clear'/'history'): ");
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
        
        if prompt.eq_ignore_ascii_case("clear") {
            engine.clear_conversation().await;
            println!("\n🧹 Conversation history cleared!\n");
            continue;
        }
        
        if prompt.eq_ignore_ascii_case("history") {
            let history = engine.get_conversation_history().await;
            println!("\n📜 Conversation History:");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            if history.is_empty() {
                println!("(empty)");
            } else {
                println!("{}", history);
            }
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            continue;
        }
        
        println!("\n🔄 Generating response...\n");
        
        match engine.generate(prompt).await {
            Ok(response) => {
                println!("🤖 Response: {}", response.text);
                println!("\n📊 Tokens generated: {}", response.tokens_generated);
                
                // Show conversation history token count
                let history = engine.get_conversation_history().await;
                let history_lines = history.lines().count();
                println!("💬 Conversation turns: {}", history_lines / 3); // Each turn has 3 lines in format
                
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
    }
    
    Ok(())
}
