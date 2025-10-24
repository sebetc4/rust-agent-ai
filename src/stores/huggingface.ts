import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { HFModel, HFModelInfo, HFSearchParams, GGUFModelInfo } from '@/types';

interface HuggingFaceState {
  // Search state
  searchResults: HFModel[];
  isSearching: boolean;
  searchError: string | null;

  // GGUF discovery state
  ggufModels: GGUFModelInfo[];
  isDiscoveringGGUF: boolean;
  ggufDiscoveryError: string | null;

  // Model info state
  selectedModel: HFModelInfo | null;
  isLoadingInfo: boolean;
  infoError: string | null;

  // Download state
  isDownloading: boolean;
  downloadProgress: number;
  downloadError: string | null;

  // Token state
  hasToken: boolean;

  // Actions
  searchModels: (params: HFSearchParams) => Promise<void>;
  discoverGGUFModels: (params?: HFSearchParams) => Promise<void>;
  getModelInfo: (repoId: string) => Promise<void>;
  downloadModel: (repoId: string, filename: string, revision?: string) => Promise<string>;
  setToken: (token: string) => Promise<void>;
  clearSearch: () => void;
  clearSelectedModel: () => void;
}

export const useHuggingFaceStore = create<HuggingFaceState>((set, get) => ({
  // Initial state
  searchResults: [],
  isSearching: false,
  searchError: null,

  ggufModels: [],
  isDiscoveringGGUF: false,
  ggufDiscoveryError: null,

  selectedModel: null,
  isLoadingInfo: false,
  infoError: null,

  isDownloading: false,
  downloadProgress: 0,
  downloadError: null,

  hasToken: false,

  // Search models
  searchModels: async (params: HFSearchParams) => {
    set({ isSearching: true, searchError: null });

    try {
      const results = await invoke<HFModel[]>('hf_search_models', {
        searchQuery: params.search,
        author: params.author,
        task: params.task,
        limit: params.limit || 20,
      });

      set({ searchResults: results, isSearching: false });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      set({ searchError: errorMessage, isSearching: false, searchResults: [] });
      throw error;
    }
  },

  // Discover GGUF models
  discoverGGUFModels: async (params?: HFSearchParams) => {
    set({ isDiscoveringGGUF: true, ggufDiscoveryError: null });

    try {
      const results = await invoke<GGUFModelInfo[]>('hf_discover_gguf_models', {
        searchQuery: params?.search,
        author: params?.author,
        task: params?.task,
        sort: params?.sort,
        limit: params?.limit || 20,
      });

      console.log(`[HF Store] Discovered ${results.length} GGUF models`);
      set({ ggufModels: results, isDiscoveringGGUF: false });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error('[HF Store] Failed to discover GGUF models:', errorMessage);
      set({ ggufDiscoveryError: errorMessage, isDiscoveringGGUF: false, ggufModels: [] });
      throw error;
    }
  },

  // Get model info
  getModelInfo: async (repoId: string) => {
    console.log('[HF Store] Getting model info for:', repoId);
    set({ isLoadingInfo: true, infoError: null });

    try {
      const info = await invoke<HFModelInfo>('hf_get_model_info', { repoId });
      console.log('[HF Store] Model info received:', info);
      set({ selectedModel: info, isLoadingInfo: false });
    } catch (error) {
      console.error('[HF Store] Failed to get model info:', error);
      const errorMessage = error instanceof Error ? error.message : String(error);
      set({ infoError: errorMessage, isLoadingInfo: false, selectedModel: null });
      throw error;
    }
  },

  // Download model
  downloadModel: async (repoId: string, filename: string, revision?: string) => {
    set({ isDownloading: true, downloadError: null, downloadProgress: 0 });

    try {
      const path = await invoke<string>('hf_download_model', {
        repoId,
        filename,
        revision: revision || 'main',
      });

      set({ isDownloading: false, downloadProgress: 100 });
      return path;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      set({ downloadError: errorMessage, isDownloading: false, downloadProgress: 0 });
      throw error;
    }
  },

  // Set authentication token
  setToken: async (token: string) => {
    try {
      await invoke('hf_set_token', { token });
      set({ hasToken: true });
    } catch (error) {
      set({ hasToken: false });
      throw error;
    }
  },

  // Clear search results
  clearSearch: () => {
    set({ searchResults: [], searchError: null });
  },

  // Clear selected model
  clearSelectedModel: () => {
    set({ selectedModel: null, infoError: null });
  },
}));
