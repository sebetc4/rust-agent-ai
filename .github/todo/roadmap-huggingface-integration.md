# Roadmap - Hugging Face API Integration

## Overview

This roadmap details the complete integration of the Hugging Face API into agents-rs to enable advanced LLM model management directly from the user interface.

### Objectives

- **Model Discovery**: List and search models on Hugging Face
- **Smart Download**: Download models with progress tracking
- **Local Management**: Delete, organize and validate downloaded models
- **Unified Interface**: Seamless integration with existing UI

---

## Technical Architecture

### Backend (Rust)
```
src-tauri/src/
├── huggingface/           # New module for HF API
│   ├── mod.rs             # Main module
│   ├── api.rs             # Hugging Face API client
│   ├── models.rs          # HF data structures
│   ├── download.rs        # Download manager
│   └── auth.rs            # Authentication token management
├── llm/
│   ├── model_manager.rs   # HF extension (existing)
│   └── ...
└── lib.rs                 # New Tauri commands
```

### Frontend (React)
```
src/
├── components/
│   ├── models/            # New model management module
│   │   ├── ModelBrowser.tsx      # HF model browser
│   │   ├── ModelCard.tsx         # Model display card
│   │   ├── ModelDownloader.tsx   # Download interface
│   │   ├── ModelManager.tsx      # Local manager
│   │   └── ModelSearch.tsx       # Advanced search
│   └── ...
├── stores/
│   ├── models.ts          # Zustand store for model management
│   └── ...
└── pages/
    ├── Models.tsx         # Dedicated models page (/models)
    └── ...
```

---

## Implementation Plan

### Phase 1: Backend Infrastructure (Week 1)

#### 1.1 Hugging Face API Module
- **Data Structures**
  ```rust
  pub struct HFModel {
      pub id: String,
      pub author: String,
      pub downloads: u64,
      pub likes: u32,
      pub pipeline_tag: Option<String>,
      pub tags: Vec<String>,
      pub model_size: Option<u64>,
  }
  
  pub struct HFSearchParams {
      pub search: Option<String>,
      pub author: Option<String>,
      pub task: Option<String>,
      pub library: Option<String>,
      pub sort: Option<String>,
      pub limit: Option<u32>,
  }
  ```

- ✅ **Client API**
  ```rust
  pub struct HuggingFaceClient {
      client: reqwest::Client,
      token: Option<String>,
  }
  
  impl HuggingFaceClient {
      pub async fn list_models(&self, params: HFSearchParams) -> Result<Vec<HFModel>>;
      pub async fn get_model_info(&self, repo_id: &str) -> Result<HFModelDetail>;
      pub async fn download_file(&self, repo_id: &str, filename: &str, path: &Path) -> Result<()>;
  }
  ```

#### 1.2 Download Manager
- **Download with Progress Tracking**
  ```rust
  pub struct DownloadManager {
      pub async fn download_model(
          &self,
          repo_id: &str,
          progress_callback: impl Fn(f64),
      ) -> Result<PathBuf>;
  }
  ```

- **Model Validation**
  - Checksum verification
  - GGUF format validation
  - Basic loading test

#### 1.3 ModelManager Extension
- **HF Integration**
  ```rust
  impl ModelManager {
      pub fn mark_as_hf_model(&mut self, model_path: &Path, hf_info: HFModel);
      pub fn get_hf_models(&self) -> Vec<LocalHFModel>;
      pub fn update_from_hf(&mut self) -> Result<()>;
  }
  ```

### Phase 2: Tauri Commands (Week 1-2)

#### 2.1 New Commands
```rust
// Search and discovery
#[tauri::command]
async fn search_hf_models(params: HFSearchParams) -> Result<Vec<HFModel>, String>;

#[tauri::command]
async fn get_hf_model_details(repo_id: String) -> Result<HFModelDetail, String>;

// Download
#[tauri::command]
async fn download_hf_model(repo_id: String, window: tauri::Window) -> Result<String, String>;

#[tauri::command]
async fn get_download_progress(download_id: String) -> Result<DownloadProgress, String>;

// Local management
#[tauri::command]
async fn get_local_models_with_hf_info() -> Result<Vec<LocalModelWithHF>, String>;

#[tauri::command]
async fn delete_local_model(model_id: String) -> Result<(), String>;

// Authentication
#[tauri::command]
async fn set_hf_token(token: String) -> Result<(), String>;

#[tauri::command]
async fn validate_hf_token() -> Result<bool, String>;
```

