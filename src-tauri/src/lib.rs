pub mod llm;
pub mod context;
pub mod mcp;
pub mod huggingface;
pub mod commands;

use llm::{LLMEngine, LLMConfig, ModelManager};
use huggingface::HuggingFaceClient;
use context::{Database, SettingsRepository, ContextManager, ConversationRepository, get_default_database_path};

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use tracing_subscriber;

// Import all commands from the commands module
use commands::*;

/// État global de l'application
pub struct AppState {
    pub llm_engine: Arc<RwLock<LLMEngine>>,
    pub model_manager: Arc<ModelManager>,
    pub hf_client: Arc<RwLock<HuggingFaceClient>>,
    pub database: Arc<Database>,
    pub settings_repo: Arc<SettingsRepository>,
    pub context_manager: Arc<RwLock<ContextManager>>,
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
            
            // Initialize Database and Settings
            // Create a new runtime for async initialization
            let runtime = tokio::runtime::Runtime::new().map_err(|e| {
                error!("Failed to create Tokio runtime: {}", e);
                e
            })?;
            
            let (database, settings_repo, context_manager) = runtime.block_on(async {
                // Get database path
                let db_url = match get_default_database_path() {
                    Ok(url) => {
                        info!("Database URL: {}", url);
                        url
                    },
                    Err(e) => {
                        error!("Failed to get database path, using in-memory: {}", e);
                        "sqlite::memory:".to_string()
                    }
                };
                
                // Create database
                let db = match Database::new(&db_url).await {
                    Ok(db) => db,
                    Err(e) => {
                        error!("Failed to create database, falling back to in-memory: {}", e);
                        Database::new("sqlite::memory:").await
                            .expect("Failed to create in-memory database")
                    }
                };
                
                // Run migrations
                if let Err(e) = db.migrate().await {
                    error!("Database migration failed: {}", e);
                }
                
                let pool = db.pool().clone();
                let settings = SettingsRepository::new(pool.clone());
                
                // Get current model or use default
                let current_model = settings.get_current_model().await
                    .unwrap_or(None)
                    .unwrap_or_else(|| "No model loaded".to_string());
                
                // Create ConversationRepository and ContextManager
                let conv_repo = ConversationRepository::new(pool);
                let ctx_manager = ContextManager::new(conv_repo, current_model);
                
                (Arc::new(db), Arc::new(settings), Arc::new(RwLock::new(ctx_manager)))
            });
            
            let app_state = Arc::new(AppState {
                llm_engine,
                model_manager,
                hf_client,
                database,
                settings_repo,
                context_manager,
            });
            
            app.manage(app_state);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initialize_llm,
            switch_model,
            send_message,
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
            hf_get_gguf_files,
            get_current_model,
            create_session,
            add_message,
            get_session,
            list_sessions,
            delete_session,
            rename_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
