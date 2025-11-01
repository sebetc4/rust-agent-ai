use crate::AppState;
use crate::context;
use std::sync::Arc;
use tauri::State;
use tracing::{info, error};

#[tauri::command]
pub async fn initialize_llm(
    state: State<'_, Arc<AppState>>,
) -> Result<String, String> {
    let model_to_load = match state.settings_repo.get_current_model().await {
        Ok(Some(saved_model)) => {
            info!("Loading last used model: {}", saved_model);
            saved_model
        }
        Ok(None) => {
            return Err("No previous model found in settings. Please select a model first.".to_string());
        }
        Err(e) => {
            return Err(format!("Failed to retrieve saved model: {}", e));
        }
    };
    
    info!("Initializing LLM with model: {}", model_to_load);
    
    // Check if model exists
    if !state.model_manager.model_exists(&model_to_load) {
        return Err(format!("Model file not found: {}. Please ensure the model is in the models directory.", model_to_load));
    }
    
    // Get full path to model
    let model_path = state.model_manager.get_model_path(&model_to_load);
    
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
    
    // Return the loaded model name
    Ok(model_to_load)
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
    info!("Sending message for session: {}", session_id);
    
    // 1. Add user message
    let user_message = context::Message::new(context::MessageRole::User, content.clone());
    {
        let context_manager = state.context_manager.read().await;
        context_manager.add_message(&session_id, user_message).await
            .map_err(|e| format!("Error adding message: {}", e))?;
    }
    
    // 2. Get complete session context
    let session = {
        let context_manager = state.context_manager.read().await;
        context_manager.get_session(&session_id).await
            .map_err(|e| format!("Error retrieving session: {}", e))?
    };
    
    // 3. Build context for LLM
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
    
    // 4. Generate response with LLM
    let response = {
        let engine = state.llm_engine.read().await;
        engine.generate(&context_str).await
            .map_err(|e| format!("LLM generation error: {}", e))?
    };
    
    // 5. Add assistant response
    let assistant_message = context::Message::new(context::MessageRole::Assistant, response.text.clone());
    {
        let context_manager = state.context_manager.read().await;
        context_manager.add_message(&session_id, assistant_message).await
            .map_err(|e| format!("Error adding response: {}", e))?;
    }
    
    info!("Message sent and response generated for session {}", session_id);
    Ok(response.text)
}

#[tauri::command]
pub async fn generate_response(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    prompt: String,
) -> Result<String, String> {
    info!("Generating response for session: {}", session_id);
    
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
