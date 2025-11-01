/// Settings repository for key-value persistence

use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::SqlitePool;
use tracing::{debug, info};

pub struct SettingsRepository {
    pool: SqlitePool,
}

impl SettingsRepository {
    /// Create a new repository instance
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Get a setting value by key
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let result = sqlx::query_scalar::<_, String>(
            "SELECT value FROM settings WHERE key = ?"
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch setting")?;
        
        Ok(result)
    }
    
    /// Set a setting value (upsert)
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let now = Utc::now().timestamp();
        
        sqlx::query(
            r#"
            INSERT INTO settings (key, value, updated_at)
            VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(key)
        .bind(value)
        .bind(now)
        .execute(&self.pool)
        .await
        .context("Failed to set setting")?;
        
        debug!("Setting updated: {} = {}", key, value);
        Ok(())
    }
    
    /// Delete a setting
    pub async fn delete(&self, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM settings WHERE key = ?")
            .bind(key)
            .execute(&self.pool)
            .await
            .context("Failed to delete setting")?;
        
        debug!("Setting deleted: {}", key);
        Ok(())
    }
    
    /// Get the current model name
    pub async fn get_current_model(&self) -> Result<Option<String>> {
        self.get("current_model").await
    }
    
    /// Set the current model name
    pub async fn set_current_model(&self, model_name: &str) -> Result<()> {
        self.set("current_model", model_name).await?;
        info!("Current model saved: {}", model_name);
        Ok(())
    }
    
    /// Get the last active session ID
    pub async fn get_last_session_id(&self) -> Result<Option<String>> {
        self.get("last_session_id").await
    }
    
    /// Set the last active session ID
    pub async fn set_last_session_id(&self, session_id: &str) -> Result<()> {
        self.set("last_session_id", session_id).await?;
        debug!("Last session ID saved: {}", session_id);
        Ok(())
    }
    
    /// Get temperature setting
    pub async fn get_temperature(&self) -> Result<Option<f32>> {
        if let Some(val) = self.get("temperature").await? {
            Ok(val.parse().ok())
        } else {
            Ok(None)
        }
    }
    
    /// Set temperature setting
    pub async fn set_temperature(&self, temperature: f32) -> Result<()> {
        self.set("temperature", &temperature.to_string()).await
    }
    
    /// Get top_p setting
    pub async fn get_top_p(&self) -> Result<Option<f32>> {
        if let Some(val) = self.get("top_p").await? {
            Ok(val.parse().ok())
        } else {
            Ok(None)
        }
    }
    
    /// Set top_p setting
    pub async fn set_top_p(&self, top_p: f32) -> Result<()> {
        self.set("top_p", &top_p.to_string()).await
    }
    
    /// Get top_k setting
    pub async fn get_top_k(&self) -> Result<Option<u32>> {
        if let Some(val) = self.get("top_k").await? {
            Ok(val.parse().ok())
        } else {
            Ok(None)
        }
    }
    
    /// Set top_k setting
    pub async fn set_top_k(&self, top_k: u32) -> Result<()> {
        self.set("top_k", &top_k.to_string()).await
    }
    
    /// Get repeat_penalty setting
    pub async fn get_repeat_penalty(&self) -> Result<Option<f32>> {
        if let Some(val) = self.get("repeat_penalty").await? {
            Ok(val.parse().ok())
        } else {
            Ok(None)
        }
    }
    
    /// Set repeat_penalty setting
    pub async fn set_repeat_penalty(&self, repeat_penalty: f32) -> Result<()> {
        self.set("repeat_penalty", &repeat_penalty.to_string()).await
    }
    
    /// List all settings
    pub async fn list_all(&self) -> Result<Vec<(String, String)>> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT key, value FROM settings ORDER BY key"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to list settings")?;
        
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::database::Database;
    
    async fn setup_test_db() -> SettingsRepository {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();
        SettingsRepository::new(db.pool().clone())
    }
    
    #[tokio::test]
    async fn test_get_set_delete() {
        let repo = setup_test_db().await;
        
        // Initially empty
        assert!(repo.get("test_key").await.unwrap().is_none());
        
        // Set value
        repo.set("test_key", "test_value").await.unwrap();
        assert_eq!(repo.get("test_key").await.unwrap(), Some("test_value".to_string()));
        
        // Update value
        repo.set("test_key", "new_value").await.unwrap();
        assert_eq!(repo.get("test_key").await.unwrap(), Some("new_value".to_string()));
        
        // Delete
        repo.delete("test_key").await.unwrap();
        assert!(repo.get("test_key").await.unwrap().is_none());
    }
    
    #[tokio::test]
    async fn test_current_model() {
        let repo = setup_test_db().await;
        
        assert!(repo.get_current_model().await.unwrap().is_none());
        
        repo.set_current_model("Qwen3-1.7B-IQ4_XS.gguf").await.unwrap();
        assert_eq!(
            repo.get_current_model().await.unwrap(),
            Some("Qwen3-1.7B-IQ4_XS.gguf".to_string())
        );
    }
    
    #[tokio::test]
    async fn test_generation_params() {
        let repo = setup_test_db().await;
        
        // Temperature
        repo.set_temperature(0.7).await.unwrap();
        assert_eq!(repo.get_temperature().await.unwrap(), Some(0.7));
        
        // Top P
        repo.set_top_p(0.9).await.unwrap();
        assert_eq!(repo.get_top_p().await.unwrap(), Some(0.9));
        
        // Top K
        repo.set_top_k(40).await.unwrap();
        assert_eq!(repo.get_top_k().await.unwrap(), Some(40));
        
        // Repeat penalty
        repo.set_repeat_penalty(1.1).await.unwrap();
        assert_eq!(repo.get_repeat_penalty().await.unwrap(), Some(1.1));
    }
    
    #[tokio::test]
    async fn test_list_all() {
        let repo = setup_test_db().await;
        
        repo.set("key1", "value1").await.unwrap();
        repo.set("key2", "value2").await.unwrap();
        
        let all = repo.list_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }
}
