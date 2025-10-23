use agents_rs_lib::huggingface::{HuggingFaceClient, ModelSearchParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    println!("Testing Hugging Face API integration...\n");

    let client = HuggingFaceClient::new()?;

    // Test 1: Search models
    println!("=== Test 1: Search for 'qwen' models ===");
    let params = ModelSearchParams::new()
        .search("qwen")
        .limit(3);

    match client.search_models(params).await {
        Ok(models) => {
            println!("✓ Search successful! Found {} models:", models.len());
            for model in &models {
                println!("  - {} (by {})", model.model_id, model.author.as_deref().unwrap_or("unknown"));
            }
        }
        Err(e) => {
            eprintln!("✗ Search failed: {}", e);
            return Err(e);
        }
    }

    println!("\n=== Test 2: Get model info ===");
    match client.get_model_info("Qwen/Qwen2.5-0.5B-Instruct-GGUF").await {
        Ok(info) => {
            println!("✓ Model info retrieved successfully!");
            println!("  Model ID: {}", info.model_id);
            println!("  Author: {}", info.author.as_deref().unwrap_or("unknown"));
            println!("  Downloads: {}", info.downloads.unwrap_or(0));
            println!("  Likes: {}", info.likes.unwrap_or(0));
            println!("  Files: {} total", info.siblings.len());
            
            let gguf_files: Vec<_> = info.siblings.iter()
                .filter(|f| f.filename.ends_with(".gguf"))
                .collect();
            println!("  GGUF files: {}", gguf_files.len());
            
            for file in gguf_files.iter().take(3) {
                println!("    - {}", file.filename);
            }
        }
        Err(e) => {
            eprintln!("✗ Get model info failed: {}", e);
            return Err(e);
        }
    }

    println!("\n✓ All tests passed!");
    Ok(())
}