#### 2.2 WebSocket Events
- Progress updates for downloads
- Download completion notifications
- Error alerts

### Phase 3: Frontend Interface (Week 2-3)

#### 3.1 Zustand Store
```typescript
interface ModelsStore {
  // State
  hfModels: HFModel[];
  localModels: LocalModel[];
  downloadQueue: DownloadItem[];
  searchParams: HFSearchParams;
  
  // Actions
  searchModels: (params: HFSearchParams) => Promise<void>;
  downloadModel: (repoId: string) => Promise<void>;
  deleteModel: (modelId: string) => Promise<void>;
  setHFToken: (token: string) => Promise<void>;
}
```

#### 3.2 React Components

**ModelBrowser.tsx** - Main browser
```tsx
export function ModelBrowser() {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div className="lg:col-span-1">
        <ModelSearch />
        <ModelFilters />
      </div>
      <div className="lg:col-span-2">
        <ModelGrid />
      </div>
    </div>
  );
}
```

**ModelCard.tsx** - Model card
```tsx
interface ModelCardProps {
  model: HFModel;
  isLocal?: boolean;
  downloadProgress?: number;
}

export function ModelCard({ model, isLocal, downloadProgress }: ModelCardProps) {
  return (
    <Card className="p-4">
      <CardHeader>
        <h3 className="font-semibold">{model.id}</h3>
        <p className="text-sm text-muted-foreground">by {model.author}</p>
      </CardHeader>
      <CardContent>
        <div className="flex justify-between items-center">
          <Badge>{model.pipeline_tag}</Badge>
          <div className="flex gap-2">
            <Button variant="outline" size="sm">
              ℹ️ Details
            </Button>
            {isLocal ? (
              <Button variant="destructive" size="sm">
                🗑️ Delete
              </Button>
            ) : (
              <Button size="sm">
                ⬇️ Download
              </Button>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
```

#### 3.3 Models Page
- Overview of local and remote models
- Advanced search interface
- Download manager
- HF authentication settings

### Phase 4: Advanced Features (Week 3-4)

#### 4.1 Collection Management
- Create custom collections
- Organize models by categories
- Import/export configurations

#### 4.2 Optimizations
- Smart metadata caching
- Background downloads
- Resume interrupted downloads

#### 4.3 Security
- HF token encryption
- Model signature validation
- Sandbox for model testing

---

## Technical Dependencies

### New Rust Dependencies
```toml
[dependencies]
# Already present
reqwest = { version = "0.12", features = ["json", "stream"] }

# To add if necessary
indicatif = "0.17"  # Progress bars
sha2 = "0.10"       # Checksum validation
```

### New Frontend Dependencies
```json
{
  "react-query": "^3.39.0",        // Cache et synchronisation
  "react-hook-form": "^7.48.0",    // Formulaires de recherche
  "@radix-ui/react-progress": "^1.0.0"  // Barres de progression
}
```

---

## Success Metrics

### Functional
- HF model search in less than 2 seconds
- Download with real-time progress tracking
- Unified interface for local/remote models
- Robust error handling

### Technical
- Unit tests > 80% coverage
- Optimized memory management (streaming downloads)
- Responsive interface (mobile-first)
- UI load time < 100ms

### UX
- Intuitive workflow: discovery → download → usage
- Continuous visual feedback
- Graceful error handling
- Complete user documentation

---

## Implementation Priorities

### Critical (Week 1)
1. **HF Backend API** - Core functionality
2. **Tauri Commands** - Backend/frontend bridge
3. **Zustand Store** - State management

### Important (Week 2)
1. **UI Components** - User interface
2. **Download Manager** - Download management
3. **Model Validation** - Security and integrity

### Desirable (Week 3-4)
1. **Advanced Search** - Filters and sorting
2. **Collections** - Personal organization
3. **Performance Optimizations** - Caching and streaming

---

## Next Steps

1. **Phase 1a**: Create `src-tauri/src/huggingface/` module
2. **Phase 1b**: Implement `HuggingFaceClient` with reqwest
3. **Phase 1c**: Extend `ModelManager` for HF
4. **Phase 2a**: Add Tauri commands
5. **Phase 3a**: Create Zustand store `models.ts`

---

**Status**: Roadmap created - Ready for implementation
**Estimation**: 3-4 weeks for complete implementation
**Risks**: Large file handling, HF API rate limiting