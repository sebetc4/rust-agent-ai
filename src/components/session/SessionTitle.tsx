import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Pencil, Check, X } from 'lucide-react';

interface SessionTitleProps {
  title: string;
  onRename?: (newTitle: string) => void;
  isEditable?: boolean;
}

export function SessionTitle({ title, onRename, isEditable = false }: SessionTitleProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editedTitle, setEditedTitle] = useState(title);

  const handleSave = () => {
    if (editedTitle.trim() && editedTitle !== title && onRename) {
      onRename(editedTitle.trim());
    }
    setIsEditing(false);
  };

  const handleCancel = () => {
    setEditedTitle(title);
    setIsEditing(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSave();
    } else if (e.key === 'Escape') {
      handleCancel();
    }
  };

  if (!isEditable || !isEditing) {
    return (
      <div className="flex items-center gap-2 flex-1">
        <h1 className="text-2xl font-semibold flex-1 truncate">
          {title}
        </h1>
        {isEditable && (
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8"
            onClick={() => setIsEditing(true)}
          >
            <Pencil className="h-4 w-4" />
          </Button>
        )}
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2 flex-1">
      <Input
        value={editedTitle}
        onChange={(e) => setEditedTitle(e.target.value)}
        onKeyDown={handleKeyDown}
        className="text-2xl font-semibold h-10"
        autoFocus
        onFocus={(e) => e.target.select()}
      />
      <Button
        variant="ghost"
        size="icon"
        className="h-8 w-8 text-green-600"
        onClick={handleSave}
      >
        <Check className="h-4 w-4" />
      </Button>
      <Button
        variant="ghost"
        size="icon"
        className="h-8 w-8 text-destructive"
        onClick={handleCancel}
      >
        <X className="h-4 w-4" />
      </Button>
    </div>
  );
}
