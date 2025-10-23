import { useState } from 'react';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useHuggingFaceStore } from '@/stores/huggingface';
import { Download, Search, Loader2, ExternalLink, Info } from 'lucide-react';
import { ScrollArea } from '@/components/ui/scroll-area';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';

export function HuggingFace() {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedRepo, setSelectedRepo] = useState<string | null>(null);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  const {
    searchResults,
    isSearching,
    searchError,
    selectedModel,
    isLoadingInfo,
    infoError,
    isDownloading,
    downloadError,
    searchModels,
    getModelInfo,
    downloadModel,
    clearSelectedModel,
  } = useHuggingFaceStore();

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;

    try {
      await searchModels({
        search: searchQuery,
        limit: 20,
      });
    } catch (error) {
      console.error('Search failed:', error);
    }
  };

  const handleModelClick = async (modelId: string) => {
    console.log('[HF Page] Model clicked:', modelId);
    setSelectedRepo(modelId);
    clearSelectedModel();
    try {
      await getModelInfo(modelId);
      console.log('[HF Page] Model info loaded successfully');
    } catch (error) {
      console.error('[HF Page] Failed to get model info:', error);
    }
  };

  const handleDownload = async () => {
    if (!selectedRepo || !selectedFile) return;

    try {
      const path = await downloadModel(selectedRepo, selectedFile);
      alert(`Model downloaded successfully to: ${path}`);
      setSelectedFile(null);
    } catch (error) {
      console.error('Download failed:', error);
    }
  };

  return (
    <ErrorBoundary>
      <div className="min-h-screen bg-background p-4">
        <div className="max-w-7xl mx-auto space-y-4">
          <div className="flex items-center justify-between">
            <h1 className="text-3xl font-bold">Hugging Face Models</h1>
          </div>

          {/* Search Bar */}
          <Card className="p-4">
            <div className="flex gap-2">
              <Input
                placeholder="Search models (e.g., 'llama', 'mistral', 'qwen')..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                className="flex-1"
              />
              <Button onClick={handleSearch} disabled={isSearching || !searchQuery.trim()}>
                {isSearching ? (
                  <Loader2 className="h-4 w-4 animate-spin" />
                ) : (
                  <Search className="h-4 w-4" />
                )}
                <span className="ml-2">Search</span>
              </Button>
            </div>

            {searchError && (
              <div className="mt-2 text-sm text-destructive">{searchError}</div>
            )}
          </Card>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {/* Search Results */}
            <Card className="p-4">
              <h2 className="text-xl font-semibold mb-4">Search Results</h2>
              <ScrollArea className="h-[600px]">
                {searchResults.length === 0 ? (
                  <div className="text-center text-muted-foreground py-8">
                    Search for models to get started
                  </div>
                ) : (
                  <div className="space-y-2">
                    {searchResults.map((model) => (
                      <Card
                        key={model.modelId}
                        className={`p-3 cursor-pointer hover:bg-accent transition-colors ${
                          selectedRepo === model.modelId ? 'border-primary' : ''
                        }`}
                        onClick={() => handleModelClick(model.modelId)}
                      >
                        <div className="flex items-start justify-between">
                          <div className="flex-1">
                            <h3 className="font-semibold text-sm">{model.modelId}</h3>
                            {model.author && (
                              <p className="text-xs text-muted-foreground">
                                by {model.author}
                              </p>
                            )}
                            {model.pipeline_tag && (
                              <span className="inline-block mt-1 text-xs bg-secondary px-2 py-0.5 rounded">
                                {model.pipeline_tag}
                              </span>
                            )}
                          </div>
                          <div className="flex flex-col items-end text-xs text-muted-foreground">
                            {model.downloads && <span>↓ {model.downloads.toLocaleString()}</span>}
                            {model.likes && <span>♥ {model.likes.toLocaleString()}</span>}
                          </div>
                        </div>
                        {model.tags && model.tags.length > 0 && (
                          <div className="mt-2 flex flex-wrap gap-1">
                            {model.tags.slice(0, 5).map((tag) => (
                              <span
                                key={tag}
                                className="text-xs bg-muted px-1.5 py-0.5 rounded"
                              >
                                {tag}
                              </span>
                            ))}
                          </div>
                        )}
                      </Card>
                    ))}
                  </div>
                )}
              </ScrollArea>
            </Card>

            {/* Model Details */}
            <Card className="p-4">
              <h2 className="text-xl font-semibold mb-4">Model Details</h2>
              <ScrollArea className="h-[600px]">
                {isLoadingInfo ? (
                  <div className="flex items-center justify-center py-8">
                    <Loader2 className="h-8 w-8 animate-spin text-primary" />
                  </div>
                ) : selectedModel ? (
                  <div className="space-y-4">
                    <div>
                      <h3 className="font-semibold text-lg">{selectedModel.modelId}</h3>
                      {selectedModel.author && (
                        <p className="text-sm text-muted-foreground">by {selectedModel.author}</p>
                      )}
                    </div>

                    <div className="flex items-center gap-2">
                      <Button variant="outline" size="sm" asChild>
                        <a
                          href={`https://huggingface.co/${selectedModel.modelId}`}
                          target="_blank"
                          rel="noopener noreferrer"
                        >
                          <ExternalLink className="h-3 w-3 mr-1" />
                          View on HF
                        </a>
                      </Button>
                    </div>

                    <div className="space-y-2">
                      <h4 className="font-semibold">Information</h4>
                      <div className="text-sm space-y-1">
                        {selectedModel.downloads && (
                          <p>Downloads: {selectedModel.downloads.toLocaleString()}</p>
                        )}
                        {selectedModel.likes && (
                          <p>Likes: {selectedModel.likes.toLocaleString()}</p>
                        )}
                        {selectedModel.library_name && (
                          <p>Library: {selectedModel.library_name}</p>
                        )}
                        {selectedModel.pipeline_tag && (
                          <p>Task: {selectedModel.pipeline_tag}</p>
                        )}
                        {selectedModel.gated && (
                          <p className="flex items-center gap-1">
                            <span className="text-yellow-600">🔒</span>
                            Gated: {typeof selectedModel.gated === 'string' ? selectedModel.gated : 'Yes'}
                          </p>
                        )}
                        {selectedModel.last_modified && (
                          <p>
                            Last modified: {new Date(selectedModel.last_modified).toLocaleDateString()}
                          </p>
                        )}
                      </div>
                    </div>

                    {selectedModel.tags && selectedModel.tags.length > 0 && (
                      <div className="space-y-2">
                        <h4 className="font-semibold">Tags</h4>
                        <div className="flex flex-wrap gap-1">
                          {selectedModel.tags.map((tag) => (
                            <span
                              key={tag}
                              className="text-xs bg-secondary px-2 py-1 rounded"
                            >
                              {tag}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}

                    <div className="space-y-2">
                      <h4 className="font-semibold">Files ({selectedModel.siblings.length})</h4>
                      <div className="space-y-1">
                        {selectedModel.siblings
                          .filter((file) => file && file.filename && file.filename.endsWith('.gguf'))
                          .map((file) => (
                            <Card
                              key={file.filename}
                              className={`p-2 cursor-pointer hover:bg-accent ${
                                selectedFile === file.filename ? 'border-primary' : ''
                              }`}
                              onClick={() => setSelectedFile(file.filename)}
                            >
                              <div className="flex items-center justify-between">
                                <div className="flex-1 min-w-0">
                                  <p className="text-sm font-medium truncate">
                                    {file.filename}
                                  </p>
                                  {file.size && (
                                    <p className="text-xs text-muted-foreground">
                                      {(file.size / 1024 / 1024 / 1024).toFixed(2)} GB
                                    </p>
                                  )}
                                </div>
                              </div>
                            </Card>
                          ))}
                      </div>
                    </div>

                    {selectedFile && (
                      <div className="pt-4 border-t">
                        <Button
                          onClick={handleDownload}
                          disabled={isDownloading}
                          className="w-full"
                        >
                          {isDownloading ? (
                            <>
                              <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                              Downloading...
                            </>
                          ) : (
                            <>
                              <Download className="h-4 w-4 mr-2" />
                              Download {selectedFile}
                            </>
                          )}
                        </Button>
                        {downloadError && (
                          <p className="mt-2 text-sm text-destructive">{downloadError}</p>
                        )}
                      </div>
                    )}
                  </div>
                ) : (
                  <div className="flex flex-col items-center justify-center py-8 text-muted-foreground">
                    <Info className="h-12 w-12 mb-2" />
                    <p>Select a model to view details</p>
                  </div>
                )}
              </ScrollArea>
            </Card>
          </div>
        </div>
      </div>
    </ErrorBoundary>
  );
}
