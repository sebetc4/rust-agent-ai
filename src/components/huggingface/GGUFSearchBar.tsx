import { useState } from 'react';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Search, Loader2, Filter } from 'lucide-react';

interface GGUFSearchBarProps {
  onSearch: (params: {
    search?: string;
    author?: string;
    sort?: string;
  }) => void;
  isSearching: boolean;
  searchError: string | null;
}

export function GGUFSearchBar({ onSearch, isSearching, searchError }: GGUFSearchBarProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [author, setAuthor] = useState('');
  const [sortBy, setSortBy] = useState('downloads');
  const [showFilters, setShowFilters] = useState(false);

  const handleSearch = () => {
    onSearch({
      search: searchQuery || undefined,
      author: author || undefined,
      sort: sortBy,
    });
  };

  return (
    <Card className="p-4">
      <div className="space-y-3">
        {/* Main search */}
        <div className="flex gap-2">
          <Input
            placeholder="Search GGUF models (e.g., 'llama', 'mistral', 'qwen')..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
            className="flex-1"
          />
          <Button
            variant="outline"
            size="icon"
            onClick={() => setShowFilters(!showFilters)}
            className={showFilters ? 'bg-accent' : ''}
          >
            <Filter className="h-4 w-4" />
          </Button>
          <Button onClick={handleSearch} disabled={isSearching}>
            {isSearching ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <Search className="h-4 w-4" />
            )}
            <span className="ml-2">Discover</span>
          </Button>
        </div>

        {/* Advanced filters */}
        {showFilters && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 pt-2 border-t">
            <div>
              <label className="text-sm font-medium mb-1 block">Author</label>
              <Input
                placeholder="e.g., TheBloke, bartowski..."
                value={author}
                onChange={(e) => setAuthor(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              />
            </div>
            <div>
              <label className="text-sm font-medium mb-1 block">Sort By</label>
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value)}
                className="w-full h-9 px-3 rounded-md border border-input bg-background"
              >
                <option value="downloads">Most Downloaded</option>
                <option value="likes">Most Liked</option>
                <option value="created">Recently Created</option>
                <option value="modified">Recently Updated</option>
              </select>
            </div>
          </div>
        )}

        {/* Error message */}
        {searchError && (
          <div className="text-sm text-destructive bg-destructive/10 p-2 rounded">
            {searchError}
          </div>
        )}

        {/* Quick filters */}
        <div className="flex flex-wrap gap-2">
          <span className="text-sm text-muted-foreground">Quick search:</span>
          {['llama', 'mistral', 'qwen', 'phi', 'gemma'].map((term) => (
            <Button
              key={term}
              variant="outline"
              size="sm"
              onClick={() => {
                setSearchQuery(term);
                onSearch({ search: term, sort: sortBy });
              }}
              className="h-7"
            >
              {term}
            </Button>
          ))}
        </div>
      </div>
    </Card>
  );
}
