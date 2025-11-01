import { Loader2 } from 'lucide-react';

interface AppLoaderProps {
  message?: string;
}

export function AppLoader({ message = 'Initializing application...' }: AppLoaderProps) {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center">
      <div className="flex flex-col items-center gap-4">
        <Loader2 className="h-12 w-12 animate-spin text-primary" />
        <p className="text-sm text-muted-foreground">{message}</p>
      </div>
    </div>
  );
}
