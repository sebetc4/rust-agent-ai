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
