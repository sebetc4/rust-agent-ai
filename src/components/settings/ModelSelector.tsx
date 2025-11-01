import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Loader2, Check } from 'lucide-react';
import { useLLMStore } from '@/stores/llm';

interface ModelInfo {
  name: string;
  file_name: string;
  size_bytes: number;
  is_loaded: boolean;
}

export function ModelSelector() {
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [switching, setSwitching] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { setModelName } = useLLMStore();

  useEffect(() => {
    loadModels();
  }, []);

  const loadModels = async () => {
    try {
      setLoading(true);
      setError(null);
      const modelList = await invoke<ModelInfo[]>('list_models');
      setModels(modelList);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load models');
      console.error('Error loading models:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSwitchModel = async (fileName: string) => {
    try {
      setSwitching(true);
      setError(null);
      const result = await invoke<string>('switch_model', { modelName: fileName });
      console.log(result);
      
      // Update the LLM store with the new model name
      const displayName = fileName.replace('.gguf', '');
      setModelName(displayName);
      
      // Reload the model list to update is_loaded status
      await loadModels();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to switch model');
      console.error('Error switching model:', err);
    } finally {
      setSwitching(false);
    }
  };

  const formatSize = (bytes: number): string => {
    const mb = bytes / (1024 * 1024);
    const gb = bytes / (1024 * 1024 * 1024);
    return gb >= 1 ? `${gb.toFixed(2)} GB` : `${mb.toFixed(2)} MB`;
  };

  if (loading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Model Selection</CardTitle>
          <CardDescription>Choose which LLM model to use</CardDescription>
        </CardHeader>
        <CardContent className="flex items-center justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Model Selection</CardTitle>
        <CardDescription>Choose which LLM model to use</CardDescription>
      </CardHeader>
      <CardContent className="space-y-3">
        {error && (
          <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
            {error}
          </div>
        )}
        
        {models.length === 0 ? (
          <p className="text-sm text-muted-foreground">
            No models found in the models directory.
          </p>
        ) : (
          models.map((model) => (
            <div
              key={model.file_name}
              className="flex items-center justify-between rounded-lg border p-4"
            >
              <div className="flex-1">
                <div className="flex items-center gap-2">
                  <h4 className="font-medium">{model.name}</h4>
                  {model.is_loaded && (
                    <span className="flex items-center gap-1 rounded-full bg-primary/10 px-2 py-0.5 text-xs font-medium text-primary">
                      <Check className="h-3 w-3" />
                      Active
                    </span>
                  )}
                </div>
                <p className="text-sm text-muted-foreground">
                  {model.file_name} • {formatSize(model.size_bytes)}
                </p>
              </div>
              
              <Button
                onClick={() => handleSwitchModel(model.file_name)}
                disabled={model.is_loaded || switching}
                variant={model.is_loaded ? "secondary" : "default"}
                size="sm"
              >
                {switching && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                {model.is_loaded ? 'Loaded' : 'Load'}
              </Button>
            </div>
          ))
        )}
      </CardContent>
    </Card>
  );
}
