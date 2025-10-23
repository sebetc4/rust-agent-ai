/// Module LLM - Gestion du moteur d'inférence local

pub mod config;
pub mod engine;
pub mod model_manager;

#[cfg(test)]
mod tests;

pub use engine::{LLMEngine, LLMResponse, ToolCall};
pub use config::LLMConfig;
pub use model_manager::{ModelManager, ModelInfo};
