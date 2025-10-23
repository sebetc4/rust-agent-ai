/// Gestionnaire de contexte conversationnel

use super::session::{ConversationSession, Message};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Gestionnaire de contexte principal
pub struct ContextManager {
    sessions: Arc<RwLock<HashMap<String, ConversationSession>>>,
    active_session_id: Arc<RwLock<Option<String>>>,
}

impl ContextManager {
    /// Crée un nouveau gestionnaire de contexte
    pub fn new() -> Self {
        info!("Initialisation du gestionnaire de contexte");
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            active_session_id: Arc::new(RwLock::new(None)),
        }
    }

    /// Crée une nouvelle session de conversation
    pub async fn create_session(&self, title: String) -> Result<String> {
        let session = ConversationSession::new(title);
        let session_id = session.id.clone();
        
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session);
        
        // Définir comme session active
        *self.active_session_id.write().await = Some(session_id.clone());
        
        info!("Nouvelle session créée: {}", session_id);
        Ok(session_id)
    }

    /// Récupère une session par son ID
    pub async fn get_session(&self, session_id: &str) -> Result<ConversationSession> {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Session non trouvée: {}", session_id))
    }

    /// Récupère la session active
    pub async fn get_active_session(&self) -> Result<ConversationSession> {
        let active_id = self.active_session_id.read().await;
        let session_id = active_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Aucune session active"))?;
        self.get_session(session_id).await
    }

    /// Ajoute un message à une session
    pub async fn add_message(&self, session_id: &str, message: Message) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session non trouvée: {}", session_id))?;
        
        debug!("Ajout d'un message {:?} à la session {}", message.role, session_id);
        session.add_message(message);
        Ok(())
    }

    /// Ajoute un message à la session active
    pub async fn add_message_to_active(&self, message: Message) -> Result<()> {
        let active_id = self.active_session_id.read().await;
        let session_id = active_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Aucune session active"))?
            .clone();
        drop(active_id);
        
        self.add_message(&session_id, message).await
    }

    /// Liste toutes les sessions
    pub async fn list_sessions(&self) -> Vec<ConversationSession> {
        let sessions = self.sessions.read().await;
        let mut sessions_vec: Vec<_> = sessions.values().cloned().collect();
        sessions_vec.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        sessions_vec
    }

    /// Supprime une session
    pub async fn delete_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions
            .remove(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session non trouvée: {}", session_id))?;
        
        // Si c'était la session active, la désactiver
        let mut active_id = self.active_session_id.write().await;
        if active_id.as_ref() == Some(&session_id.to_string()) {
            *active_id = None;
        }
        
        info!("Session supprimée: {}", session_id);
        Ok(())
    }

    /// Définit la session active
    pub async fn set_active_session(&self, session_id: &str) -> Result<()> {
        // Vérifier que la session existe
        let sessions = self.sessions.read().await;
        if !sessions.contains_key(session_id) {
            anyhow::bail!("Session non trouvée: {}", session_id);
        }
        drop(sessions);

        *self.active_session_id.write().await = Some(session_id.to_string());
        info!("Session active définie: {}", session_id);
        Ok(())
    }

    /// Sauvegarde les sessions (à implémenter avec SQLite)
    pub async fn save_to_disk(&self) -> Result<()> {
        // TODO: Implémenter la persistance avec SQLite
        info!("Sauvegarde des sessions (à implémenter)");
        Ok(())
    }

    /// Charge les sessions depuis le disque
    pub async fn load_from_disk(&self) -> Result<()> {
        // TODO: Implémenter le chargement depuis SQLite
        info!("Chargement des sessions (à implémenter)");
        Ok(())
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::session::Message;

    #[tokio::test]
    async fn test_create_session() {
        let manager = ContextManager::new();
        let session_id = manager.create_session("Test".to_string()).await.unwrap();
        assert!(!session_id.is_empty());
    }

    #[tokio::test]
    async fn test_add_message() {
        let manager = ContextManager::new();
        let session_id = manager.create_session("Test".to_string()).await.unwrap();
        
        let message = Message::user("Hello".to_string());
        manager.add_message(&session_id, message).await.unwrap();
        
        let session = manager.get_session(&session_id).await.unwrap();
        assert_eq!(session.messages.len(), 1);
    }

    #[tokio::test]
    async fn test_active_session() {
        let manager = ContextManager::new();
        let session_id = manager.create_session("Test".to_string()).await.unwrap();
        
        let active = manager.get_active_session().await.unwrap();
        assert_eq!(active.id, session_id);
    }
}
