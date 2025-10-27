import { Card } from '@/components/ui/card';
import type { GGUFFile } from '@/types';

interface FileCardProps {
  file: GGUFFile;
  isSelected: boolean;
  onClick: () => void;
}

export function FileCard({ file, isSelected, onClick }: FileCardProps) {
  return (
    <Card
      className={`p-2 cursor-pointer hover:bg-accent ${
        isSelected ? 'border-primary' : ''
      }`}
      onClick={onClick}
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
  );
}
