/// SQLite database connection and migrations

use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::ConnectOptions;

use std::str::FromStr;
use tracing::info;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection
    pub async fn new(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)?
            .create_if_missing(true)
            .disable_statement_logging();
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        
        Ok(Self { pool })
    }
    
    /// Initialize database with schema
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations...");
        
        // Create conversations table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                model_name TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create conversations table")?;
        
        // Create messages table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL CHECK(role IN ('user', 'assistant', 'system')),
                content TEXT NOT NULL,
                tokens INTEGER,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create messages table")?;
        
        // Create indexes
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_messages_conversation 
            ON messages(conversation_id)
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create conversation index")?;
        
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_messages_created_at 
            ON messages(created_at)
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create timestamp index")?;
        
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_conversations_updated_at 
            ON conversations(updated_at DESC)
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create conversations index")?;
        
        info!("Database migrations completed successfully");
        
        Ok(())
    }
    
    /// Get the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    /// Close the database connection
    pub async fn close(self) {
        self.pool.close().await;
    }
}

/// Get the default database path for the application
pub fn get_default_database_path() -> Result<String> {
    let app_dir = directories::ProjectDirs::from("com", "agents-rs", "AgentsRS")
        .context("Failed to determine application directory")?;
    
    let data_dir = app_dir.data_dir();
    std::fs::create_dir_all(data_dir)
        .context("Failed to create data directory")?;
    
    let db_path = data_dir.join("conversations.db");
    let db_url = format!("sqlite://{}", db_path.display());
    
    Ok(db_url)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_creation() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        db.migrate().await.unwrap();
        
        // Verify tables exist
        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(db.pool())
            .await
            .unwrap();
        
        assert!(result.len() >= 2);
    }
}
