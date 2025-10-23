use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use tracing::{debug, info};

use super::models::{Model, ModelInfo, ModelSearchParams};

const HF_API_BASE: &str = "https://huggingface.co";
const HF_API_MODELS: &str = "https://huggingface.co/api/models";

/// Hugging Face API client
#[derive(Debug, Clone)]
pub struct HuggingFaceClient {
    client: Client,
    token: Option<String>,
}

impl HuggingFaceClient {
    /// Create a new Hugging Face client without authentication
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("agents-rs/0.1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            token: None,
        })
    }

    /// Create a new Hugging Face client with authentication token
    pub fn with_token(token: impl Into<String>) -> Result<Self> {
        let client = Client::builder()
            .user_agent("agents-rs/0.1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            token: Some(token.into()),
        })
    }

    /// Set the authentication token
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    /// Search for models on Hugging Face
    pub async fn search_models(&self, params: ModelSearchParams) -> Result<Vec<Model>> {
        debug!("Searching models with params: {:?}", params);

        let mut request = self.client.get(HF_API_MODELS);

        // Add query parameters
        if let Some(search) = &params.search {
            request = request.query(&[("search", search)]);
        }
        if let Some(author) = &params.author {
            request = request.query(&[("author", author)]);
        }
        if let Some(task) = &params.task {
            request = request.query(&[("task", task)]);
        }
        if let Some(library) = &params.library {
            request = request.query(&[("library", library)]);
        }
        if let Some(language) = &params.language {
            request = request.query(&[("language", language)]);
        }
        if let Some(sort) = &params.sort {
            request = request.query(&[("sort", sort)]);
        }
        if let Some(direction) = &params.direction {
            request = request.query(&[("direction", direction)]);
        }
        if let Some(limit) = params.limit {
            request = request.query(&[("limit", limit.to_string())]);
        }
        if let Some(full) = params.full {
            request = request.query(&[("full", full.to_string())]);
        }

        // Add authentication if available
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to Hugging Face API")?;

        self.handle_response(response).await
    }

    /// Get detailed information about a specific model
    pub async fn get_model_info(&self, repo_id: &str) -> Result<ModelInfo> {
        debug!("Fetching model info for: {}", repo_id);

        let url = format!("{}/{}", HF_API_MODELS, repo_id);
        let mut request = self.client.get(&url);

        // Add authentication if available
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to fetch model info")?;

        self.handle_response(response).await
    }

    /// Download a specific file from a model repository
    pub async fn download_file(
        &self,
        repo_id: &str,
        filename: &str,
        revision: Option<&str>,
        output_path: PathBuf,
    ) -> Result<PathBuf> {
        let revision = revision.unwrap_or("main");
        let url = format!(
            "{}/{}/resolve/{}/{}",
            HF_API_BASE, repo_id, revision, filename
        );

        info!("Downloading {} from {} to {:?}", filename, repo_id, output_path);

        let mut request = self.client.get(&url);

        // Add authentication if available
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to download file")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Failed to download file: HTTP {} - {}",
                status,
                error_text
            ));
        }

        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("Failed to create output directory")?;
        }

        // Download file
        let bytes = response
            .bytes()
            .await
            .context("Failed to read response bytes")?;

        tokio::fs::write(&output_path, bytes)
            .await
            .context("Failed to write file to disk")?;

        info!("Successfully downloaded file to {:?}", output_path);
        Ok(output_path)
    }

    /// Download a specific file with progress callback
    pub async fn download_file_with_progress<F>(
        &self,
        repo_id: &str,
        filename: &str,
        revision: Option<&str>,
        output_path: PathBuf,
        mut progress_callback: F,
    ) -> Result<PathBuf>
    where
        F: FnMut(u64, Option<u64>), // (downloaded_bytes, total_bytes)
    {
        let revision = revision.unwrap_or("main");
        let url = format!(
            "{}/{}/resolve/{}/{}",
            HF_API_BASE, repo_id, revision, filename
        );

        info!("Downloading {} from {} to {:?}", filename, repo_id, output_path);

        let mut request = self.client.get(&url);

        // Add authentication if available
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to download file")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Failed to download file: HTTP {} - {}",
                status,
                error_text
            ));
        }

        // Get total size if available
        let total_size = response.content_length();

        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context("Failed to create output directory")?;
        }

        // Download with progress tracking
        use tokio::io::AsyncWriteExt;

        let mut file = tokio::fs::File::create(&output_path)
            .await
            .context("Failed to create output file")?;

        let bytes = response.bytes().await.context("Failed to read response bytes")?;
        
        file.write_all(&bytes)
            .await
            .context("Failed to write file")?;
        
        let downloaded = bytes.len() as u64;
        progress_callback(downloaded, total_size);

        file.flush().await.context("Failed to flush file")?;

        info!("Successfully downloaded file to {:?}", output_path);
        Ok(output_path)
    }

    /// Handle API response and deserialize JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Hugging Face API error: HTTP {} - {}",
                status,
                error_text
            ));
        }

        let text = response.text().await.context("Failed to read response")?;
        
        // Debug: log the response for troubleshooting
        debug!("API Response (first 500 chars): {}", &text.chars().take(500).collect::<String>());
        
        serde_json::from_str(&text).with_context(|| {
            format!("Failed to deserialize response JSON. Response preview: {}", 
                &text.chars().take(200).collect::<String>())
        })
    }
}

impl Default for HuggingFaceClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HuggingFace client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_models() {
        let client = HuggingFaceClient::new().unwrap();
        let params = ModelSearchParams::new()
            .search("bert")
            .limit(5);

        let result = client.search_models(params).await;
        assert!(result.is_ok());
        
        let models = result.unwrap();
        assert!(!models.is_empty());
    }

    #[tokio::test]
    async fn test_get_model_info() {
        let client = HuggingFaceClient::new().unwrap();
        let result = client.get_model_info("bert-base-uncased").await;
        assert!(result.is_ok());
        
        let info = result.unwrap();
        assert_eq!(info.model_id, "bert-base-uncased");
    }
}
