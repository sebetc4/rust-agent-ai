import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { ConversationSession } from '../types';

interface SessionState {
  // State
  sessions: ConversationSession[];
  activeSessionId: string | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadSessions: () => Promise<void>;
  createSession: (title: string) => Promise<string | null>;
  selectSession: (id: string) => Promise<void>;
  deleteSession: (id: string) => Promise<void>;
  renameSession: (id: string, newTitle: string) => Promise<void>;
  clearError: () => void;
  
  // Getters
  getActiveSession: () => ConversationSession | null;
}

export const useSessionStore = create<SessionState>((set, get) => ({
  sessions: [],
  activeSessionId: null,
  isLoading: false,
  error: null,

  loadSessions: async () => {
    set({ isLoading: true, error: null });
    try {
      const sessions = await invoke<ConversationSession[]>('list_sessions');
      set({ sessions, isLoading: false });
      
      const { activeSessionId } = get();
      if (!activeSessionId && sessions.length > 0) {
        await get().selectSession(sessions[0].id);
      }
    } catch (error) {
      set({ 
        error: error instanceof Error ? error.message : 'Failed to load sessions',
        isLoading: false 
      });
    }
  },

  createSession: async (title: string) => {
    set({ isLoading: true, error: null });
    try {
      const newSession = await invoke<ConversationSession>('create_session', { title });
      
      set(state => ({
        sessions: [newSession, ...state.sessions],
        activeSessionId: newSession.id,
        isLoading: false
      }));
      
      return newSession.id;
    } catch (error) {
      console.error('Failed to create session:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to create session',
        isLoading: false 
      });
      return null;
    }
  },

  selectSession: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      const session = await invoke<ConversationSession>('get_session', { sessionId: id });
      
      set(state => ({
        sessions: state.sessions.map(s => s.id === id ? session : s),
        activeSessionId: id,
        isLoading: false
      }));

      // Charger les messages dans le message store
      const { useMessageStore } = await import('./message');
      await useMessageStore.getState().loadMessages(id);
    } catch (error) {
      console.error('Failed to load session:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to load session',
        isLoading: false 
      });
    }
  },

  deleteSession: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_session', { sessionId: id });
      
      // Si on supprime la session active, vider les messages
      const wasActive = get().activeSessionId === id;
      
      set(state => {
        const newSessions = state.sessions.filter(s => s.id !== id);
        const newActiveId = state.activeSessionId === id 
          ? (newSessions.length > 0 ? newSessions[0].id : null)
          : state.activeSessionId;
        
        return {
          sessions: newSessions,
          activeSessionId: newActiveId,
          isLoading: false
        };
      });

      // Charger les messages de la nouvelle session active ou vider
      const { useMessageStore } = await import('./message');
      const newActiveId = get().activeSessionId;
      if (wasActive) {
        if (newActiveId) {
          await useMessageStore.getState().loadMessages(newActiveId);
        } else {
          useMessageStore.getState().clearMessages();
        }
      }
    } catch (error) {
      console.error('Failed to delete session:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to delete session',
        isLoading: false 
      });
    }
  },

  renameSession: async (id: string, newTitle: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('rename_session', { 
        sessionId: id, 
        newTitle 
      });
      
      set(state => ({
        sessions: state.sessions.map(s => 
          s.id === id ? { ...s, title: newTitle, updated_at: new Date().toISOString() } : s
        ),
        isLoading: false
      }));
    } catch (error) {
      console.error('Failed to rename session:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to rename session',
        isLoading: false 
      });
    }
  },

  clearError: () => set({ error: null }),

  getActiveSession: () => {
    const { sessions, activeSessionId } = get();
    return sessions.find(s => s.id === activeSessionId) || null;
  }
}));
