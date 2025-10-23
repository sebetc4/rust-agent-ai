import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { Message } from '@/types';

interface ChatState {
  messages: Message[];
  currentSessionId: string | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  sendMessage: (content: string) => Promise<void>;
  addMessage: (message: Message) => void;
  clearMessages: () => void;
  setError: (error: string | null) => void;
}

export const useChatStore = create<ChatState>((set, get) => ({
  messages: [],
  currentSessionId: null,
  isLoading: false,
  error: null,

  sendMessage: async (content: string) => {
    const state = get();
    
    // Add user message
    const userMessage: Message = {
      id: crypto.randomUUID(),
      role: 'user',
      content,
      timestamp: new Date(),
    };
    
    set({ 
      messages: [...state.messages, userMessage],
      isLoading: true,
      error: null 
    });

    try {
      // Call Tauri backend
      const response = await invoke<string>('generate_response', { 
        prompt: content 
      });

      // Add assistant message
      const assistantMessage: Message = {
        id: crypto.randomUUID(),
        role: 'assistant',
        content: response,
        timestamp: new Date(),
      };

      set(state => ({ 
        messages: [...state.messages, assistantMessage],
        isLoading: false 
      }));
    } catch (error) {
      console.error('Error generating response:', error);
      set({ 
        error: error as string,
        isLoading: false 
      });
    }
  },

  addMessage: (message) => {
    set(state => ({ 
      messages: [...state.messages, message] 
    }));
  },

  clearMessages: () => {
    set({ messages: [] });
  },

  setError: (error) => {
    set({ error });
  },
}));
