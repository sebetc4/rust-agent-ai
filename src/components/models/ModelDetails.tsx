import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { ScrollArea } from '@/components/ui/scroll-area';
import { ExternalLink, Package } from 'lucide-react';
import { FileList } from './FileList';
import { DownloadSection } from './DownloadSection';
import type { GGUFModelMetadata, GGUFFile } from '@/types';

interface ModelDetailsProps {
  model: GGUFModelMetadata | null;
  files: GGUFFile[];
  isLoadingFiles: boolean;
  selectedFile: string | null;
  isDownloading: boolean;
  downloadProgress: number;
  downloadStatus: {
    success: boolean;
    message: string;
    path?: string;
  } | null;
  onFileSelect: (filename: string) => void;
  onDownload: () => void;
}

export function ModelDetails({
  model,
  files,
  isLoadingFiles,
  selectedFile,
  isDownloading,
  downloadProgress,
  downloadStatus,
  onFileSelect,
  onDownload,
}: ModelDetailsProps) {
  if (!model) {
    return (
      <Card className="p-4">
        <h2 className="text-xl font-semibold mb-4">Model Details</h2>
        <ScrollArea className="h-[600px]">
          <div className="flex flex-col items-center justify-center py-8 text-muted-foreground">
            <Package className="h-12 w-12 mb-4 opacity-50" />
            <p>Select a model to view details</p>
          </div>
        </ScrollArea>
      </Card>
    );
  }

  return (
    <Card className="p-4">
      <h2 className="text-xl font-semibold mb-4">Model Details</h2>
      <ScrollArea className="h-[600px]">
        <div className="space-y-4">
          <div>
            <h3 className="font-semibold text-lg">{model.repo_id}</h3>
            {model.author && (
              <p className="text-sm text-muted-foreground">by {model.author}</p>
            )}
          </div>

          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" asChild>
              <a
                href={`https://huggingface.co/${model.repo_id}`}
                target="_blank"
                rel="noopener noreferrer"
              >
                <ExternalLink className="h-3 w-3 mr-1" />
                View on Hugging Face
              </a>
            </Button>
          </div>

          <div className="space-y-2">
            <h4 className="font-semibold">Information</h4>
            <div className="text-sm space-y-1">
              <p>Downloads: {model.downloads.toLocaleString()}</p>
              <p>Likes: {model.likes.toLocaleString()}</p>
              {model.task && <p>Task: {model.task}</p>}
              {model.last_modified && (
                <p>
                  Last modified:{' '}
                  {new Date(model.last_modified).toLocaleDateString()}
                </p>
              )}
            </div>
          </div>

          {model.tags && model.tags.length > 0 && (
            <div className="space-y-2">
              <h4 className="font-semibold">Tags</h4>
              <div className="flex flex-wrap gap-1">
                {model.tags.map((tag) => (
                  <span key={tag} className="text-xs bg-secondary px-2 py-1 rounded">
                    {tag}
                  </span>
                ))}
              </div>
            </div>
          )}

          <div className="space-y-2">
            <h4 className="font-semibold">
              GGUF Files {isLoadingFiles ? '(Loading...)' : `(${files.length})`}
            </h4>
            {isLoadingFiles ? (
              <div className="text-sm text-muted-foreground py-4 text-center">
                Loading files...
              </div>
            ) : files.length > 0 ? (
              <FileList
                files={files}
                selectedFile={selectedFile}
                onFileSelect={onFileSelect}
              />
            ) : (
              <div className="text-sm text-muted-foreground py-4 text-center">
                No GGUF files found
              </div>
            )}
          </div>

          {selectedFile && (
            <DownloadSection
              isDownloading={isDownloading}
              downloadProgress={downloadProgress}
              downloadStatus={downloadStatus}
              onDownload={onDownload}
            />
          )}
        </div>
      </ScrollArea>
    </Card>
  );
}
