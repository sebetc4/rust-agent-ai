use agents_rs_lib::huggingface::{HuggingFaceClient, ModelSearchParams};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug,agents_rs_lib=trace")
        .init();

    println!("=== Testing GGUF Model Discovery ===\n");

    // Create HuggingFace client
    let client = HuggingFaceClient::new()?;

    // Test 1: Discover popular GGUF models (no search query)
    println!("Test 1: Discovering popular GGUF models...");
    let params = ModelSearchParams::new()
        .sort_by_downloads()
        .descending()
        .limit(5);

    match client.discover_gguf_models(params).await {
        Ok(models) => {
            println!("✅ Found {} GGUF models:", models.len());
            for model in &models {
                println!("\n  📦 {}", model.repo_id);
                println!("     Author: {}", model.author);
                println!("     Downloads: {}", model.downloads);
                println!("     Likes: {}", model.likes);
                println!("     GGUF Files: {}", model.gguf_files.len());
                
                // Show first 3 files
                for (i, file) in model.gguf_files.iter().take(3).enumerate() {
                    let quant = file.quantization.as_ref()
                        .map(|q| format!(" [{}]", q))
                        .unwrap_or_default();
                    println!(
                        "       {}. {} ({:.2} GB){}",
                        i + 1,
                        file.filename,
                        file.size as f64 / 1024.0 / 1024.0 / 1024.0,
                        quant
                    );
                }
                
                if model.gguf_files.len() > 3 {
                    println!("       ... and {} more files", model.gguf_files.len() - 3);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }

    // Test 2: Search for specific model
    println!("\n\nTest 2: Searching for 'llama' GGUF models...");
    let params = ModelSearchParams::new()
        .search("llama")
        .sort_by_downloads()
        .descending()
        .limit(3);

    match client.discover_gguf_models(params).await {
        Ok(models) => {
            println!("✅ Found {} Llama GGUF models:", models.len());
            for model in &models {
                println!("\n  📦 {}", model.repo_id);
                println!("     Downloads: {} | Likes: {}", model.downloads, model.likes);
                
                // Group by quantization
                let mut quants: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                for file in &model.gguf_files {
                    if let Some(q) = &file.quantization {
                        *quants.entry(q.clone()).or_insert(0) += 1;
                    }
                }
                
                println!("     Quantizations available:");
                for (quant, count) in quants {
                    println!("       - {}: {} file(s)", quant, count);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }

    // Test 3: Search by author
    println!("\n\nTest 3: Searching for models by 'TheBloke'...");
    let params = ModelSearchParams::new()
        .author("TheBloke")
        .sort_by_likes()
        .descending()
        .limit(2);

    match client.discover_gguf_models(params).await {
        Ok(models) => {
            println!("✅ Found {} models by TheBloke:", models.len());
            for model in &models {
                println!("\n  📦 {}", model.repo_id);
                println!("     Likes: {} | Downloads: {}", model.likes, model.downloads);
                println!("     Total GGUF files: {}", model.gguf_files.len());
            }
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }

    println!("\n=== Tests completed ===");
    Ok(())
}
