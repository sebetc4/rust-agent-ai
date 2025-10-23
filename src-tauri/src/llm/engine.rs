/// LLM Engine Module
/// Native llama.cpp integration for standalone all-in-one application

use super::config::LLMConfig;
use anyhow::{Context, Result};
use llama_cpp_2::{
    llama_backend::LlamaBackend,
    llama_batch::LlamaBatch,
    model::{AddBos, LlamaModel, params::LlamaModelParams},
    sampling::LlamaSampler,
};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

/// LLM model response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub tokens_generated: usize,
    pub done: bool,
}

/// Tool call detected in response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Wrapper for LlamaModel to make it Send + Sync
/// SAFETY: We ensure single-threaded access via Mutex
struct ModelWrapper(LlamaModel);
unsafe impl Send for ModelWrapper {}
unsafe impl Sync for ModelWrapper {}

/// Main LLM engine with native llama.cpp integration
pub struct LLMEngine {
    pub config: LLMConfig,
    backend: Arc<LlamaBackend>,
    model: Arc<Mutex<Option<ModelWrapper>>>,
    conversation_history: Arc<Mutex<String>>,
}

impl LLMEngine {
    /// Create a new LLM engine instance
    pub fn new(config: LLMConfig) -> Result<Self> {
        info!("Initializing native llama.cpp LLM engine...");
        
        // Initialize llama.cpp backend
        let backend = LlamaBackend::init()
            .context("Failed to initialize llama.cpp backend")?;
        
        Ok(Self {
            config,
            backend: Arc::new(backend),
            model: Arc::new(Mutex::new(None)),
            conversation_history: Arc::new(Mutex::new(String::new())),
        })
    }

    /// Load the LLM model from the configured path
    pub async fn load_model(&self) -> Result<()> {
        let mut model_lock = self.model.lock().await;
        
        // Check if already loaded
        if model_lock.is_some() {
            info!("Model already loaded");
            return Ok(());
        }
        
        // Check if model file exists
        let model_path = std::path::Path::new(&self.config.model_path);
        if !model_path.exists() {
            anyhow::bail!(
                "Model file not found: {}",
                model_path.display()
            );
        }

        info!("Loading model from: {}", model_path.display());
        
        // Configure model parameters with GPU settings
        let mut model_params = LlamaModelParams::default();
        
        if self.config.use_gpu {
            info!("GPU acceleration enabled");
            info!("GPU layers: {}", if self.config.n_gpu_layers == u32::MAX { "all".to_string() } else { self.config.n_gpu_layers.to_string() });
            info!("Main GPU: {}", self.config.main_gpu);
            
            model_params = model_params
                .with_n_gpu_layers(self.config.n_gpu_layers)
                .with_main_gpu(self.config.main_gpu);
        } else {
            info!("GPU acceleration disabled - using CPU only");
            model_params = model_params.with_n_gpu_layers(0);
        }
        
        // Load the model with GPU parameters
        let model = LlamaModel::load_from_file(
            &self.backend,
            &self.config.model_path,
            &model_params,
        )
        .context("Failed to load GGUF model")?;
        
        info!("Model loaded successfully!");
        info!("Context size: {} tokens", self.config.n_ctx);
        info!("Threads: {}", self.config.n_threads);
        info!("GPU info: {}", self.gpu_info());
        
        *model_lock = Some(ModelWrapper(model));
        
        Ok(())
    }

    /// Detect GPU availability and return recommended configuration
    pub fn detect_gpu_config() -> (bool, String) {
        // Check for NVIDIA GPU (CUDA)
        #[cfg(feature = "cuda")]
        {
            // This would ideally check nvidia-smi or CUDA runtime
            // For now, we assume CUDA is available if compiled with cuda feature
            return (true, "CUDA GPU detected".to_string());
        }
        
        // Check for Apple Silicon (Metal)
        #[cfg(all(target_os = "macos", feature = "metal"))]
        {
            // Check if we're on Apple Silicon
            if std::env::consts::ARCH == "aarch64" {
                return (true, "Apple Silicon Metal GPU detected".to_string());
            }
        }
        
        // Fallback to CPU
        (false, "No GPU acceleration available - using CPU".to_string())
    }

