use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use tracing::{debug, info, warn};

use super::models::{GGUFFile, GGUFModelInfo, Model, ModelInfo, ModelSearchParams};

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

    /// Discover models with GGUF files only
    pub async fn discover_gguf_models(
        &self,
        mut params: ModelSearchParams,
    ) -> Result<Vec<GGUFModelInfo>> {
        debug!("Discovering GGUF models with params: {:?}", params);

        // Build search query to include "gguf" keyword
        let search_query = if let Some(existing_search) = params.search {
            format!("{} gguf", existing_search)
        } else {
            "gguf".to_string()
        };
        
        params.search = Some(search_query);
        params.full = Some(true);

        let mut request = self.client.get(HF_API_MODELS);

        // Add query parameters
        request = request.query(&[("search", params.search.as_ref().unwrap())]);
        
        if let Some(author) = &params.author {
            request = request.query(&[("author", author)]);
        }
        if let Some(task) = &params.task {
            request = request.query(&[("task", task)]);
        }
        request = request.query(&[("full", "true")]);
        
        if let Some(sort) = &params.sort {
            request = request.query(&[("sort", sort)]);
        }
        if let Some(direction) = &params.direction {
            request = request.query(&[("direction", direction)]);
        }
        // Request more to compensate for filtering
        let api_limit = params.limit.unwrap_or(20) * 3; // 3x to get enough after filtering
        request = request.query(&[("limit", api_limit.to_string())]);

        // Add authentication if available
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to Hugging Face API")?;

        let models: Vec<Model> = self.handle_response(response).await?;
        
        info!("Found {} potential GGUF models", models.len());

        // Filter and transform to GGUFModelInfo
        let mut gguf_models = Vec::new();

        for model in models {
            // Validate library_name or tags contain "gguf"
            let has_gguf_library = model
                .library_name
                .as_ref()
                .map(|lib| lib.to_lowercase() == "gguf")
                .unwrap_or(false);

            let has_gguf_tag = model.tags.iter().any(|tag| tag.to_lowercase() == "gguf");

            if !has_gguf_library && !has_gguf_tag {
                debug!("Skipping {} - no gguf library or tag", model.model_id);
                continue;
            }

            // Fetch detailed model info to get siblings
            let model_info = match self.get_model_info(&model.model_id).await {
                Ok(info) => info,
                Err(e) => {
                    warn!("Failed to get info for {}: {}", model.model_id, e);
                    continue;
                }
            };

            // Filter for .gguf files
            let gguf_files: Vec<GGUFFile> = model_info
                .siblings
                .iter()
                .filter(|file| {
                    file.filename.to_lowercase().ends_with(".gguf")
                })
                .map(|file| GGUFFile {
                    filename: file.filename.clone(),
                    size: file.size.unwrap_or(0),
                    quantization: GGUFFile::extract_quantization(&file.filename),
                })
                .collect();

            // Skip if no .gguf files found
            if gguf_files.is_empty() {
                debug!("Skipping {} - no .gguf files found", model.model_id);
                continue;
            }

            info!(
                "Found {} with {} GGUF files",
                model.model_id,
                gguf_files.len()
            );

            gguf_models.push(GGUFModelInfo {
                repo_id: model.model_id,
                gguf_files,
                downloads: model.downloads.unwrap_or(0),
                likes: model.likes.unwrap_or(0),
                author: model.author.unwrap_or_else(|| "Unknown".to_string()),
                task: model.pipeline_tag,
                tags: model.tags,
                last_modified: model.last_modified.unwrap_or_else(|| "Unknown".to_string()),
            });
        }

        info!("Discovered {} models with GGUF files", gguf_models.len());
        
        // Apply limit after filtering
        let final_limit = params.limit.unwrap_or(20) as usize;
        if gguf_models.len() > final_limit {
            gguf_models.truncate(final_limit);
        }
        
        Ok(gguf_models)
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
