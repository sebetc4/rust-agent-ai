// Public modules for library usage
pub mod llm;
pub mod context;
pub mod mcp;
pub mod huggingface;

use llm::{LLMEngine, LLMConfig, ModelManager, ModelInfo};
use huggingface::{HuggingFaceClient, ModelSearchParams, HFModelInfo};

use tauri::Manager;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;
use tracing::{info, error};
use tracing_subscriber;

/// État global de l'application
pub struct AppState {
    pub llm_engine: Arc<RwLock<LLMEngine>>,
    pub model_manager: Arc<ModelManager>,
    pub hf_client: Arc<RwLock<HuggingFaceClient>>,
}// ===== Commandes Tauri =====
#[tauri::command]
async fn initialize_llm(
    state: State<'_, Arc<AppState>>,
    model_name: String,
) -> Result<String, String> {
    info!("Chargement du modèle LLM: {}", model_name);
    
    // Check if model exists
    if !state.model_manager.model_exists(&model_name) {
        return Err(format!("Model file not found: {}. Please ensure the model is in the models directory.", model_name));
    }
    
    // Get full path to model
    let model_path = state.model_manager.get_model_path(&model_name);
    
    // Update the model path in the existing engine
    let engine = state.llm_engine.read().await;
    
    // Update config and load model
    {
        let mut config = engine.config.clone();
        config.model_path = model_path.to_string_lossy().to_string();
        drop(engine); // Release read lock
        
        let mut engine_write = state.llm_engine.write().await;
        engine_write.config = config;
        engine_write.load_model().await.map_err(|e| e.to_string())?;
    }
    
    Ok("Modèle chargé avec succès".to_string())
}

#[tauri::command]
async fn generate_response(
    state: State<'_, Arc<AppState>>,
    prompt: String,
) -> Result<String, String> {
    info!("Génération de réponse");
    
    let engine = state.llm_engine.read().await;
    let response = engine.generate(&prompt).await.map_err(|e| e.to_string())?;
    
    Ok(response.text)
}

/*
#[tauri::command]
async fn create_session(
    state: State<'_, Arc<AppState>>,
    title: String,
) -> Result<String, String> {
    info!("Création d'une nouvelle session: {}", title);
    
    state.context_manager
        .create_session(title)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_message(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    role: String,
    content: String,
) -> Result<(), String> {
    let message_role = match role.as_str() {
        "system" => MessageRole::System,
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        "tool" => MessageRole::Tool,
        _ => return Err("Rôle de message invalide".to_string()),
    };
    
    let message = Message::new(message_role, content);
    
    state.context_manager
        .add_message(&session_id, message)
        .await
        .map_err(|e| e.to_string())
}
*/

/*
#[tauri::command]
async fn get_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
) -> Result<ConversationSession, String> {
    state.context_manager
        .get_session(&session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_sessions(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<ConversationSession>, String> {
    Ok(state.context_manager.list_sessions().await)
}

#[tauri::command]
async fn delete_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
) -> Result<(), String> {
    state.context_manager
        .delete_session(&session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_tools(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<String>, String> {
    let registry_lock = state.mcp_server.tool_registry();
    let registry = registry_lock.read().await;
    let tools = registry.list_tools();
    Ok(tools.iter().map(|t| t.name.clone()).collect())
}
*/

/*
#[tauri::command]
async fn execute_tool(
    state: State<'_, Arc<AppState>>,
    tool_name: String,
    arguments: serde_json::Value,
) -> Result<String, String> {
    info!("Exécution de l'outil: {}", tool_name);
    
    let registry_lock = state.mcp_server.tool_registry();
    let registry = registry_lock.read().await;
    registry
        .execute_tool(&tool_name, arguments)
        .await
        .map_err(|e| e.to_string())
}
*/

#[tauri::command]
async fn list_models(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<ModelInfo>, String> {
    info!("Listing available models");
    
    state.model_manager
        .list_models()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_model(
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
async fn get_models_directory(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let path = state.model_manager.models_directory();
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
async fn get_gpu_info(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let engine = state.llm_engine.read().await;
    Ok(engine.gpu_info())
}

#[tauri::command]
async fn detect_gpu(
) -> Result<(bool, String), String> {
    let (available, info) = LLMEngine::detect_gpu_config();
    Ok((available, info))
}

#[tauri::command]
async fn update_gpu_settings(
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

// ===== Commandes Hugging Face =====

#[tauri::command]
async fn hf_search_models(
    state: State<'_, Arc<AppState>>,
    search_query: Option<String>,
    author: Option<String>,
    task: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<huggingface::Model>, String> {
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
async fn hf_get_model_info(
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
async fn hf_download_model(
    state: State<'_, Arc<AppState>>,
    repo_id: String,
    filename: String,
    revision: Option<String>,
) -> Result<String, String> {
    info!("Downloading {} from {}", filename, repo_id);
    
    let models_dir = state.model_manager.models_directory();
    let output_path = models_dir.join(&filename);
    
    let client = state.hf_client.read().await;
    let result_path = client.download_file(
        &repo_id,
        &filename,
        revision.as_deref(),
        output_path,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(result_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn hf_set_token(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> Result<String, String> {
    info!("Setting HuggingFace token");
    
    let mut client = state.hf_client.write().await;
    client.set_token(token);
    
    Ok("Token set successfully".to_string())
}

#[tauri::command]
async fn hf_discover_gguf_models(
    state: State<'_, Arc<AppState>>,
    search_query: Option<String>,
    author: Option<String>,
    task: Option<String>,
    sort: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<huggingface::GGUFModelInfo>, String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("info,agents_rs=debug")
        .init();

    info!("Démarrage de agents-rs");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialiser les composants backend
            let model_manager = Arc::new(ModelManager::new().map_err(|e| {
                error!("Failed to initialize model manager: {}", e);
                e
            })?);
            
            let llm_config = LLMConfig::default();
            let llm_engine = match LLMEngine::new(llm_config) {
                Ok(engine) => Arc::new(RwLock::new(engine)),
                Err(e) => {
                    error!("Erreur lors de l'initialisation du moteur LLM: {}", e);
                    return Err(e.into());
                }
            };

            // Initialize HuggingFace client
            let hf_client = Arc::new(RwLock::new(
                HuggingFaceClient::new().map_err(|e| {
                    error!("Failed to initialize HuggingFace client: {}", e);
                    e
                })?
            ));
            
            // Créer l'état global
            let app_state = Arc::new(AppState {
                llm_engine,
                model_manager,
                hf_client,
            });
            
            app.manage(app_state);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initialize_llm,
            generate_response,
            list_models,
            delete_model,
            get_models_directory,
            get_gpu_info,
            detect_gpu,
            update_gpu_settings,
            hf_search_models,
            hf_get_model_info,
            hf_download_model,
            hf_set_token,
            hf_discover_gguf_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
