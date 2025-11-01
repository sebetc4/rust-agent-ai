import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useSessionStore } from '@/stores/session';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card } from '@/components/ui/card';
import { MessageSquare, Plus, MessageCircle } from 'lucide-react';
import { Path } from '@/constants';

export const HomePage = () => {
  const [newChatInput, setNewChatInput] = useState('');
  const { sessions, createSession, selectSession } = useSessionStore();
  const navigate = useNavigate();

  const handleCreateChat = async () => {
    if (!newChatInput.trim()) return;
    
    const sessionId = await createSession(newChatInput.trim());
    if (sessionId) {
      selectSession(sessionId);
      setNewChatInput('');
      navigate(Path.CHAT);
    }
  };

  const handleSelectSession = (sessionId: string) => {
    selectSession(sessionId);
    navigate(Path.CHAT);
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleCreateChat();
    }
  };

  // Fonction pour formater la date relative
  const getRelativeTime = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = Date.now();
    const diff = now - date.getTime();
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    if (days > 0) return `il y a ${days} jour${days > 1 ? 's' : ''}`;
    if (hours > 0) return `il y a ${hours} heure${hours > 1 ? 's' : ''}`;
    if (minutes > 0) return `il y a ${minutes} minute${minutes > 1 ? 's' : ''}`;
    return 'à l\'instant';
  };

  return (
    <div className="flex flex-col h-screen bg-background">
      {/* Header */}
      <header className="border-b border-border px-6 py-4">
        <div className="flex items-center justify-between max-w-7xl mx-auto">
          <div className="flex items-center gap-2">
            <MessageSquare className="w-6 h-6 text-primary" />
            <h1 className="text-xl font-semibold">Agents RS</h1>
          </div>
          <div className="flex gap-2">
            <Button variant="ghost" onClick={() => navigate(Path.MODELS)}>
              Modèles
            </Button>
            <Button variant="ghost" onClick={() => navigate(Path.SETTINGS)}>
              Paramètres
            </Button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 overflow-y-auto">
        <div className="max-w-3xl mx-auto px-6 py-12">
          {/* Center Input */}
          <div className="mb-12">
            <h2 className="text-3xl font-bold text-center mb-8">
              Comment puis-je vous aider aujourd'hui ?
            </h2>
            <div className="flex gap-2">
              <Input
                placeholder="Démarrer une nouvelle conversation..."
                value={newChatInput}
                onChange={(e) => setNewChatInput(e.target.value)}
                onKeyPress={handleKeyPress}
                className="flex-1 text-base py-6"
              />
              <Button 
                onClick={handleCreateChat}
                disabled={!newChatInput.trim()}
                size="lg"
                className="px-6"
              >
                <Plus className="w-5 h-5" />
              </Button>
            </div>
          </div>

          {/* Conversation Cards */}
          {sessions.length > 0 && (
            <div>
              <h3 className="text-lg font-semibold mb-4">Conversations récentes</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {sessions.map((session) => (
                  <Card
                    key={session.id}
                    className="p-4 cursor-pointer hover:bg-accent transition-colors"
                    onClick={() => handleSelectSession(session.id)}
                  >
                    <div className="flex items-start gap-3">
                      <MessageCircle className="w-5 h-5 text-muted-foreground flex-shrink-0 mt-0.5" />
                      <div className="flex-1 min-w-0">
                        <h4 className="font-medium truncate mb-1">
                          {session.title}
                        </h4>
                        <p className="text-xs text-muted-foreground">
                          {getRelativeTime(session.updated_at)}
                        </p>
                        {session.messages.length > 0 && (
                          <p className="text-sm text-muted-foreground mt-2 line-clamp-2">
                            {session.messages[session.messages.length - 1].content}
                          </p>
                        )}
                      </div>
                    </div>
                  </Card>
                ))}
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
