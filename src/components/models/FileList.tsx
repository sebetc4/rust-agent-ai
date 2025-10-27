import { FileCard } from './FileCard';
import type { GGUFFile } from '@/types';

interface FileListProps {
  files: GGUFFile[];
  selectedFile: string | null;
  onFileSelect: (filename: string) => void;
}

export function FileList({ files, selectedFile, onFileSelect }: FileListProps) {
  // Group files by quantization
  const groupedFiles = files.reduce((acc, file) => {
    const quant = file.quantization || 'Unknown';
    if (!acc[quant]) acc[quant] = [];
    acc[quant].push(file);
    return acc;
  }, {} as Record<string, GGUFFile[]>);

  return (
    <div className="space-y-3">
      {Object.entries(groupedFiles).map(([quant, groupFiles]) => (
        <div key={quant} className="space-y-1">
          <p className="text-sm font-medium text-primary">{quant}</p>
          {groupFiles.map((file) => (
            <FileCard
              key={file.filename}
              file={file}
              isSelected={selectedFile === file.filename}
              onClick={() => onFileSelect(file.filename)}
            />
          ))}
        </div>
      ))}
    </div>
  );
}
