import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ChatInput } from '@/components/chat/ChatInput';
import { MessageList } from '@/components/chat/MessageList';
import { SessionList, SessionTitle } from '@/components/session';
import { useSessionStore } from '@/stores/session';
import { useLLMStore } from '@/stores/llm';
import { useNavigate } from 'react-router-dom';
import { Loader2, Download, Settings, Cpu, Menu, X } from 'lucide-react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Path } from '@/constants';

export const ChatPage = () => {
  const navigate = useNavigate();
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const [isGenerating, setIsGenerating] = useState(false);
  const [generationError, setGenerationError] = useState<string | null>(null);
  
  const { 
    getMessages, 
    getActiveSession, 
    isLoading: isSessionLoading, 
    error: sessionError, 
    loadSessions,
    createSession,
    renameSession,
    activeSessionId 
  } = useSessionStore();
  
  const { isModelLoaded, modelName } = useLLMStore();
  
  const messages = getMessages();
  const activeSession = getActiveSession();

  // Create initial session if none exists
  useEffect(() => {
    if (!activeSessionId && isModelLoaded) {
      createSession('New Conversation');
    }
  }, [activeSessionId, isModelLoaded, createSession]);

  const handleSendMessage = async (content: string) => {
    if (!activeSessionId) {
      console.error('No active session');
      return;
    }

    setGenerationError(null);
    setIsGenerating(true);
    
    try {
      // Envoyer le message et générer la réponse en un seul appel
      const response = await invoke<string>('send_message', {
        sessionId: activeSessionId,
        content: content
      });

      console.log('LLM Response:', response);
      
      // Recharger la session pour afficher les nouveaux messages
      await loadSessions();
      
    } catch (error) {
      console.error('Failed to send message:', error);
      setGenerationError(error instanceof Error ? error.message : 'Failed to send message');
    } finally {
      setIsGenerating(false);
    }
  };

  const handleRenameSession = async (newTitle: string) => {
    if (activeSessionId) {
      await renameSession(activeSessionId, newTitle);
    }
  };

  return (
    <main className="min-h-screen bg-background flex">
      {/* Sidebar */}
      <aside 
        className={`
          ${isSidebarOpen ? 'w-80' : 'w-0'} 
          transition-all duration-300 overflow-hidden border-r
        `}
      >
        <div className="h-screen">
          <SessionList />
        </div>
      </aside>

      {/* Main Chat Area */}
      <div className="flex-1 flex flex-col p-4">
        <Card className="flex-1 flex flex-col">
          <div className="border-b p-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={() => setIsSidebarOpen(!isSidebarOpen)}
                >
                  {isSidebarOpen ? (
                    <X className="h-5 w-5" />
                  ) : (
                    <Menu className="h-5 w-5" />
                  )}
                </Button>
                
                <div>
                  <div className="flex items-center gap-3">
                    <SessionTitle
                      title={activeSession?.title || 'Chat Interface'}
                      onRename={handleRenameSession}
                      isEditable={!!activeSession}
                    />
                    {isModelLoaded && modelName && (
                      <span className="flex items-center gap-1.5 rounded-md bg-primary/10 px-2.5 py-1 text-xs font-medium text-primary">
                        <Cpu className="h-3.5 w-3.5" />
                        {modelName}
                      </span>
                    )}
                  </div>
                  <p className="text-sm text-muted-foreground mt-1">
                    {isModelLoaded ? "Ready to chat" : "Initializing..."}
                  </p>
                </div>
              </div>
              
              <div className="flex gap-2">
                <Button variant="outline" size="sm" onClick={() => navigate(Path.MODELS)}>
                  <Download className="h-4 w-4 mr-2" />
                  Models
                </Button>
                <Button variant="outline" size="sm" onClick={() => navigate(Path.SETTINGS)}>
                  <Settings className="h-4 w-4 mr-2" />
                  Settings
                </Button>
              </div>
            </div>
          </div>
          
          <div className="flex-1 overflow-hidden">
            <MessageList messages={messages} isLoading={isGenerating} />
          </div>
          
          {isSessionLoading && (
            <div className="px-4 py-2 flex items-center gap-2 text-sm text-muted-foreground">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Processing...</span>
            </div>
          )}
          
          {sessionError && (
            <div className="px-4 py-2 bg-destructive/10 text-destructive text-sm">
              Error: {sessionError}
            </div>
          )}
          
          {generationError && (
            <div className="px-4 py-2 bg-destructive/10 text-destructive text-sm">
              Error: {generationError}
            </div>
          )}
          
          <div className="border-t p-4">
            <ChatInput 
              onSend={handleSendMessage} 
              disabled={isSessionLoading || isGenerating || !isModelLoaded || !activeSessionId} 
            />
          </div>
        </Card>
      </div>
    </main>
  );
}
