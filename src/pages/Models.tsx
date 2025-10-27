import { useEffect, useState } from 'react';
import { useHuggingFaceStore } from '@/stores/huggingface';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';
import { SearchBar, ModelList, ModelDetails } from '@/components/models';
import type { GGUFModelMetadata } from '@/types';

export function Models() {
  const [searchQuery, setSearchQuery] = useState('');
  const [authorFilter, setAuthorFilter] = useState('');
  const [sortBy, setSortBy] = useState<'downloads' | 'likes' | 'created' | 'modified'>('downloads');
  const [showFilters, setShowFilters] = useState(false);
  const [selectedModel, setSelectedModel] = useState<GGUFModelMetadata | null>(null);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [downloadStatus, setDownloadStatus] = useState<{
    success: boolean;
    message: string;
    path?: string;
  } | null>(null);

  const {
    ggufModels,
    isDiscoveringGGUF,
    ggufDiscoveryError,
    discoverGGUFModels,
    selectedModelFiles,
    isLoadingFiles,
    getGGUFFiles,
    downloadModel,
    isDownloading,
    downloadProgress,
  } = useHuggingFaceStore();

  useEffect(() => {
    if (ggufModels.length === 0 && !isDiscoveringGGUF) {
      console.log('[Models Page] Auto-discovering popular GGUF models...');
      discoverGGUFModels({
        sort: sortBy,
        limit: 10,
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

  const handleModelClick = async (model: GGUFModelMetadata) => {
    console.log('[Models Page] Model selected:', model.repo_id);
    setSelectedModel(model);
    setSelectedFile(null);
    
    // Load GGUF files for this model
    try {
      await getGGUFFiles(model.repo_id);
    } catch (error) {
      console.error('[Models Page] Failed to load GGUF files:', error);
    }
  };

  const handleDownload = async () => {
    if (!selectedModel || !selectedFile) return;

    console.log('[Models Page] Downloading:', selectedModel.repo_id, selectedFile);
    setDownloadStatus(null);
    
    try {
      const path = await downloadModel(selectedModel.repo_id, selectedFile);
      console.log('[Models Page] Download successful:', path);
      setDownloadStatus({
        success: true,
        message: 'Model downloaded successfully!',
        path,
      });
      
      // Clear selection after 3 seconds
      setTimeout(() => {
        setSelectedFile(null);
        setDownloadStatus(null);
      }, 3000);
    } catch (error) {
      console.error('[Models Page] Download failed:', error);
      setDownloadStatus({
        success: false,
        message: error instanceof Error ? error.message : 'Download failed',
      });
    }
  };

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

          <SearchBar
            searchQuery={searchQuery}
            setSearchQuery={setSearchQuery}
            authorFilter={authorFilter}
            setAuthorFilter={setAuthorFilter}
            sortBy={sortBy}
            setSortBy={setSortBy}
            showFilters={showFilters}
            setShowFilters={setShowFilters}
            isSearching={isDiscoveringGGUF}
            error={ggufDiscoveryError}
            onSearch={handleSearch}
            onQuickSearch={handleQuickSearch}
          />

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <ModelList
              models={ggufModels}
              isLoading={isDiscoveringGGUF}
              selectedModelId={selectedModel?.repo_id || null}
              onModelSelect={handleModelClick}
            />

            <ModelDetails
              model={selectedModel}
              files={selectedModelFiles}
              isLoadingFiles={isLoadingFiles}
              selectedFile={selectedFile}
              isDownloading={isDownloading}
              downloadProgress={downloadProgress}
              downloadStatus={downloadStatus}
              onFileSelect={setSelectedFile}
              onDownload={handleDownload}
            />
          </div>
        </div>
      </div>
    </ErrorBoundary>
  );
}
