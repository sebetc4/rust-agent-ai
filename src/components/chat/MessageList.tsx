import { useEffect, useRef } from 'react';
import { ScrollArea } from '@/components/ui/scroll-area';
import { ChatMessage } from './ChatMessage';
import { Loader2 } from 'lucide-react';
import type { Message } from '@/types';

interface MessageListProps {
  messages: Message[];
  isLoading?: boolean;
}

export function MessageList({ messages, isLoading = false }: MessageListProps) {
  const scrollRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollIntoView({ behavior: 'smooth', block: 'end' });
    }
  }, [messages, isLoading]);

  return (
    <ScrollArea className="h-full w-full">
      <div className="space-y-4 p-4">
        {messages.length === 0 ? (
          <div className="flex items-center justify-center min-h-[200px] text-muted-foreground">
            <p>No messages yet. Start a conversation!</p>
          </div>
        ) : (
          messages.map((message) => (
            <ChatMessage key={message.id} message={message} />
          ))
        )}
        {isLoading && (
          <div className="flex items-center gap-2 text-muted-foreground p-4 rounded-lg bg-muted/50">
            <Loader2 className="h-4 w-4 animate-spin" />
            <span>Generating response...</span>
          </div>
        )}
        <div ref={scrollRef} />
      </div>
    </ScrollArea>
  );
}
