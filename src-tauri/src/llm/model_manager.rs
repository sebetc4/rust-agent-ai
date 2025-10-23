/// Model manager for handling model files
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub is_loaded: bool,
}

pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    pub fn new() -> Result<Self> {
        // Determine models directory based on platform
        let models_dir = get_models_directory()?;
        
        info!("ModelManager initialized with directory: {:?}", models_dir);
        info!("Models directory exists: {}", models_dir.exists());
        
        // Create models directory if it doesn't exist
        if !models_dir.exists() {
            fs::create_dir_all(&models_dir)
                .with_context(|| format!("Failed to create models directory: {:?}", models_dir))?;
            info!("Created models directory: {:?}", models_dir);
        }

        Ok(Self { models_dir })
    }

    /// Get the absolute path to a model file
    pub fn get_model_path(&self, model_name: &str) -> PathBuf {
        self.models_dir.join(model_name)
    }

    /// List all available model files
    pub fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models = Vec::new();

        if !self.models_dir.exists() {
            return Ok(models);
        }

        let entries = fs::read_dir(&self.models_dir)
            .with_context(|| format!("Failed to read models directory: {:?}", self.models_dir))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "gguf" {
                        let file_name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let name = path.file_stem()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let size_bytes = entry.metadata()?.len();

                        models.push(ModelInfo {
                            name,
                            file_name,
                            size_bytes,
                            is_loaded: false,
                        });
                    }
                }
            }
        }

        models.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(models)
    }

    /// Check if a model file exists
    pub fn model_exists(&self, model_name: &str) -> bool {
        let path = self.get_model_path(model_name);
        let exists = path.exists() && path.is_file();
        info!("Checking model '{}' at path: {:?} - exists: {}", model_name, path, exists);
        exists
    }

    /// Get the models directory path
    pub fn models_directory(&self) -> &Path {
        &self.models_dir
    }

    /// Delete a model file
    pub fn delete_model(&self, model_name: &str) -> Result<()> {
        let path = self.get_model_path(model_name);
        
        if !path.exists() {
            return Err(anyhow::anyhow!("Model file not found: {}", model_name));
        }

        fs::remove_file(&path)
            .with_context(|| format!("Failed to delete model file: {:?}", path))?;
        
        info!("Deleted model: {}", model_name);
        Ok(())
    }
}

/// Get the appropriate models directory for the current platform
fn get_models_directory() -> Result<PathBuf> {
    // Try to use the models directory in the current working directory first
    let cwd_models = std::env::current_dir()?.join("models");
    info!("Checking CWD models directory: {:?} - exists: {}", cwd_models, cwd_models.exists());
    if cwd_models.exists() {
        return Ok(cwd_models);
    }

    // Try parent directory (for when running from src-tauri/)
    if let Ok(current) = std::env::current_dir() {
        if let Some(parent) = current.parent() {
            let parent_models = parent.join("models");
            info!("Checking parent models directory: {:?} - exists: {}", parent_models, parent_models.exists());
            if parent_models.exists() {
                return Ok(parent_models);
            }
        }
    }

    // For development/portable mode, use models directory relative to executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let portable_models = exe_dir.join("models");
            info!("Checking portable models directory: {:?} - exists: {}", portable_models, portable_models.exists());
            if portable_models.exists() {
                return Ok(portable_models);
            }
        }
    }

    // Fallback to user data directory
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let models_dir = PathBuf::from(home)
                .join(".local/share/agents-rs/models");
            return Ok(models_dir);
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let models_dir = PathBuf::from(appdata)
                .join("agents-rs/models");
            return Ok(models_dir);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let models_dir = PathBuf::from(home)
                .join("Library/Application Support/agents-rs/models");
            return Ok(models_dir);
        }
    }

    // Final fallback
    Ok(PathBuf::from("models"))
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            error!("Failed to create ModelManager: {}", e);
            // Create with fallback directory
            Self {
                models_dir: PathBuf::from("models"),
            }
        })
    }
}