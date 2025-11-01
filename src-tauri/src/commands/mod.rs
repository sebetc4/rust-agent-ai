/// Module de commandes Tauri
/// 
/// Ce module organise toutes les commandes Tauri en sous-modules spécialisés:
/// - llm: Gestion du moteur LLM et génération de texte
/// - session: Gestion des sessions de conversation
/// - model: Gestion des modèles locaux et GPU
/// - huggingface: Intégration avec HuggingFace Hub

pub mod llm;
pub mod session;
pub mod model;
pub mod huggingface;

// Re-export toutes les commandes pour faciliter l'importation
pub use llm::*;
pub use session::*;
pub use model::*;
pub use huggingface::*;
