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
