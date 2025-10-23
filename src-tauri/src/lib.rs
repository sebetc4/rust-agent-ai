// Public modules for library usage
pub mod llm;
pub mod context;
pub mod mcp;

use tauri::Manager;
use llm::{LLMEngine, LLMConfig, ModelManager, ModelInfo};
use context::{ContextManager, Message, MessageRole, ConversationSession};
use mcp::MCPServer;
use mcp::tools::{create_file_reader_tool, create_file_writer_tool};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;
use tracing::{info, error};
use tracing_subscriber;

/// État global de l'application
pub struct AppState {
    llm_engine: Arc<RwLock<LLMEngine>>,
    context_manager: Arc<ContextManager>,
    mcp_server: Arc<MCPServer>,
    model_manager: Arc<ModelManager>,
}

// ===== Commandes Tauri =====
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
        config.model_path = model_path;
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

            let context_manager = Arc::new(ContextManager::new());
            let mcp_server = Arc::new(MCPServer::new(3000));

            // Enregistrer les outils additionnels
            let registry = mcp_server.tool_registry();
            tauri::async_runtime::spawn(async move {
                let mut reg = registry.write().await;
                let _ = reg.register_tool(create_file_reader_tool());
                let _ = reg.register_tool(create_file_writer_tool());
                info!("Outils MCP enregistrés");
            });

            // Démarrer le serveur MCP dans une tâche asynchrone
            let mcp_server_clone = Arc::clone(&mcp_server);
            tauri::async_runtime::spawn(async move {
                if let Err(e) = mcp_server_clone.start().await {
                    error!("Erreur du serveur MCP: {}", e);
                }
            });

            // Créer l'état global
            let app_state = Arc::new(AppState {
                llm_engine,
                context_manager,
                mcp_server,
                model_manager,
            });

            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initialize_llm,
            generate_response,
            create_session,
            add_message,
            get_session,
            list_sessions,
            delete_session,
            list_tools,
            execute_tool,
            list_models,
            delete_model,
            get_models_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
