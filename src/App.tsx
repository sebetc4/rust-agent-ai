import "./index.css";
import { useEffect } from "react";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ChatInput } from "@/components/chat/ChatInput";
import { MessageList } from "@/components/chat/MessageList";
import { useChatStore } from "@/stores/chat";
import { useLLMStore } from "@/stores/llm";
import { Loader2, AlertCircle } from "lucide-react";

function App() {
  const { messages, isLoading, error, sendMessage } = useChatStore();
  const { isModelLoaded, isLoading: isModelLoading, error: modelError, loadModel } = useLLMStore();

  // Load model on startup
  useEffect(() => {
    // Default model name (file name without path)
    const modelName = "Qwen3-1.7B-IQ4_XS.gguf";
    loadModel(modelName);
  }, [loadModel]);

  // Show loading state while model is loading
  if (isModelLoading) {
    return (
      <main className="min-h-screen bg-background flex items-center justify-center">
        <Card className="p-8 flex flex-col items-center gap-4">
          <Loader2 className="h-8 w-8 animate-spin text-primary" />
          <div className="text-center">
            <h2 className="text-lg font-semibold">Loading Model</h2>
            <p className="text-sm text-muted-foreground">
              Initializing Qwen3-1.7B...
            </p>
          </div>
        </Card>
      </main>
    );
  }

  // Show error if model failed to load
  if (modelError) {
    return (
      <main className="min-h-screen bg-background flex items-center justify-center p-4">
        <Card className="p-8 max-w-md">
          <div className="flex items-start gap-4">
            <AlertCircle className="h-6 w-6 text-destructive flex-shrink-0 mt-0.5" />
            <div className="flex-1">
              <h2 className="text-lg font-semibold mb-2">Model Loading Failed</h2>
              <p className="text-sm text-muted-foreground mb-4">
                {modelError}
              </p>
              <Button 
                onClick={() => loadModel("Qwen3-1.7B-IQ4_XS.gguf")}
                variant="outline"
              >
                Retry
              </Button>
            </div>
          </div>
        </Card>
      </main>
    );
  }

  return (
    <main className="min-h-screen bg-background p-4">
      <div className="max-w-4xl mx-auto h-[calc(100vh-2rem)]">
        <Card className="h-full flex flex-col">
          {/* Header */}
          <div className="border-b p-4">
            <h1 className="text-2xl font-semibold">Chat Interface</h1>
            <p className="text-sm text-muted-foreground">
              {isModelLoaded ? "Model loaded • Ready to chat" : "Initializing..."}
            </p>
          </div>

          {/* Messages Area */}
          <div className="flex-1 overflow-hidden">
            <MessageList messages={messages} />
          </div>

          {/* Loading Indicator */}
          {isLoading && (
            <div className="px-4 py-2 flex items-center gap-2 text-sm text-muted-foreground">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Generating response...</span>
            </div>
          )}

          {/* Error Display */}
          {error && (
            <div className="px-4 py-2 bg-destructive/10 text-destructive text-sm">
              Error: {error}
            </div>
          )}

          {/* Input Area */}
          <div className="border-t p-4">
            <ChatInput onSend={sendMessage} disabled={isLoading || !isModelLoaded} />
          </div>
        </Card>
      </div>
    </main>
  );
}

export default App;
