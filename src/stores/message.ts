import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { useSessionStore } from './session';
import type { Message } from '../types';

interface MessageState {
  // State
  messages: Message[];
  isGenerating: boolean;
  error: string | null;

  // Actions
  sendMessage: (content: string) => Promise<void>;
  loadMessages: (sessionId: string) => Promise<void>;
  addMessage: (message: Message) => void;
  updateMessage: (id: string, updates: Partial<Message>) => void;
  deleteMessage: (id: string) => void;
  clearMessages: () => void;
  clearError: () => void;
}

export const useMessageStore = create<MessageState>((set) => ({
  messages: [],
  isGenerating: false,
  error: null,

  loadMessages: async (sessionId: string) => {
    try {
      const session = await invoke<{ messages: Message[] }>('get_session', { sessionId });
      set({ messages: session.messages || [], error: null });
    } catch (error) {
      console.error('Failed to load messages:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to load messages',
        messages: []
      });
    }
  },

  sendMessage: async (content: string) => {
    const { activeSessionId } = useSessionStore.getState();
    
    if (!activeSessionId) {
      set({ error: 'No active session' });
      return;
    }

    // Optimistic update : message temporaire pendant la génération
    const tempMessage: Message = {
      id: `temp-${Date.now()}`,
      role: 'user',
      content,
      timestamp: new Date().toISOString()
    };

    set(state => ({ 
      messages: [...state.messages, tempMessage],
      error: null, 
      isGenerating: true 
    }));

    try {
      // Le backend retourne les deux messages avec leurs vrais IDs
      const response = await invoke<{
        user_message: Message;
        assistant_message: Message;
      }>('send_message', {
        sessionId: activeSessionId,
        content: content
      });

      // Remplacer le message temporaire par les vrais messages du backend
      set(state => ({ 
        messages: [
          ...state.messages.filter(m => m.id !== tempMessage.id),
          response.user_message,
          response.assistant_message
        ],
        isGenerating: false 
      }));

    } catch (error) {
      console.error('Failed to send message:', error);
      
      // En cas d'erreur, retirer le message temporaire
      set(state => ({
        messages: state.messages.filter(m => m.id !== tempMessage.id),
        error: error instanceof Error ? error.message : 'Failed to send message',
        isGenerating: false
      }));
    }
  },

  // Ajouter un message manuellement
  addMessage: (message: Message) => {
    set(state => ({ 
      messages: [...state.messages, message] 
    }));
  },

  // Mettre à jour un message existant
  updateMessage: (id: string, updates: Partial<Message>) => {
    set(state => ({
      messages: state.messages.map(msg => 
        msg.id === id ? { ...msg, ...updates } : msg
      )
    }));
  },

  // Supprimer un message
  deleteMessage: (id: string) => {
    set(state => ({
      messages: state.messages.filter(msg => msg.id !== id)
    }));
  },

  // Vider les messages (lors du changement de session)
  clearMessages: () => {
    set({ messages: [], error: null });
  },

  clearError: () => set({ error: null }),
}));
