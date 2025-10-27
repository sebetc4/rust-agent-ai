// Message types
export interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
}

// Session types
export interface Session {
  id: string;
  title: string;
  created_at: Date;
  updated_at: Date;
}

// LLM configuration types
export interface LLMConfig {
  model_path?: string;
  temperature?: number;
  top_p?: number;
  top_k?: number;
  repeat_penalty?: number;
  max_tokens?: number;
}

// Tool types
export interface Tool {
  name: string;
  description: string;
  parameters?: Record<string, any>;
}

// Model types
export interface ModelInfo {
  name: string;
  file_name: string;
  size_bytes: number;
  is_loaded: boolean;
}

// Hugging Face types
export interface HFModel {
  modelId: string;
  author?: string;
  downloads?: number;
  likes?: number;
  pipeline_tag?: string;
  tags: string[];
  private?: boolean;
  gated?: boolean | string; // Can be false, true, "manual", or "auto"
  last_modified?: string;
  library_name?: string;
}

export interface HFModelFile {
  filename: string;
  size?: number;
  lfs?: {
    oid: string;
    size: number;
    pointer_size?: number;
  };
}

export interface HFModelInfo {
  modelId: string;
  author?: string;
  sha: string;
  last_modified: string;
  private: boolean;
  disabled?: boolean;
  gated?: boolean | string; // Can be false, true, "manual", or "auto"
  tags: string[];
  pipeline_tag?: string;
  siblings: HFModelFile[];
  downloads?: number;
  likes?: number;
  library_name?: string;
}

export interface HFSearchParams {
  search?: string;
  author?: string;
  task?: string;
  library?: string;
  language?: string;
  sort?: string;
  direction?: string;
  limit?: number;
  full?: boolean;
}

// GGUF specific types
export interface GGUFFile {
  filename: string;
  size: number;
  quantization?: string; // e.g., "Q4_0", "Q8_0"
}

export interface GGUFModelInfo {
  repo_id: string;
  gguf_files: GGUFFile[];
  downloads: number;
  likes: number;
  author: string;
  task?: string;
  tags: string[];
  last_modified: string;
}

// GGUF model metadata (without files)
export interface GGUFModelMetadata {
  repo_id: string;
  downloads: number;
  likes: number;
  author: string;
  task?: string;
  tags: string[];
  last_modified: string;
}
