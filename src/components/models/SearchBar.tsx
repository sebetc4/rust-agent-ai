import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { ChevronDown, ChevronUp, Loader2, Search } from 'lucide-react';

interface SearchBarProps {
  searchQuery: string;
  setSearchQuery: (value: string) => void;
  authorFilter: string;
  setAuthorFilter: (value: string) => void;
  sortBy: 'downloads' | 'likes' | 'created' | 'modified';
  setSortBy: (value: 'downloads' | 'likes' | 'created' | 'modified') => void;
  showFilters: boolean;
  setShowFilters: (value: boolean) => void;
  isSearching: boolean;
  error: string | null;
  onSearch: () => void;
  onQuickSearch: (keyword: string) => void;
}

const QUICK_SEARCH_KEYWORDS = ['llama', 'mistral', 'qwen', 'phi', 'gemma'];

export function SearchBar({
  searchQuery,
  setSearchQuery,
  authorFilter,
  setAuthorFilter,
  sortBy,
  setSortBy,
  showFilters,
  setShowFilters,
  isSearching,
  error,
  onSearch,
  onQuickSearch,
}: SearchBarProps) {
  return (
    <Card className="p-4">
      <div className="space-y-3">
        <div className="flex gap-2">
          <Input
            placeholder="Search models (e.g., 'llama', 'mistral', 'qwen')..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && onSearch()}
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
          <Button onClick={onSearch} disabled={isSearching}>
            {isSearching ? (
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
          {QUICK_SEARCH_KEYWORDS.map((keyword) => (
            <Button
              key={keyword}
              variant="outline"
              size="sm"
              onClick={() => onQuickSearch(keyword)}
            >
              {keyword}
            </Button>
          ))}
        </div>

        {error && <div className="text-sm text-destructive">{error}</div>}
      </div>
    </Card>
  );
}
