import { Navigation } from '@/components/common/Navigation';
import { Models as ModelsContent } from './Models';

type Page = 'chat' | 'models' | 'settings';

interface ModelsPageProps {
  onPageChange: (page: Page) => void;
}

export function ModelsPage({ onPageChange }: ModelsPageProps) {
  return (
    <div className="min-h-screen bg-background">
      <Navigation currentPage="models" onPageChange={onPageChange} />
      <ModelsContent />
    </div>
  );
}
