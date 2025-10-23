/// Configuration du moteur LLM

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model_path: String,
    pub max_tokens: usize,
    pub context_size: usize,
    pub n_ctx: usize, // Alias for context_size for compatibility
    pub n_threads: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub repeat_penalty: f32,
    pub use_gpu: bool,
    pub n_gpu_layers: u32,
    pub main_gpu: i32,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_path: "models/Qwen3-1.7B-IQ4_XS.gguf".to_string(),
            context_size: 2048,
            n_ctx: 2048,
            n_threads: 4,
            max_tokens: 512,
            temperature: 0.8,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            use_gpu: false,
            n_gpu_layers: 0, // 0 means CPU only, set to u32::MAX for all layers
            main_gpu: 0,
        }
    }
}