    /// Get GPU information and recommendations
    pub fn gpu_info(&self) -> String {
        let (has_gpu, info) = Self::detect_gpu_config();
        
        if self.config.use_gpu && has_gpu {
            format!("GPU: Enabled - {}", info)
        } else if self.config.use_gpu && !has_gpu {
            format!("GPU: Requested but not available - {}", info)
        } else {
            format!("GPU: Disabled - {}", info)
        }
    }

    /// Check if model is currently loaded
    pub async fn is_loaded(&self) -> bool {
        self.model.lock().await.is_some()
    }

    /// Clear conversation history to start a fresh conversation
    pub async fn clear_conversation(&self) {
        let mut history = self.conversation_history.lock().await;
        history.clear();
        info!("Conversation history cleared");
    }

    /// Get current conversation history
    pub async fn get_conversation_history(&self) -> String {
        self.conversation_history.lock().await.clone()
    }

    /// Generate a response from a prompt
    pub async fn generate(&self, prompt: &str) -> Result<LLMResponse> {
        if !self.is_loaded().await {
            anyhow::bail!("No model is loaded. Call load_model() first.");
        }

        info!("Generating response for prompt ({}...)", &prompt[..50.min(prompt.len())]);

        let model_lock = self.model.lock().await;
        let model = &model_lock
            .as_ref()
            .context("Model not loaded despite is_loaded check")?
            .0;
        
        // Add the new user message to conversation history with proper format
        let mut history = self.conversation_history.lock().await;
        if !history.is_empty() {
            history.push_str("\n");
        }
        // Use Qwen3 chat format: <|im_start|>user\n{message}<|im_end|>
        history.push_str("<|im_start|>user\n");
        history.push_str(prompt);
        history.push_str("<|im_end|>\n<|im_start|>assistant\n");
        
        // Create context parameters for this generation
        let ctx_params = llama_cpp_2::context::params::LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(self.config.n_ctx as u32))
            .with_n_threads(self.config.n_threads as i32);
        
        // Create a new context with the full conversation history
        let mut ctx = model
            .new_context(&self.backend, ctx_params)
            .context("Failed to create context")?;
        
        // Tokenize the FULL conversation history (not just the current prompt)
        let tokens = model
            .str_to_token(&history, AddBos::Always)
            .context("Failed to tokenize conversation history")?;
        
        info!("Conversation history tokenized: {} tokens", tokens.len());
        
        // Create batch for processing
        let mut batch = LlamaBatch::new(self.config.n_ctx as usize, 1);
        
