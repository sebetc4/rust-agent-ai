/// Structures pour les sessions de conversation et les messages

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Rôle d'un message dans la conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Message dans une conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Message {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role,
            content,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn system(content: String) -> Self {
        Self::new(MessageRole::System, content)
    }

    pub fn user(content: String) -> Self {
        Self::new(MessageRole::User, content)
    }

    pub fn assistant(content: String) -> Self {
        Self::new(MessageRole::Assistant, content)
    }

    pub fn tool(content: String) -> Self {
        Self::new(MessageRole::Tool, content)
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Résumé d'une session (sans les messages) pour l'affichage dans la liste
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Session de conversation complète avec tous les messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSession {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ConversationSession {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            created_at: now,
            updated_at: now,
            messages: vec![],
            metadata: HashMap::new(),
        }
    }
    
    pub fn new_with_id(id: String, title: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            created_at: now,
            updated_at: now,
            messages: vec![],
            metadata: HashMap::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn get_context_window(&self, _max_tokens: usize) -> Vec<Message> {
        // TODO: Implémenter une vraie gestion de la fenêtre de contexte
        // basée sur le nombre de tokens
        let max_messages = 20; // Temporaire
        let start = self.messages.len().saturating_sub(max_messages);
        self.messages[start..].to_vec()
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::user("Hello".to_string());
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_session_creation() {
        let session = ConversationSession::new("Test".to_string());
        assert_eq!(session.title, "Test");
        assert!(session.messages.is_empty());
    }
}
