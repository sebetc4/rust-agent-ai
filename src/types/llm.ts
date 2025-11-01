// LLM configuration types
export interface LLMConfig {
  model_path?: string;
  temperature?: number;
  top_p?: number;
  top_k?: number;
  repeat_penalty?: number;
  max_tokens?: number;
}

// Model types
export interface ModelInfo {
  name: string;
  file_name: string;
  size_bytes: number;
  is_loaded: boolean;
}
