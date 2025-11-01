import { cn } from '@/lib/utils';
import type { Message } from '@/types';

interface ChatMessageProps {
  message: Message;
}

export function ChatMessage({ message }: ChatMessageProps) {
  const isUser = message.role === 'user';
  
  // Parse timestamp (ISO string from backend)
  const timestamp = new Date(message.timestamp);
  const timeString = timestamp.toLocaleTimeString();

  return (
    <div
      className={cn(
        'flex w-full mb-4',
        isUser ? 'justify-end' : 'justify-start'
      )}
    >
      <div
        className={cn(
          'max-w-[80%] rounded-lg px-4 py-3',
          isUser
            ? 'bg-primary text-primary-foreground'
            : 'bg-muted text-foreground'
        )}
      >
        <div className="text-sm whitespace-pre-wrap break-words">
          {message.content}
        </div>
        <div className="text-xs opacity-70 mt-1">
          {timeString}
        </div>
      </div>
    </div>
  );
}
