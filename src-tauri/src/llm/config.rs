/// Configuration du moteur LLM

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model_path: PathBuf,
    pub n_ctx: u32,
    pub n_threads: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub repeat_penalty: f32,
    pub max_tokens: u32,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/Qwen3-1.7B-IQ4_XS.gguf"),
            n_ctx: 2048,
            n_threads: 4,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            max_tokens: 512,  // Allow longer responses to complete reasoning + answer
        }
    }
}
