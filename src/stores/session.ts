import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { ConversationSession, Message } from '../types';

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
  addMessage: (role: 'user' | 'assistant' | 'system' | 'tool', content: string) => Promise<void>;
  deleteSession: (id: string) => Promise<void>;
  renameSession: (id: string, newTitle: string) => Promise<void>;
  clearError: () => void;
  
  // Getters
  getActiveSession: () => ConversationSession | null;
  getMessages: () => Message[];
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
    } catch (error) {
      console.error('Failed to load session:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to load session',
        isLoading: false 
      });
    }
  },

  // Add a message to the active session
  addMessage: async (role: 'user' | 'assistant' | 'system' | 'tool', content: string) => {
    const { activeSessionId } = get();
    if (!activeSessionId) {
      set({ error: 'No active session' });
      return;
    }

    set({ isLoading: true, error: null });
    try {
      await invoke('add_message', {
        sessionId: activeSessionId,
        role,
        content
      });

      // Reload the active session to get the updated messages
      await get().selectSession(activeSessionId);
    } catch (error) {
      console.error('Failed to add message:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to add message',
        isLoading: false 
      });
    }
  },

  // Delete a session
  deleteSession: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_session', { sessionId: id });
      
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
    } catch (error) {
      console.error('Failed to delete session:', error);
      set({ 
        error: error instanceof Error ? error.message : 'Failed to delete session',
        isLoading: false 
      });
    }
  },

  // Rename a session
  renameSession: async (id: string, newTitle: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('rename_session', { 
        sessionId: id, 
        newTitle 
      });
      
      // Mise à jour optimiste locale
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

  // Clear error
  clearError: () => set({ error: null }),

  // Get active session
  getActiveSession: () => {
    const { sessions, activeSessionId } = get();
    return sessions.find(s => s.id === activeSessionId) || null;
  },

  // Get messages from active session
  getMessages: () => {
    const activeSession = get().getActiveSession();
    return activeSession?.messages || [];
  }
}));
