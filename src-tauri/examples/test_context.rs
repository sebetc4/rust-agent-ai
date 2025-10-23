use agents_rs_lib::llm::{LLMEngine, config::LLMConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧪 Testing Context Persistence\n");
    println!("═════════════════════════════════════\n");
    
    let config = LLMConfig {
        max_tokens: 100,
        ..LLMConfig::default()
    };
    
    let engine = LLMEngine::new(config)?;
    engine.load_model().await?;
    
    println!("✅ Model loaded\n");
    
    // Test 1: Set a name
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Test 1: Setting context");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let prompt1 = "My name is Alice. Remember this.";
    println!("👤 User: {}\n", prompt1);
    
    let response1 = engine.generate(prompt1).await?;
    println!("🤖 Assistant: {}\n", response1.text);
    
    // Test 2: Check if model remembers the name
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Test 2: Recalling context");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let prompt2 = "What is my name?";
    println!("👤 User: {}\n", prompt2);
    
    let response2 = engine.generate(prompt2).await?;
    println!("🤖 Assistant: {}\n", response2.text);
    
    // Check if the response mentions "Alice"
    if response2.text.contains("Alice") {
        println!("✅ SUCCESS: Model remembered the name!");
    } else {
        println!("❌ FAILED: Model did not remember the name");
        println!("   Expected response to contain 'Alice'");
    }
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📜 Full conversation history:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}", engine.get_conversation_history().await);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Test 3: Clear and verify
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Test 3: Clearing context");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    engine.clear_conversation().await;
    
    let prompt3 = "What is my name?";
    println!("👤 User: {}\n", prompt3);
    
    let response3 = engine.generate(prompt3).await?;
    println!("🤖 Assistant: {}\n", response3.text);
    
    if !response3.text.contains("Alice") {
        println!("✅ SUCCESS: Context was cleared correctly!");
    } else {
        println!("⚠️  WARNING: Model still remembers after clear");
    }
    
    Ok(())
}
