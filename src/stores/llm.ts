import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface LLMState {
  modelPath: string | null;
  modelName: string | null;
  isModelLoaded: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadModel: (modelPath: string) => Promise<void>;
  setModelPath: (path: string) => void;
  setModelName: (name: string | null) => void;
  setModelLoaded: (loaded: boolean) => void;
  hydrateFromSettings: () => Promise<void>;
}

export const useLLMStore = create<LLMState>((set) => ({
  modelPath: null,
  modelName: null,
  isModelLoaded: false,
  isLoading: false,
  error: null,

  loadModel: async (modelName: string) => {
    set({ isLoading: true, error: null });

    try {
      const loadedModelName = await invoke<string>('initialize_llm', { modelName });
      
      // Extract display name from filename (remove .gguf extension)
      const displayName = loadedModelName.replace('.gguf', '');
      
      set({ 
        modelPath: loadedModelName,
        modelName: displayName,
        isModelLoaded: true,
        isLoading: false,
        error: null
      });
    } catch (error) {
      console.error('Error loading model:', error);
      set({ 
        error: error as string,
        isModelLoaded: false,
        isLoading: false,
        modelName: null
      });
    }
  },

  setModelPath: (path: string) => {
    set({ modelPath: path });
  },

  setModelName: (name: string | null) => {
    set({ modelName: name });
  },

  setModelLoaded: (loaded: boolean) => {
    set({ isModelLoaded: loaded });
  },

  // Restore model from settings on app startup
  hydrateFromSettings: async () => {
    set({ isLoading: true });
    try {
      // initialize_llm automatically loads the last used model from settings
      const loadedModelName = await invoke<string>('initialize_llm');
      
      console.log('Model restored and loaded:', loadedModelName);
      
      // Extract display name from filename (remove .gguf extension)
      const displayName = loadedModelName.replace('.gguf', '');
      
      set({ 
        modelPath: loadedModelName,
        modelName: displayName,
        isModelLoaded: true,
        isLoading: false,
        error: null
      });
    } catch (error) {
      console.error('Failed to restore model from settings:', error);
      // If no model was saved or loading failed, just mark as not loading
      set({ 
        isModelLoaded: false,
        isLoading: false,
        error: null // Don't show error if no model was previously saved
      });
    }
  },
}));
