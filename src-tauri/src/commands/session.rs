/// Commandes Tauri pour la gestion des sessions de conversation

use crate::AppState;
use crate::context::{ConversationSession, SessionSummary, Message, MessageRole};
use std::sync::Arc;
use tauri::State;
use tracing::info;

#[tauri::command]
pub async fn create_session(
    state: State<'_, Arc<AppState>>,
    title: String,
) -> Result<ConversationSession, String> {
    info!("Création d'une nouvelle session: {}", title);
    
    let session_id = state.context_manager
        .write()
        .await
        .create_session(title)
        .await
        .map_err(|e| e.to_string())?;
    
    // Récupérer la session complète pour la retourner au frontend
    state.context_manager
        .read()
        .await
        .get_session(&session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_message(
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
        .write()
        .await
        .add_message(&session_id, message)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
) -> Result<ConversationSession, String> {
    state.context_manager
        .read()
        .await
        .get_session(&session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_sessions(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<SessionSummary>, String> {
    state.context_manager
        .read()
        .await
        .list_sessions()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
) -> Result<(), String> {
    state.context_manager
        .write()
        .await
        .delete_session(&session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rename_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    new_title: String,
) -> Result<(), String> {
    state.context_manager
        .write()
        .await
        .rename_session(&session_id, new_title)
        .await
        .map_err(|e| e.to_string())
}
