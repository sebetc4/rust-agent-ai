// Message types
export interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system' | 'tool';
  content: string;
  timestamp: string; // ISO string from backend
  metadata?: Record<string, any>;
}

// Session summary for listing (without messages)
export interface SessionSummary {
  id: string;
  title: string;
  created_at: string; 
  updated_at: string; 
}

// Full conversation session with all messages
export interface ConversationSession {
  id: string;
  title: string;
  created_at: string; 
  updated_at: string; 
  messages: Message[];
  metadata?: Record<string, any>;
}

// Legacy Session type for compatibility
export interface Session {
  id: string;
  title: string;
  created_at: Date;
  updated_at: Date;
}
