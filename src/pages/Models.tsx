import { useEffect, useState } from 'react';
import { useHuggingFaceStore } from '@/stores/huggingface';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import {
  Download,
  Search,
  Loader2,
  ExternalLink,
  Package,
  ChevronDown,
  ChevronUp,
} from 'lucide-react';
import type { GGUFModelInfo } from '@/types';

export function Models() {
  const [searchQuery, setSearchQuery] = useState('');
  const [authorFilter, setAuthorFilter] = useState('');
  const [sortBy, setSortBy] = useState<'downloads' | 'likes' | 'created' | 'modified'>('downloads');
  const [showFilters, setShowFilters] = useState(false);
  const [selectedModel, setSelectedModel] = useState<GGUFModelInfo | null>(null);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  const {
    ggufModels,
    isDiscoveringGGUF,
    ggufDiscoveryError,
    discoverGGUFModels,
    downloadModel,
    isDownloading,
  } = useHuggingFaceStore();

  // Auto-discover popular models on mount
  useEffect(() => {
    if (ggufModels.length === 0 && !isDiscoveringGGUF) {
      console.log('[Models Page] Auto-discovering popular GGUF models...');
      discoverGGUFModels({
        sort: sortBy,
        limit: 20,
      });
    }
  }, []);

  const handleSearch = async () => {
    console.log('[Models Page] Searching with:', { searchQuery, authorFilter, sortBy });
    try {
      await discoverGGUFModels({
        search: searchQuery || undefined,
        author: authorFilter || undefined,
        sort: sortBy,
        limit: 20,
      });
      setSelectedModel(null);
      setSelectedFile(null);
    } catch (error) {
      console.error('[Models Page] Search failed:', error);
    }
  };

  const handleQuickSearch = (keyword: string) => {
    setSearchQuery(keyword);
    discoverGGUFModels({
      search: keyword,
      author: authorFilter || undefined,
      sort: sortBy,
      limit: 20,
    });
    setSelectedModel(null);
    setSelectedFile(null);
  };

  const handleModelClick = (model: GGUFModelInfo) => {
    console.log('[Models Page] Model selected:', model.repo_id);
    setSelectedModel(model);
    setSelectedFile(null);
  };

  const handleDownload = async () => {
    if (!selectedModel || !selectedFile) return;

    console.log('[Models Page] Downloading:', selectedModel.repo_id, selectedFile);
    try {
      const path = await downloadModel(selectedModel.repo_id, selectedFile);
      alert(`Model downloaded successfully to: ${path}`);
      setSelectedFile(null);
    } catch (error) {
      console.error('[Models Page] Download failed:', error);
      alert(`Download failed: ${error}`);
    }
  };

  const groupedFiles = selectedModel
    ? selectedModel.gguf_files.reduce((acc, file) => {
        const quant = file.quantization || 'Unknown';
        if (!acc[quant]) acc[quant] = [];
        acc[quant].push(file);
        return acc;
      }, {} as Record<string, typeof selectedModel.gguf_files>)
    : {};

  return (
    <ErrorBoundary>
      <div className="min-h-screen bg-background p-4">
        <div className="max-w-7xl mx-auto space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold">GGUF Models</h1>
              <p className="text-muted-foreground">
                Discover and download GGUF quantized models from Hugging Face
              </p>
            </div>
          </div>

          {/* Search Bar */}
          <Card className="p-4">
            <div className="space-y-3">
              <div className="flex gap-2">
                <Input
                  placeholder="Search models (e.g., 'llama', 'mistral', 'qwen')..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                  className="flex-1"
                />
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setShowFilters(!showFilters)}
                >
                  {showFilters ? (
                    <ChevronUp className="h-4 w-4" />
                  ) : (
                    <ChevronDown className="h-4 w-4" />
                  )}
                </Button>
                <Button onClick={handleSearch} disabled={isDiscoveringGGUF}>
                  {isDiscoveringGGUF ? (
                    <Loader2 className="h-4 w-4 animate-spin" />
                  ) : (
                    <Search className="h-4 w-4" />
                  )}
                  <span className="ml-2">Search</span>
                </Button>
              </div>

              {showFilters && (
                <div className="grid grid-cols-1 md:grid-cols-2 gap-3 pt-3 border-t">
                  <Input
                    placeholder="Filter by author..."
                    value={authorFilter}
                    onChange={(e) => setAuthorFilter(e.target.value)}
                  />
                  <select
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    value={sortBy}
                    onChange={(e) => setSortBy(e.target.value as any)}
                  >
                    <option value="downloads">Most Downloaded</option>
                    <option value="likes">Most Liked</option>
                    <option value="created">Recently Created</option>
                    <option value="modified">Recently Modified</option>
                  </select>
                </div>
              )}

              <div className="flex flex-wrap gap-2">
                {['llama', 'mistral', 'qwen', 'phi', 'gemma'].map((keyword) => (
                  <Button
                    key={keyword}
                    variant="outline"
                    size="sm"
                    onClick={() => handleQuickSearch(keyword)}
                  >
                    {keyword}
                  </Button>
                ))}
              </div>

              {ggufDiscoveryError && (
                <div className="text-sm text-destructive">{ggufDiscoveryError}</div>
              )}
            </div>
          </Card>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {/* Search Results */}
            <Card className="p-4">
              <h2 className="text-xl font-semibold mb-4">
                Search Results {ggufModels.length > 0 && `(${ggufModels.length})`}
              </h2>
              <ScrollArea className="h-[600px]">
                {isDiscoveringGGUF ? (
                  <div className="flex items-center justify-center py-8">
                    <Loader2 className="h-8 w-8 animate-spin text-primary" />
                  </div>
                ) : ggufModels.length === 0 ? (
                  <div className="flex flex-col items-center justify-center py-8 text-muted-foreground">
                    <Package className="h-12 w-12 mb-2" />
                    <p>Search for GGUF models to get started</p>
                  </div>
                ) : (
                  <div className="space-y-2">
                    {ggufModels.map((model) => (
                      <Card
                        key={model.repo_id}
                        className={`p-3 cursor-pointer hover:bg-accent transition-colors ${
                          selectedModel?.repo_id === model.repo_id ? 'border-primary' : ''
                        }`}
                        onClick={() => handleModelClick(model)}
                      >
                        <div className="flex items-start justify-between">
                          <div className="flex-1">
                            <h3 className="font-semibold text-sm">{model.repo_id}</h3>
                            {model.author && (
                              <p className="text-xs text-muted-foreground">by {model.author}</p>
                            )}
                            {model.task && (
                              <span className="inline-block mt-1 text-xs bg-secondary px-2 py-0.5 rounded">
                                {model.task}
                              </span>
                            )}
                          </div>
                          <div className="flex flex-col items-end text-xs text-muted-foreground">
                            <span>↓ {model.downloads.toLocaleString()}</span>
                            <span>♥ {model.likes.toLocaleString()}</span>
                            <span className="text-primary font-medium">
                              {model.gguf_files.length} GGUF
                            </span>
                          </div>
                        </div>
                        {model.tags && model.tags.length > 0 && (
                          <div className="mt-2 flex flex-wrap gap-1">
                            {model.tags.slice(0, 5).map((tag) => (
                              <span key={tag} className="text-xs bg-muted px-1.5 py-0.5 rounded">
                                {tag}
                              </span>
                            ))}
                            {model.tags.length > 5 && (
                              <span className="text-xs text-muted-foreground">
                                +{model.tags.length - 5}
                              </span>
                            )}
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
                {selectedModel ? (
                  <div className="space-y-4">
                    <div>
                      <h3 className="font-semibold text-lg">{selectedModel.repo_id}</h3>
                      {selectedModel.author && (
                        <p className="text-sm text-muted-foreground">by {selectedModel.author}</p>
                      )}
                    </div>

                    <div className="flex items-center gap-2">
                      <Button variant="outline" size="sm" asChild>
                        <a
                          href={`https://huggingface.co/${selectedModel.repo_id}`}
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
                        <p>Downloads: {selectedModel.downloads.toLocaleString()}</p>
                        <p>Likes: {selectedModel.likes.toLocaleString()}</p>
                        <p>GGUF Files: {selectedModel.gguf_files.length}</p>
                        {selectedModel.task && <p>Task: {selectedModel.task}</p>}
                        {selectedModel.last_modified && (
                          <p>
                            Last modified:{' '}
                            {new Date(selectedModel.last_modified).toLocaleDateString()}
                          </p>
                        )}
                      </div>
                    </div>

                    {selectedModel.tags && selectedModel.tags.length > 0 && (
                      <div className="space-y-2">
                        <h4 className="font-semibold">Tags</h4>
                        <div className="flex flex-wrap gap-1">
                          {selectedModel.tags.map((tag) => (
                            <span key={tag} className="text-xs bg-secondary px-2 py-1 rounded">
                              {tag}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}

                    <div className="space-y-2">
                      <h4 className="font-semibold">
                        GGUF Files ({selectedModel.gguf_files.length})
                      </h4>
                      <div className="space-y-3">
                        {Object.entries(groupedFiles).map(([quant, files]) => (
                          <div key={quant} className="space-y-1">
                            <p className="text-sm font-medium text-primary">{quant}</p>
                            {files.map((file) => (
                              <Card
                                key={file.filename}
                                className={`p-2 cursor-pointer hover:bg-accent ${
                                  selectedFile === file.filename ? 'border-primary' : ''
                                }`}
                                onClick={() => setSelectedFile(file.filename)}
                              >
                                <div className="flex items-center justify-between">
                                  <div className="flex-1 min-w-0">
                                    <p className="text-sm font-medium truncate">{file.filename}</p>
                                    <p className="text-xs text-muted-foreground">
                                      {(file.size / 1024 / 1024 / 1024).toFixed(2)} GB
                                    </p>
                                  </div>
                                </div>
                              </Card>
                            ))}
                          </div>
                        ))}
                      </div>
                    </div>

                    {selectedFile && (
                      <div className="pt-4 border-t">
                        <Button onClick={handleDownload} disabled={isDownloading} className="w-full">
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
                      </div>
                    )}
                  </div>
                ) : (
                  <div className="flex flex-col items-center justify-center py-8 text-muted-foreground">
                    <Package className="h-12 w-12 mb-2" />
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
