use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Gated status - can be either boolean or string ("manual", "auto")
#[derive(Debug, Clone)]
pub enum GatedStatus {
    Boolean(bool),
    String(String),
}

impl GatedStatus {
    /// Check if the model is gated (any type of gating)
    pub fn is_gated(&self) -> bool {
        match self {
            GatedStatus::Boolean(b) => *b,
            GatedStatus::String(_) => true, // "manual" or "auto" means gated
        }
    }
}

impl Serialize for GatedStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            GatedStatus::Boolean(b) => serializer.serialize_bool(*b),
            GatedStatus::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for GatedStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Bool(b) => Ok(GatedStatus::Boolean(b)),
            Value::String(s) => Ok(GatedStatus::String(s)),
            _ => Err(serde::de::Error::custom("gated must be bool or string")),
        }
    }
}

impl Default for GatedStatus {
    fn default() -> Self {
        GatedStatus::Boolean(false)
    }
}

/// Represents a model on Hugging Face
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(skip)]
    _id: Option<String>, // Ignore the duplicate 'id' field
    pub author: Option<String>,
    pub downloads: Option<u64>,
    pub likes: Option<u64>,
    pub pipeline_tag: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub private: Option<bool>,
    #[serde(default)]
    pub gated: Option<GatedStatus>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    pub library_name: Option<String>,
}

/// File information within a model repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFile {
    #[serde(rename = "rfilename")]
    pub filename: String,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub lfs: Option<ModelFileLfs>,
}

/// LFS (Large File Storage) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelFileLfs {
    pub oid: String,
    pub size: u64,
    pub pointer_size: Option<u64>,
}

/// Detailed model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(skip)]
    _id: Option<String>, // Ignore the duplicate 'id' field
    pub author: Option<String>,
    pub sha: String,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    pub private: bool,
    pub disabled: Option<bool>,
    #[serde(default)]
    pub gated: Option<GatedStatus>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub pipeline_tag: Option<String>,
    #[serde(default)]
    pub siblings: Vec<ModelFile>,
    pub downloads: Option<u64>,
    pub likes: Option<u64>,
    pub library_name: Option<String>,
}

/// Parameters for searching models
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModelSearchParams {
    pub search: Option<String>,
    pub author: Option<String>,
    pub task: Option<String>,
    pub library: Option<String>,
    pub language: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
    pub limit: Option<u32>,
    pub full: Option<bool>,
}

impl ModelSearchParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(mut self, query: &str) -> Self {
        self.search = Some(query.to_string());
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    pub fn task(mut self, task: &str) -> Self {
        self.task = Some(task.to_string());
        self
    }

    pub fn library(mut self, library: &str) -> Self {
        self.library = Some(library.to_string());
        self
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.to_string());
        self
    }

    pub fn sort_by_downloads(mut self) -> Self {
        self.sort = Some("downloads".to_string());
        self
    }

    pub fn sort_by_likes(mut self) -> Self {
        self.sort = Some("likes".to_string());
        self
    }

    pub fn descending(mut self) -> Self {
        self.direction = Some("desc".to_string());
        self
    }

    pub fn ascending(mut self) -> Self {
        self.direction = Some("asc".to_string());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn full(mut self, full: bool) -> Self {
        self.full = Some(full);
        self
    }
}
