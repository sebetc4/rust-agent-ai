/// Commandes Tauri pour la gestion des modèles

use crate::AppState;
use crate::llm::{LLMEngine, ModelInfo};
use std::sync::Arc;
use tauri::State;
use tracing::info;

#[tauri::command]
pub async fn list_models(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<ModelInfo>, String> {
    info!("Listing available models");
    
    state.model_manager
        .list_models()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_model(
    state: State<'_, Arc<AppState>>,
    model_name: String,
) -> Result<String, String> {
    info!("Deleting model: {}", model_name);
    
    state.model_manager
        .delete_model(&model_name)
        .map_err(|e| e.to_string())?;
    
    Ok("Model deleted successfully".to_string())
}

#[tauri::command]
pub async fn get_models_directory(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let path = state.model_manager.models_directory();
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_gpu_info(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let engine = state.llm_engine.read().await;
    Ok(engine.gpu_info())
}

#[tauri::command]
pub async fn detect_gpu() -> Result<(bool, String), String> {
    let (available, info) = LLMEngine::detect_gpu_config();
    Ok((available, info))
}

#[tauri::command]
pub async fn update_gpu_settings(
    state: State<'_, Arc<AppState>>,
    use_gpu: bool,
    n_gpu_layers: Option<u32>,
) -> Result<String, String> {
    info!("Updating GPU settings: use_gpu={}, n_gpu_layers={:?}", use_gpu, n_gpu_layers);
    
    let mut engine = state.llm_engine.write().await;
    engine.config.use_gpu = use_gpu;
    
    if let Some(layers) = n_gpu_layers {
        engine.config.n_gpu_layers = layers;
    }
    
    Ok("GPU settings updated successfully".to_string())
}
