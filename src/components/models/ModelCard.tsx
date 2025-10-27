import { Card } from '@/components/ui/card';
import type { GGUFModelMetadata } from '@/types';

interface ModelCardProps {
  model: GGUFModelMetadata;
  isSelected: boolean;
  onClick: () => void;
}

export function ModelCard({ model, isSelected, onClick }: ModelCardProps) {
  return (
    <Card
      className={`p-3 cursor-pointer hover:bg-accent transition-colors ${
        isSelected ? 'border-primary' : ''
      }`}
      onClick={onClick}
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
  );
}
