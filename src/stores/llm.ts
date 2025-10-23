import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface LLMState {
  modelPath: string | null;
  isModelLoaded: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadModel: (modelPath: string) => Promise<void>;
  setModelPath: (path: string) => void;
}

export const useLLMStore = create<LLMState>((set) => ({
  modelPath: null,
  isModelLoaded: false,
  isLoading: false,
  error: null,

  loadModel: async (modelName: string) => {
    set({ isLoading: true, error: null });

    try {
      await invoke('initialize_llm', { modelName });
      
      set({ 
        modelPath: modelName,
        isModelLoaded: true,
        isLoading: false,
        error: null
      });
    } catch (error) {
      console.error('Error loading model:', error);
      set({ 
        error: error as string,
        isModelLoaded: false,
        isLoading: false 
      });
    }
  },

  setModelPath: (path: string) => {
    set({ modelPath: path });
  },
}));
