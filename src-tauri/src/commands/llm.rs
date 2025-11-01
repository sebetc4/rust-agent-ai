/// Commandes Tauri pour la gestion du LLM

use crate::AppState;
use crate::context;
use std::sync::Arc;
use tauri::State;
use tracing::{info, error};

#[tauri::command]
pub async fn initialize_llm(
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
    
    // Persist current model to settings
    if let Err(e) = state.settings_repo.set_current_model(&model_name).await {
        error!("Failed to persist current model: {}", e);
    }
    
    Ok("Modèle chargé avec succès".to_string())
}

#[tauri::command]
pub async fn switch_model(
    state: State<'_, Arc<AppState>>,
    model_name: String,
) -> Result<String, String> {
    info!("Switching to model: {}", model_name);
    
    let models_dir = state.model_manager.models_directory();
    let model_path = models_dir.join(&model_name);
    
    if !model_path.exists() {
        return Err(format!("Model file not found: {}", model_name));
    }
    
    // Update config and load model
    {
        let engine = state.llm_engine.read().await;
        let mut config = engine.config.clone();
        config.model_path = model_path.to_string_lossy().to_string();
        drop(engine); // Release read lock
        
        let mut engine_write = state.llm_engine.write().await;
        engine_write.config = config;
        engine_write.load_model().await.map_err(|e| e.to_string())?;
    }
    
    // Persist current model to settings
    if let Err(e) = state.settings_repo.set_current_model(&model_name).await {
        error!("Failed to persist current model: {}", e);
    }
    
    info!("Successfully switched to model: {}", model_name);
    Ok(format!("Switched to model: {}", model_name))
}

#[tauri::command]
pub async fn send_message(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    content: String,
) -> Result<String, String> {
    info!("Envoi de message pour session: {}", session_id);
    
    // 1. Ajouter le message utilisateur
    let user_message = context::Message::new(context::MessageRole::User, content.clone());
    {
        let context_manager = state.context_manager.read().await;
        context_manager.add_message(&session_id, user_message).await
            .map_err(|e| format!("Erreur ajout message: {}", e))?;
    }
    
    // 2. Récupérer le contexte complet de la session
    let session = {
        let context_manager = state.context_manager.read().await;
        context_manager.get_session(&session_id).await
            .map_err(|e| format!("Erreur récupération session: {}", e))?
    };
    
    // 3. Construire le contexte pour le LLM
    let mut context_str = String::new();
    for message in &session.messages {
        let role = match message.role {
            context::MessageRole::System => "System",
            context::MessageRole::User => "User",
            context::MessageRole::Assistant => "Assistant",
            context::MessageRole::Tool => "Tool",
        };
        context_str.push_str(&format!("{}: {}\n", role, message.content));
    }
    context_str.push_str("Assistant: ");
    
    // 4. Générer la réponse avec le LLM
    let response = {
        let engine = state.llm_engine.read().await;
        engine.generate(&context_str).await
            .map_err(|e| format!("Erreur génération LLM: {}", e))?
    };
    
    // 5. Ajouter la réponse de l'assistant
    let assistant_message = context::Message::new(context::MessageRole::Assistant, response.text.clone());
    {
        let context_manager = state.context_manager.read().await;
        context_manager.add_message(&session_id, assistant_message).await
            .map_err(|e| format!("Erreur ajout réponse: {}", e))?;
    }
    
    info!("Message envoyé et réponse générée pour session {}", session_id);
    Ok(response.text)
}

#[tauri::command]
pub async fn generate_response(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    prompt: String,
) -> Result<String, String> {
    info!("Génération de réponse pour session: {}", session_id);
    
    // Get the session with full context
    let context_manager = state.context_manager.read().await;
    let session = context_manager.get_session(&session_id).await
        .map_err(|e| e.to_string())?;
    
    // Build context from message history
    let mut context_str = String::new();
    for message in &session.messages {
        let role = match message.role {
            context::MessageRole::System => "System",
            context::MessageRole::User => "User",
            context::MessageRole::Assistant => "Assistant",
            context::MessageRole::Tool => "Tool",
        };
        context_str.push_str(&format!("{}: {}\n", role, message.content));
    }
    
    // Add current user message to context
    context_str.push_str(&format!("User: {}\n", prompt));
    
    // Generate response with full context
    let engine = state.llm_engine.read().await;
    let response = engine.generate(&context_str).await.map_err(|e| e.to_string())?;
    
    Ok(response.text)
}

#[tauri::command]
pub async fn get_current_model(
    state: State<'_, Arc<AppState>>,
) -> Result<Option<String>, String> {
    state.settings_repo
        .get_current_model()
        .await
        .map_err(|e| e.to_string())
}
