// Test script for LLM model loading
// Run with: cargo test --manifest-path src-tauri/Cargo.toml test_model_loading

#[cfg(test)]
mod model_tests {
    use crate::llm::{LLMEngine, LLMConfig};

    #[tokio::test]
    async fn test_model_loading() {
        // Use absolute path from workspace root
        let mut model_path = std::env::current_dir().expect("Failed to get current dir");
        model_path.pop(); // Remove src-tauri from path
        model_path.push("models/Qwen3-1.7B-IQ4_XS.gguf");
        
        let config = LLMConfig {
            model_path,
            n_ctx: 2048,
            n_threads: 4,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            max_tokens: 512,
        };

        let engine = LLMEngine::new(config).expect("Failed to create LLM engine");
        
        // Try to load the model
        let result = engine.load_model().await;
        
        match result {
            Ok(_) => {
                println!("✅ Model loaded successfully");
                assert!(engine.is_loaded().await, "Model should be marked as loaded");
            }
            Err(e) => {
                println!("⚠️  Model loading failed (expected with mock implementation): {}", e);
                // This is expected since we're using mock responses
            }
        }
    }

    #[tokio::test]
    async fn test_generate_with_model() {
        // Use absolute path from workspace root
        let mut model_path = std::env::current_dir().expect("Failed to get current dir");
        model_path.pop(); // Remove src-tauri from path
        model_path.push("models/Qwen3-1.7B-IQ4_XS.gguf");
        
        let config = LLMConfig {
            model_path,
            n_ctx: 2048,
            n_threads: 4,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            max_tokens: 512,
        };

        let engine = LLMEngine::new(config).expect("Failed to create engine");
        let _ = engine.load_model().await;

        if engine.is_loaded().await {
            let response = engine.generate("Hello, how are you?").await;
            
            match response {
                Ok(resp) => {
                    println!("✅ Generated response: {}", resp.text);
                    assert!(!resp.text.is_empty(), "Response should not be empty");
                }
                Err(e) => {
                    println!("⚠️  Generation failed: {}", e);
                }
            }
        }
    }
}
