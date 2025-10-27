import { Card } from '@/components/ui/card';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Loader2, Package } from 'lucide-react';
import { ModelCard } from './ModelCard';
import type { GGUFModelMetadata } from '@/types';

interface ModelListProps {
  models: GGUFModelMetadata[];
  isLoading: boolean;
  selectedModelId: string | null;
  onModelSelect: (model: GGUFModelMetadata) => void;
}

export function ModelList({
  models,
  isLoading,
  selectedModelId,
  onModelSelect,
}: ModelListProps) {
  return (
    <Card className="p-4">
      <h2 className="text-xl font-semibold mb-4">
        Search Results {models.length > 0 && `(${models.length})`}
      </h2>
      <ScrollArea className="h-[600px]">
        {isLoading ? (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-8 w-8 animate-spin text-primary" />
          </div>
        ) : models.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-8 text-muted-foreground">
            <Package className="h-12 w-12 mb-2" />
            <p>Search for GGUF models to get started</p>
          </div>
        ) : (
          <div className="space-y-2">
            {models.map((model) => (
              <ModelCard
                key={model.repo_id}
                model={model}
                isSelected={selectedModelId === model.repo_id}
                onClick={() => onModelSelect(model)}
              />
            ))}
          </div>
        )}
      </ScrollArea>
    </Card>
  );
}
