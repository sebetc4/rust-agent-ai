/// Système de gestion des outils MCP

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Définition d'un outil MCP
#[derive(Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    #[serde(skip)]
    pub handler: Option<Arc<dyn ToolHandler>>,
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("input_schema", &self.input_schema)
            .finish()
    }
}

/// Trait pour implémenter un handler d'outil
#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    async fn execute(&self, arguments: serde_json::Value) -> Result<String>;
}

/// Registre des outils disponibles
pub struct ToolRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolRegistry {
    /// Crée un nouveau registre vide
    pub fn new() -> Self {
        info!("Initialisation du registre d'outils");
        let mut registry = Self {
            tools: HashMap::new(),
        };
        
        // Enregistrer les outils par défaut
        registry.register_default_tools();
        
        registry
    }

    /// Enregistre les outils par défaut
    fn register_default_tools(&mut self) {
        // Outil echo pour test
        let echo_tool = Tool {
            name: "echo".to_string(),
            description: "Retourne le texte fourni en entrée".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "Le texte à retourner"
                    }
                },
                "required": ["text"]
            }),
            handler: Some(Arc::new(EchoHandler)),
        };
        self.tools.insert("echo".to_string(), echo_tool);

        info!("Outils par défaut enregistrés");
    }

    /// Enregistre un nouvel outil
    pub fn register_tool(&mut self, tool: Tool) -> Result<()> {
        if self.tools.contains_key(&tool.name) {
            warn!("Outil {} déjà enregistré, remplacement", tool.name);
        }
        
        info!("Enregistrement de l'outil: {}", tool.name);
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    /// Liste tous les outils disponibles
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools.values().cloned().collect()
    }

    /// Exécute un outil avec les arguments fournis
    pub async fn execute_tool(&self, name: &str, arguments: serde_json::Value) -> Result<String> {
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Outil non trouvé: {}", name))?;

        let handler = tool
            .handler
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Outil {} n'a pas de handler", name))?;

        info!("Exécution de l'outil: {}", name);
        handler.execute(arguments).await
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Implémentations d'outils par défaut =====

/// Handler pour l'outil echo
struct EchoHandler;

#[async_trait::async_trait]
impl ToolHandler for EchoHandler {
    async fn execute(&self, arguments: serde_json::Value) -> Result<String> {
        let text = arguments
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Paramètre 'text' manquant ou invalide"))?;
        
        Ok(format!("Echo: {}", text))
    }
}

/// Outil de lecture de fichiers
pub struct FileReaderHandler;

#[async_trait::async_trait]
impl ToolHandler for FileReaderHandler {
    async fn execute(&self, arguments: serde_json::Value) -> Result<String> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Paramètre 'path' manquant"))?;
        
        let content = tokio::fs::read_to_string(path)
            .await
            .context("Échec de la lecture du fichier")?;
        
        Ok(content)
    }
}

/// Outil d'écriture de fichiers
pub struct FileWriterHandler;

#[async_trait::async_trait]
impl ToolHandler for FileWriterHandler {
    async fn execute(&self, arguments: serde_json::Value) -> Result<String> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Paramètre 'path' manquant"))?;
        
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Paramètre 'content' manquant"))?;
        
        tokio::fs::write(path, content)
            .await
            .context("Échec de l'écriture du fichier")?;
        
        Ok(format!("Fichier écrit avec succès: {}", path))
    }
}

/// Fonction helper pour créer l'outil file_reader
pub fn create_file_reader_tool() -> Tool {
    Tool {
        name: "file_reader".to_string(),
        description: "Lit le contenu d'un fichier texte".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Chemin du fichier à lire"
                }
            },
            "required": ["path"]
        }),
        handler: Some(Arc::new(FileReaderHandler)),
    }
}

/// Fonction helper pour créer l'outil file_writer
pub fn create_file_writer_tool() -> Tool {
    Tool {
        name: "file_writer".to_string(),
        description: "Écrit du contenu dans un fichier".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Chemin du fichier à écrire"
                },
                "content": {
                    "type": "string",
                    "description": "Contenu à écrire dans le fichier"
                }
            },
            "required": ["path", "content"]
        }),
        handler: Some(Arc::new(FileWriterHandler)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert!(!registry.list_tools().is_empty());
    }

    #[tokio::test]
    async fn test_echo_tool() {
        let registry = ToolRegistry::new();
        let result = registry
            .execute_tool("echo", serde_json::json!({"text": "Hello"}))
            .await
            .unwrap();
        assert_eq!(result, "Echo: Hello");
    }

    #[test]
    fn test_tool_registration() {
        let mut registry = ToolRegistry::new();
        let tool = create_file_reader_tool();
        registry.register_tool(tool).unwrap();
        assert!(registry.list_tools().iter().any(|t| t.name == "file_reader"));
    }
}
