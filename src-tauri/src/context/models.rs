/// Data models for conversation persistence

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A conversation represents a single chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub model_name: String,
}

/// A message within a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: Option<i64>,
    pub conversation_id: String,
    pub role: String,  // "user", "assistant", "system"
    pub content: String,
    pub tokens: Option<i32>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(title: String, model_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            created_at: now,
            updated_at: now,
            model_name,
        }
    }
}

impl StoredMessage {
    pub fn new(conversation_id: String, role: String, content: String) -> Self {
        Self {
            id: None,
            conversation_id,
            role,
            content,
            tokens: None,
            created_at: Utc::now(),
        }
    }
    
    pub fn with_tokens(mut self, tokens: i32) -> Self {
        self.tokens = Some(tokens);
        self
    }
}
