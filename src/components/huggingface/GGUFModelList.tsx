import { ScrollArea } from '@/components/ui/scroll-area';
import { GGUFModelCard } from './GGUFModelCard';
import type { GGUFModelInfo } from '@/types';
import { Package } from 'lucide-react';

interface GGUFModelListProps {
  models: GGUFModelInfo[];
  onDownload: (repoId: string, filename: string) => void;
  downloadingModel?: string;
}

export function GGUFModelList({
  models,
  onDownload,
  downloadingModel,
}: GGUFModelListProps) {
  if (models.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-muted-foreground">
        <Package className="h-16 w-16 mb-4 opacity-50" />
        <p className="text-lg font-medium">No GGUF models found</p>
        <p className="text-sm">Try searching for popular models like "llama" or "mistral"</p>
      </div>
    );
  }

  return (
    <ScrollArea className="h-[calc(100vh-280px)]">
      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4 pr-4">
        {models.map((model) => (
          <GGUFModelCard
            key={model.repo_id}
            model={model}
            onDownload={(filename) => onDownload(model.repo_id, filename)}
            isDownloading={downloadingModel === model.repo_id}
          />
        ))}
      </div>
    </ScrollArea>
  );
}
