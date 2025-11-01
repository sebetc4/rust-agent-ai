/// Commandes Tauri pour l'intégration HuggingFace

use crate::AppState;
use crate::huggingface::{HFModelInfo, ModelSearchParams};
use std::sync::Arc;
use tauri::{AppHandle, State, Emitter};
use tracing::{info, error};

#[tauri::command]
pub async fn hf_search_models(
    state: State<'_, Arc<AppState>>,
    search_query: Option<String>,
    author: Option<String>,
    task: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::huggingface::Model>, String> {
    info!("Searching HuggingFace models");
    
    let mut params = ModelSearchParams::new();
    
    if let Some(query) = search_query {
        params = params.search(&query);
    }
    if let Some(author) = author {
        params = params.author(&author);
    }
    if let Some(task) = task {
        params = params.task(&task);
    }
    if let Some(limit) = limit {
        params = params.limit(limit);
    } else {
        params = params.limit(20); // Default limit
    }
    
    let client = state.hf_client.read().await;
    client.search_models(params)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn hf_get_model_info(
    state: State<'_, Arc<AppState>>,
    repo_id: String,
) -> Result<HFModelInfo, String> {
    info!("Fetching HuggingFace model info: {}", repo_id);
    
    let client = state.hf_client.read().await;
    client.get_model_info(&repo_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn hf_download_model(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
    repo_id: String,
    filename: String,
    revision: Option<String>,
) -> Result<String, String> {
    info!("Downloading {} from {}", filename, repo_id);
    
    let models_dir = state.model_manager.models_directory();
    let output_path = models_dir.join(&filename);
    
    let client = state.hf_client.read().await;
    
    // Use download_file_with_progress to emit progress events
    let result_path = client.download_file_with_progress(
        &repo_id,
        &filename,
        revision.as_deref(),
        output_path,
        |downloaded, total| {
            let progress = if let Some(total) = total {
                (downloaded as f64 / total as f64 * 100.0) as u32
            } else {
                0
            };
            
            // Emit progress event
            let _ = app.emit("download-progress", serde_json::json!({
                "repo_id": repo_id,
                "filename": filename,
                "downloaded": downloaded,
                "total": total,
                "progress": progress,
            }));
        },
    )
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(result_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn hf_set_token(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> Result<String, String> {
    info!("Setting HuggingFace token");
    
    let mut client = state.hf_client.write().await;
    client.set_token(token);
    
    Ok("Token set successfully".to_string())
}

#[tauri::command]
pub async fn hf_discover_gguf_models(
    state: State<'_, Arc<AppState>>,
    search_query: Option<String>,
    author: Option<String>,
    task: Option<String>,
    sort: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<crate::huggingface::GGUFModelMetadata>, String> {
    info!("Discovering GGUF models from HuggingFace");
    
    let mut params = ModelSearchParams::new();
    
    if let Some(query) = search_query {
        params = params.search(&query);
    }
    if let Some(author) = author {
        params = params.author(&author);
    }
    if let Some(task) = task {
        params = params.task(&task);
    }
    if let Some(sort) = sort {
        params.sort = Some(sort);
    }
    if let Some(limit) = limit {
        params = params.limit(limit);
    } else {
        params = params.limit(20); // Default limit
    }
    
    let client = state.hf_client.read().await;
    client.discover_gguf_models(params)
        .await
        .map_err(|e| {
            error!("Failed to discover GGUF models: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn hf_get_gguf_files(
    state: State<'_, Arc<AppState>>,
    repo_id: String,
) -> Result<Vec<crate::huggingface::GGUFFile>, String> {
    info!("Getting GGUF files for {}", repo_id);
    
    let client = state.hf_client.read().await;
    client.get_gguf_files(&repo_id)
        .await
        .map_err(|e| {
            error!("Failed to get GGUF files for {}: {}", repo_id, e);
            e.to_string()
        })
}
