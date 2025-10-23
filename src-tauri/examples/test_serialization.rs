use agents_rs_lib::huggingface::HuggingFaceClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HuggingFaceClient::new()?;
    
    println!("Testing JSON serialization for frontend...\n");
    
    // Test with a gated model
    let info = client.get_model_info("meta-llama/Llama-3.1-8B-Instruct").await?;
    
    // Serialize to JSON (like Tauri does)
    let json = serde_json::to_string_pretty(&info)?;
    
    println!("Serialized ModelInfo (first 800 chars):");
    println!("{}", &json.chars().take(800).collect::<String>());
    println!("...\n");
    
    // Verify it can be deserialized
    let _parsed: serde_json::Value = serde_json::from_str(&json)?;
    println!("✓ JSON is valid and can be parsed!");
    
    // Check the gated field specifically
    let value: serde_json::Value = serde_json::from_str(&json)?;
    if let Some(gated) = value.get("gated") {
        println!("✓ Gated field: {}", gated);
        match gated {
            serde_json::Value::Bool(b) => println!("  Type: Boolean({})", b),
            serde_json::Value::String(s) => println!("  Type: String(\"{}\")", s),
            _ => println!("  Type: Other"),
        }
    }
    
    Ok(())
}
