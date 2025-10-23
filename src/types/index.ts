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
