import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Download, ExternalLink } from 'lucide-react';
import type { GGUFModelInfo } from '@/types';

interface GGUFModelCardProps {
  model: GGUFModelInfo;
  onDownload: (filename: string) => void;
  isDownloading?: boolean;
}

export function GGUFModelCard({ model, onDownload, isDownloading }: GGUFModelCardProps) {
  // Group files by quantization
  const quantizationGroups = model.gguf_files.reduce((acc, file) => {
    const quant = file.quantization || 'Other';
    if (!acc[quant]) {
      acc[quant] = [];
    }
    acc[quant].push(file);
    return acc;
  }, {} as Record<string, typeof model.gguf_files>);

  const sortedQuantizations = Object.keys(quantizationGroups).sort((a, b) => {
    // Sort quantizations: Q4, Q5, Q6, Q8, F16, F32, Other
    const order = ['Q4_0', 'Q4_K_M', 'Q5_0', 'Q5_K_M', 'Q6_K', 'Q8_0', 'F16', 'F32'];
    const aIndex = order.findIndex(o => a.startsWith(o));
    const bIndex = order.findIndex(o => b.startsWith(o));
    if (aIndex === -1 && bIndex === -1) return a.localeCompare(b);
    if (aIndex === -1) return 1;
    if (bIndex === -1) return -1;
    return aIndex - bIndex;
  });

  return (
    <Card className="p-4 hover:shadow-lg transition-shadow">
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-start justify-between">
          <div className="flex-1">
            <h3 className="font-semibold text-lg">{model.repo_id}</h3>
            <p className="text-sm text-muted-foreground">by {model.author}</p>
            {model.task && (
              <span className="inline-block mt-1 text-xs bg-secondary px-2 py-0.5 rounded">
                {model.task}
              </span>
            )}
          </div>
          <Button variant="outline" size="sm" asChild>
            <a
              href={`https://huggingface.co/${model.repo_id}`}
              target="_blank"
              rel="noopener noreferrer"
            >
              <ExternalLink className="h-3 w-3" />
            </a>
          </Button>
        </div>

        {/* Stats */}
        <div className="flex gap-4 text-sm text-muted-foreground">
          <span>↓ {model.downloads.toLocaleString()}</span>
          <span>♥ {model.likes.toLocaleString()}</span>
          <span>📦 {model.gguf_files.length} files</span>
        </div>

        {/* Tags */}
        {model.tags.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {model.tags.slice(0, 5).map((tag) => (
              <span
                key={tag}
                className="text-xs bg-muted px-1.5 py-0.5 rounded"
              >
                {tag}
              </span>
            ))}
            {model.tags.length > 5 && (
              <span className="text-xs text-muted-foreground">
                +{model.tags.length - 5} more
              </span>
            )}
          </div>
        )}

        {/* Quantizations */}
        <div className="space-y-2">
          <h4 className="text-sm font-semibold">Available Quantizations:</h4>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-2">
            {sortedQuantizations.map((quant) => {
              const files = quantizationGroups[quant];
              const totalSize = files.reduce((sum, f) => sum + f.size, 0);
              const avgSizeGB = (totalSize / files.length / 1024 / 1024 / 1024).toFixed(1);

              return (
                <div
                  key={quant}
                  className="flex items-center justify-between text-xs p-2 bg-muted/50 rounded"
                >
                  <div>
                    <span className="font-medium">{quant}</span>
                    <span className="text-muted-foreground ml-1">
                      ({files.length})
                    </span>
                  </div>
                  <span className="text-muted-foreground">~{avgSizeGB}GB</span>
                </div>
              );
            })}
          </div>
        </div>

        {/* Quick download button for smallest file */}
        <div className="pt-2 border-t">
          <Button
            onClick={() => {
              // Find smallest file
              const smallest = model.gguf_files.reduce((min, file) =>
                file.size < min.size ? file : min
              );
              onDownload(smallest.filename);
            }}
            disabled={isDownloading}
            size="sm"
            className="w-full"
          >
            <Download className="h-4 w-4 mr-2" />
            {isDownloading ? 'Downloading...' : 'Download Smallest'}
          </Button>
        </div>
      </div>
    </Card>
  );
}
