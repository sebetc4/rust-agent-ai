/// Repository pattern for conversation and message persistence

use super::models::{Conversation, StoredMessage};
use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use tracing::{debug, info};

pub struct ConversationRepository {
    pool: SqlitePool,
}

impl ConversationRepository {
    /// Create a new repository instance
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    // ==================== Conversation CRUD ====================
    
    /// Create a new conversation
    pub async fn create_conversation(&self, title: &str, model_name: &str) -> Result<Conversation> {
        let conversation = Conversation::new(title.to_string(), model_name.to_string());
        
        sqlx::query(
            r#"
            INSERT INTO conversations (id, title, created_at, updated_at, model_name)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&conversation.id)
        .bind(&conversation.title)
        .bind(conversation.created_at.timestamp())
        .bind(conversation.updated_at.timestamp())
        .bind(&conversation.model_name)
        .execute(&self.pool)
        .await
        .context("Failed to create conversation")?;
        
        info!("Created conversation: {} ({})", conversation.title, conversation.id);
        
        Ok(conversation)
    }
    
    /// Get a conversation by ID
    pub async fn get_conversation(&self, id: &str) -> Result<Option<Conversation>> {
        let row = sqlx::query(
            r#"
            SELECT id, title, created_at, updated_at, model_name
            FROM conversations
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch conversation")?;
        
        if let Some(row) = row {
            let created_timestamp: i64 = row.get("created_at");
            let updated_timestamp: i64 = row.get("updated_at");
            
            Ok(Some(Conversation {
                id: row.get("id"),
                title: row.get("title"),
                created_at: DateTime::from_timestamp(created_timestamp, 0)
                    .unwrap_or_else(|| Utc::now()),
                updated_at: DateTime::from_timestamp(updated_timestamp, 0)
                    .unwrap_or_else(|| Utc::now()),
                model_name: row.get("model_name"),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// List all conversations (most recent first)
    pub async fn list_conversations(&self, limit: i32, offset: i32) -> Result<Vec<Conversation>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, created_at, updated_at, model_name
            FROM conversations
            ORDER BY updated_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .context("Failed to list conversations")?;
        
        let conversations: Vec<Conversation> = rows
            .into_iter()
            .map(|row| {
                let created_timestamp: i64 = row.get("created_at");
                let updated_timestamp: i64 = row.get("updated_at");
                Conversation {
                    id: row.get("id"),
                    title: row.get("title"),
                    created_at: DateTime::from_timestamp(created_timestamp, 0)
                        .unwrap_or_else(|| Utc::now()),
                    updated_at: DateTime::from_timestamp(updated_timestamp, 0)
                        .unwrap_or_else(|| Utc::now()),
                    model_name: row.get("model_name"),
                }
            })
            .collect();
        
        debug!("Listed {} conversations", conversations.len());
        
        Ok(conversations)
    }
    
    /// Update conversation's updated_at timestamp
    pub async fn touch_conversation(&self, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE conversations
            SET updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(Utc::now().timestamp())
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to update conversation timestamp")?;
        
        Ok(())
    }
    
    /// Update conversation title
    pub async fn update_conversation_title(&self, id: &str, new_title: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE conversations
            SET title = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(new_title)
        .bind(Utc::now().timestamp())
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to update conversation title")?;
        
        info!("Updated conversation {} title to: {}", id, new_title);
        
        Ok(())
    }
    
    /// Delete a conversation and all its messages
    pub async fn delete_conversation(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM conversations WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to delete conversation")?;
        
        info!("Deleted conversation: {}", id);
        
        Ok(())
    }
    
    /// Count total conversations
    pub async fn count_conversations(&self) -> Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM conversations")
            .fetch_one(&self.pool)
            .await
            .context("Failed to count conversations")?;
        
        Ok(count.0)
    }
    
    // ==================== Message CRUD ====================
    
    /// Add a message to a conversation
    pub async fn add_message(&self, message: &StoredMessage) -> Result<StoredMessage> {
        let result = sqlx::query(
            r#"
            INSERT INTO messages (conversation_id, role, content, tokens, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&message.conversation_id)
        .bind(&message.role)
        .bind(&message.content)
        .bind(message.tokens)
        .bind(message.created_at.timestamp())
        .execute(&self.pool)
        .await
        .context("Failed to add message")?;
        
        // Update conversation's updated_at
        self.touch_conversation(&message.conversation_id).await?;
        
        let mut saved_message = message.clone();
        saved_message.id = Some(result.last_insert_rowid());
        
        debug!("Added message to conversation {}: {} bytes", 
               message.conversation_id, message.content.len());
        
        Ok(saved_message)
    }
    
    /// Get all messages for a conversation
    pub async fn get_messages(&self, conversation_id: &str) -> Result<Vec<StoredMessage>> {
        let rows = sqlx::query(
            r#"
            SELECT id, conversation_id, role, content, tokens, created_at
            FROM messages
            WHERE conversation_id = ?
            ORDER BY created_at ASC
            "#,
        )
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch messages")?;
        
        let messages: Vec<StoredMessage> = rows
            .into_iter()
            .map(|row| {
                let created_timestamp: i64 = row.get("created_at");
                StoredMessage {
                    id: Some(row.get("id")),
                    conversation_id: row.get("conversation_id"),
                    role: row.get("role"),
                    content: row.get("content"),
                    tokens: row.get("tokens"),
                    created_at: DateTime::from_timestamp(created_timestamp, 0)
                        .unwrap_or_else(|| Utc::now()),
                }
            })
            .collect();
        
        debug!("Retrieved {} messages for conversation {}", 
               messages.len(), conversation_id);
        
        Ok(messages)
    }
    
    /// Get the last N messages from a conversation
    pub async fn get_last_n_messages(&self, conversation_id: &str, n: i32) -> Result<Vec<StoredMessage>> {
        let rows = sqlx::query(
            r#"
            SELECT id, conversation_id, role, content, tokens, created_at
            FROM messages
            WHERE conversation_id = ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(conversation_id)
        .bind(n)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch last messages")?;
        
        let mut messages: Vec<StoredMessage> = rows
            .into_iter()
            .map(|row| {
                let created_timestamp: i64 = row.get("created_at");
                StoredMessage {
                    id: Some(row.get("id")),
                    conversation_id: row.get("conversation_id"),
                    role: row.get("role"),
                    content: row.get("content"),
                    tokens: row.get("tokens"),
                    created_at: DateTime::from_timestamp(created_timestamp, 0)
                        .unwrap_or_else(|| Utc::now()),
                }
            })
            .collect();
        
        // Reverse to get chronological order
        messages.reverse();
        
        Ok(messages)
    }
    
    /// Delete old messages, keeping only the last N
    pub async fn delete_old_messages(&self, conversation_id: &str, keep_last: i32) -> Result<usize> {
        let result = sqlx::query(
            r#"
            DELETE FROM messages
            WHERE conversation_id = ?
            AND id NOT IN (
                SELECT id FROM messages
                WHERE conversation_id = ?
                ORDER BY created_at DESC
                LIMIT ?
            )
            "#,
        )
        .bind(conversation_id)
        .bind(conversation_id)
        .bind(keep_last)
        .execute(&self.pool)
        .await
        .context("Failed to delete old messages")?;
        
        let deleted = result.rows_affected() as usize;
        
        if deleted > 0 {
            info!("Deleted {} old messages from conversation {}", deleted, conversation_id);
        }
        
        Ok(deleted)
    }
    
    /// Count messages in a conversation
    pub async fn count_messages(&self, conversation_id: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE conversation_id = ?"
        )
        .bind(conversation_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to count messages")?;
        
        Ok(count.0)
    }
    
    /// Calculate total tokens in a conversation
    pub async fn calculate_total_tokens(&self, conversation_id: &str) -> Result<i64> {
        let total: (Option<i64>,) = sqlx::query_as(
            "SELECT SUM(tokens) FROM messages WHERE conversation_id = ?"
        )
        .bind(conversation_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to calculate tokens")?;
        
        Ok(total.0.unwrap_or(0))
    }
}

// Import DateTime for the repository methods
use chrono::DateTime;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::database::Database;
    
    async fn setup_test_db() -> ConversationRepository {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();
        ConversationRepository::new(db.pool().clone())
    }
    
    #[tokio::test]
    async fn test_create_and_get_conversation() {
        let repo = setup_test_db().await;
        
        let conv = repo.create_conversation("Test Chat", "gpt-4").await.unwrap();
        let retrieved = repo.get_conversation(&conv.id).await.unwrap();
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Chat");
    }
    
    #[tokio::test]
    async fn test_add_and_retrieve_messages() {
        let repo = setup_test_db().await;
        
        let conv = repo.create_conversation("Test", "gpt-4").await.unwrap();
        
        let msg1 = StoredMessage::new(conv.id.clone(), "user".to_string(), "Hello".to_string());
        repo.add_message(&msg1).await.unwrap();
        
        let msg2 = StoredMessage::new(conv.id.clone(), "assistant".to_string(), "Hi!".to_string());
        repo.add_message(&msg2).await.unwrap();
        
        let messages = repo.get_messages(&conv.id).await.unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "Hello");
        assert_eq!(messages[1].content, "Hi!");
    }
    
    #[tokio::test]
    async fn test_delete_old_messages() {
        let repo = setup_test_db().await;
        
        let conv = repo.create_conversation("Test", "gpt-4").await.unwrap();
        
        // Add 5 messages
        for i in 0..5 {
            let msg = StoredMessage::new(
                conv.id.clone(),
                "user".to_string(),
                format!("Message {}", i),
            );
            repo.add_message(&msg).await.unwrap();
        }
        
        // Keep only last 2
        let deleted = repo.delete_old_messages(&conv.id, 2).await.unwrap();
        assert_eq!(deleted, 3);
        
        let remaining = repo.get_messages(&conv.id).await.unwrap();
        assert_eq!(remaining.len(), 2);
    }
}
