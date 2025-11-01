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
      await invoke('initialize_llm', { modelName });
      
      // Extract display name from filename (remove .gguf extension)
      const displayName = modelName.replace('.gguf', '');
      
      set({ 
        modelPath: modelName,
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
    try {
      const currentModel = await invoke<string | null>('get_current_model');
      
      if (currentModel) {
        console.log('Restoring model from settings:', currentModel);
        // Extract display name from filename (remove .gguf extension)
        const displayName = currentModel.replace('.gguf', '');
        
        set({ 
          modelPath: currentModel,
          modelName: displayName,
          isModelLoaded: true,
          error: null
        });
      }
    } catch (error) {
      console.error('Failed to restore model from settings:', error);
      // Don't set error state, just log it - this is not critical
    }
  },
}));
