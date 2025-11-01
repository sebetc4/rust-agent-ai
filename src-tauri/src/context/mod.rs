/// Module Context - Gestion des sessions et de l'historique conversationnel

pub mod manager;
pub mod session;
pub mod database;
pub mod models;
pub mod repository;
pub mod settings;

pub use manager::ContextManager;
pub use session::{ConversationSession, SessionSummary, Message, MessageRole};
pub use database::{Database, get_default_database_path};
pub use models::{Conversation, StoredMessage};
pub use repository::ConversationRepository;
pub use settings::SettingsRepository;
