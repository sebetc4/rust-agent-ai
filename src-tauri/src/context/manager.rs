/// Gestionnaire de contexte conversationnel

use super::session::{ConversationSession, Message, MessageRole};
use super::repository::ConversationRepository;
use super::models::StoredMessage;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Gestionnaire de contexte principal
pub struct ContextManager {
    repository: ConversationRepository,
    sessions_cache: Arc<RwLock<HashMap<String, ConversationSession>>>,
    active_session_id: Arc<RwLock<Option<String>>>,
    current_model: Arc<RwLock<String>>,
}

impl ContextManager {
    /// Crée un nouveau gestionnaire de contexte avec un repository
    pub fn new(repository: ConversationRepository, model_name: String) -> Self {
        info!("Initialisation du gestionnaire de contexte");
        Self {
            repository,
            sessions_cache: Arc::new(RwLock::new(HashMap::new())),
            active_session_id: Arc::new(RwLock::new(None)),
            current_model: Arc::new(RwLock::new(model_name)),
        }
    }
    
    /// Set the current model name
    pub async fn set_current_model(&self, model_name: String) {
        *self.current_model.write().await = model_name;
    }

    /// Crée une nouvelle session de conversation persistée
    pub async fn create_session(&self, title: String) -> Result<String> {
        let model_name = self.current_model.read().await.clone();
        debug!("Création d'une nouvelle session avec le modèle: {}", model_name);
        
        // Créer dans le repository
        let conversation = self.repository.create_conversation(
            &title,
            &model_name
        ).await?;
        
        let session_id = conversation.id.clone();
        
        // Créer la session en mémoire
        let session = ConversationSession::new_with_id(session_id.clone(), title);
        
        // Mettre en cache
        self.sessions_cache.write().await.insert(session_id.clone(), session);
        
        // Définir comme session active
        *self.active_session_id.write().await = Some(session_id.clone());
        
        info!("Nouvelle session créée: {}", session_id);
        Ok(session_id)
    }
    
    /// Helper: Charge une session depuis le repository vers le cache
    async fn load_session_to_cache(&self, session_id: &str) -> Result<()> {
        let conversation = self.repository.get_conversation(session_id).await?
            .ok_or_else(|| anyhow::anyhow!("Session non trouvée dans la base: {}", session_id))?;
        let messages = self.repository.get_messages(session_id).await?;
        
        let mut session = ConversationSession::new_with_id(
            conversation.id.clone(),
            conversation.title.clone()
        );
        
        // Ajouter les messages récupérés
        for stored_msg in messages {
            let role = Self::parse_role(&stored_msg.role)?;
            let msg = Message::new(role, stored_msg.content.clone());
            session.add_message(msg);
        }
        
        self.sessions_cache.write().await.insert(session_id.to_string(), session);
        Ok(())
    }
    
    /// Helper: Convertit une chaîne en MessageRole
    fn parse_role(role_str: &str) -> Result<MessageRole> {
        match role_str {
            "system" => Ok(MessageRole::System),
            "user" => Ok(MessageRole::User),
            "assistant" => Ok(MessageRole::Assistant),
            "tool" => Ok(MessageRole::Tool),
            _ => anyhow::bail!("Rôle inconnu: {}", role_str),
        }
    }

    /// Récupère une session par son ID (charge depuis DB si nécessaire)
    pub async fn get_session(&self, session_id: &str) -> Result<ConversationSession> {
        // Vérifier le cache d'abord
        {
            let sessions = self.sessions_cache.read().await;
            if let Some(session) = sessions.get(session_id) {
                return Ok(session.clone());
            }
        }
        
        // Pas en cache, charger depuis DB
        self.load_session_to_cache(session_id).await?;
        
        let sessions = self.sessions_cache.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Session non trouvée après chargement: {}", session_id))
    }

    /// Récupère la session active
    pub async fn get_active_session(&self) -> Result<ConversationSession> {
        let active_id = self.active_session_id.read().await;
        let session_id = active_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Aucune session active"))?;
        self.get_session(session_id).await
    }

    /// Ajoute un message à une session (persiste dans DB)
    pub async fn add_message(&self, session_id: &str, message: Message) -> Result<()> {
        debug!("Ajout d'un message {:?} à la session {}", message.role, session_id);
        
        // Convertir MessageRole en chaîne pour le DB
        let role_str = match message.role {
            MessageRole::System => "system",
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::Tool => "tool",
        };
        
        // Persister dans le repository
        let stored_msg = StoredMessage::new(
            session_id.to_string(),
            role_str.to_string(),
            message.content.clone(),
        );
        let _stored_message = self.repository.add_message(&stored_msg).await?;
        
        // Mettre à jour le cache - charger la session si nécessaire
        {
            let sessions = self.sessions_cache.read().await;
            if !sessions.contains_key(session_id) {
                drop(sessions); // Release read lock
                self.load_session_to_cache(session_id).await?;
            }
        }
        
        // Maintenant ajouter le message au cache
        let mut sessions = self.sessions_cache.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.add_message(message);
        }
        
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

    /// Liste toutes les sessions (charge depuis DB)
    pub async fn list_sessions(&self) -> Result<Vec<ConversationSession>> {
        let conversations = self.repository.list_conversations(100, 0).await?;
        
        let mut sessions = Vec::new();
        for conv in conversations {
            let session = ConversationSession::new_with_id(
                conv.id.clone(),
                conv.title.clone()
            );
            sessions.push(session);
        }
        
        // Tri par date de mise à jour (plus récent en premier)
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(sessions)
    }

    /// Supprime une session (DB + cache)
    pub async fn delete_session(&self, session_id: &str) -> Result<()> {
        // Supprimer du repository
        self.repository.delete_conversation(session_id).await?;
        
        // Supprimer du cache
        self.sessions_cache.write().await.remove(session_id);
        
        // Si c'était la session active, la désactiver
        let mut active_id = self.active_session_id.write().await;
        if active_id.as_ref() == Some(&session_id.to_string()) {
            *active_id = None;
        }
        
        info!("Session supprimée: {}", session_id);
        Ok(())
    }
    
    /// Renomme une session
    pub async fn rename_session(&self, session_id: &str, new_title: String) -> Result<()> {
        // Mettre à jour dans le repository
        self.repository.update_conversation_title(session_id, &new_title).await?;
        
        // Mettre à jour dans le cache si présent
        let mut sessions = self.sessions_cache.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.title = new_title.clone();
        }
        
        info!("Session {} renommée: {}", session_id, new_title);
        Ok(())
    }

    /// Définit la session active
    pub async fn set_active_session(&self, session_id: &str) -> Result<()> {
        // Vérifier que la session existe
        let sessions = self.sessions_cache.read().await;
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

#[cfg(test)]
mod tests {
    // Tests require database setup - will be implemented with integration tests
    // TODO: Add integration tests with test database
}
