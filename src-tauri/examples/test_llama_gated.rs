use agents_rs_lib::huggingface::HuggingFaceClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    let client = HuggingFaceClient::new()?;
    
    println!("Testing Llama 3.1 model (has gated='manual')...\n");
    
    match client.get_model_info("meta-llama/Llama-3.1-8B-Instruct").await {
        Ok(info) => {
            println!("✓ SUCCESS! Model info retrieved:");
            println!("  Model ID: {}", info.model_id);
            println!("  Author: {}", info.author.as_deref().unwrap_or("unknown"));
            println!("  Downloads: {}", info.downloads.unwrap_or(0));
            println!("  Likes: {}", info.likes.unwrap_or(0));
            
            if let Some(gated) = &info.gated {
                println!("  Gated: {:?} (is_gated: {})", gated, gated.is_gated());
            }
            
            println!("  Private: {}", info.private);
            println!("  Disabled: {:?}", info.disabled);
            println!("  Files: {} total", info.siblings.len());
            
            let safetensors: Vec<_> = info.siblings.iter()
                .filter(|f| f.filename.ends_with(".safetensors"))
                .take(3)
                .collect();
            
            if !safetensors.is_empty() {
                println!("  Sample files:");
                for file in safetensors {
                    println!("    - {}", file.filename);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ FAILED: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