        // Add prompt tokens to batch
        for (i, token) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            batch
                .add(*token, i as i32, &[0], is_last)
                .context("Failed to add token to batch")?;
        }
        
        // Decode the prompt batch
        ctx
            .decode(&mut batch)
            .context("Failed to decode prompt batch")?;
        
        // Generate tokens
        let mut generated_text = String::new();
        let mut tokens_generated = 0;
        let max_tokens = self.config.max_tokens as usize;
        
        // Create sampler chain with configured parameters
        // This uses proper sampling (temperature, top_k, top_p, penalties) instead of greedy sampling
        // Order matters: penalties -> top_k -> top_p -> temperature -> distribution
        // See: https://github.com/ggerganov/llama.cpp/blob/master/examples/main/README.md#sampling
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(
                64,  // penalty_last_n: consider last 64 tokens for repeat detection
                self.config.repeat_penalty,  // penalty_repeat: from config (default 1.1)
                0.0, // penalty_freq: frequency penalty (0 = disabled for now)
                0.0, // penalty_present: presence penalty (0 = disabled for now)
            ),
            LlamaSampler::top_k(self.config.top_k),  // Keep only top K tokens (default 40)
            LlamaSampler::top_p(self.config.top_p, 1),  // Nucleus sampling with top_p (default 0.9), min_keep=1
            LlamaSampler::temp(self.config.temperature),  // Apply temperature (default 0.7)
            LlamaSampler::dist(0),  // Sample from distribution (seed=0 for deterministic per session)
        ]);
        
        for i in 0..max_tokens {
            // Sample next token using the configured sampler chain
            let next_token = sampler.sample(&ctx, batch.n_tokens() - 1);
            
            // Check for EOS token
            if model.is_eog_token(next_token) {
                info!("Generated {} tokens (EOS reached)", tokens_generated);
                break;
            }
            
            // Decode token to text (skip if it fails, but continue with generation)
            if let Ok(piece) = model.token_to_str(next_token, llama_cpp_2::model::Special::Tokenize) {
                generated_text.push_str(&piece);
                tokens_generated += 1;
            } else {
                warn!("Failed to decode token {}. Continuing generation...", next_token.0);
            }
            
            // Accept the token for repeat penalty tracking
            sampler.accept(next_token);
            
            // Prepare next batch with the new token
            batch.clear();
            let new_pos = tokens.len() as i32 + i as i32;
            batch
                .add(next_token, new_pos, &[0], true)
                .context("Failed to add generated token to batch")?;
            
            // Decode the new token
            ctx
                .decode(&mut batch)
                .context("Failed to decode generated token")?;
        }
        
        info!("Generated {} tokens", tokens_generated);
        
        // Add the assistant's response to conversation history with proper format
        history.push_str(&generated_text);
        history.push_str("<|im_end|>");
        drop(history); // Release the lock
        
        Ok(LLMResponse {
            text: generated_text.trim().to_string(),
            tool_calls: Self::parse_tool_calls(&generated_text),
            tokens_generated,
            done: true,
        })
    }

    /// Generate a streaming response (callback receives chunks)
    pub async fn generate_stream<F>(
        &self,
        prompt: &str,
        mut callback: F,
    ) -> Result<LLMResponse>
    where
        F: FnMut(String) -> Result<()>,
    {
        if !self.is_loaded().await {
            anyhow::bail!("No model is loaded. Call load_model() first.");
        }

        info!("Generating streaming response for prompt ({}...)", &prompt[..50.min(prompt.len())]);

        let model_lock = self.model.lock().await;
        let model = &model_lock
            .as_ref()
            .context("Model not loaded despite is_loaded check")?
            .0;
        
        // Create context for this generation
        let ctx_params = llama_cpp_2::context::params::LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(self.config.n_ctx as u32))
            .with_n_threads(self.config.n_threads as i32);
        
        let mut ctx = model.new_context(&self.backend, ctx_params)?;
        
        // Tokenize prompt
        let tokens = model
            .str_to_token(prompt, AddBos::Always)
            .context("Failed to tokenize prompt")?;
        
        let mut batch = LlamaBatch::new(self.config.n_ctx as usize, 1);
        
        // Process prompt
        for (i, token) in tokens.iter().enumerate() {
            batch
                .add(*token, i as i32, &[0], i == tokens.len() - 1)
                .context("Failed to add token")?;
        }
        
        ctx.decode(&mut batch)?;
        
        // Generate with streaming
        let mut generated_text = String::new();
        let mut tokens_generated = 0;
        let max_tokens = self.config.max_tokens as usize;
        
        for i in 0..max_tokens {
            let candidates = ctx.candidates_ith(batch.n_tokens() - 1);
            let next_token = candidates
                .into_iter()
                .max_by(|a, b| a.logit().partial_cmp(&b.logit()).unwrap())
                .map(|d| d.id())
                .context("No candidates")?;
            
            if model.is_eog_token(next_token) {
                break;
            }
            
            let piece = model.token_to_str(next_token, llama_cpp_2::model::Special::Tokenize)?;
            
            // Stream the chunk
            callback(piece.clone())?;
            
            generated_text.push_str(&piece);
            tokens_generated += 1;
            
            batch.clear();
            batch.add(next_token, tokens.len() as i32 + i as i32, &[0], true)?;
            ctx.decode(&mut batch)?;
        }
        
        let tool_calls = Self::parse_tool_calls(&generated_text);
        
        Ok(LLMResponse {
            text: generated_text,
            tool_calls,
            tokens_generated,
            done: true,
        })
    }

    /// Parse tool calls from response text (placeholder for future implementation)
    fn parse_tool_calls(_text: &str) -> Vec<ToolCall> {
        // TODO: Implement tool call detection based on JSON format
        vec![]
    }

    /// Unload model from memory
    pub async fn unload_model(&self) -> Result<()> {
        info!("Unloading model");
        let mut model_lock = self.model.lock().await;
        *model_lock = None;
        info!("Model unloaded successfully");
        Ok(())
    }

    /// Get current configuration
    pub fn config(&self) -> &LLMConfig {
        &self.config
    }

    /// Update configuration (requires reload)
    pub fn set_config(&mut self, config: LLMConfig) {
        warn!("Configuration changed. Model must be reloaded.");
        self.config = config;
    }
}

impl Drop for LLMEngine {
    fn drop(&mut self) {
        info!("LLMEngine dropping - cleanup will occur automatically");
    }
}
