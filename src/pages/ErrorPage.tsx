import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { AlertCircle } from 'lucide-react';
import { useLLMStore } from '@/stores/llm';

interface ErrorPageProps {
  error: string;
}

export const ErrorPage = ({ error }: ErrorPageProps) => {
  const { loadModel } = useLLMStore();

  return (
    <main className="min-h-screen bg-background flex items-center justify-center p-4">
      <Card className="p-8 max-w-md">
        <div className="flex items-start gap-4">
          <AlertCircle className="h-6 w-6 text-destructive flex-shrink-0 mt-0.5" />
          <div className="flex-1">
            <h2 className="text-lg font-semibold mb-2">Model Loading Failed</h2>
            <p className="text-sm text-muted-foreground mb-4">{error}</p>
            <Button onClick={() => loadModel("Qwen3-1.7B-IQ4_XS.gguf")} variant="outline">
              Retry
            </Button>
          </div>
        </div>
      </Card>
    </main>
  );
}
