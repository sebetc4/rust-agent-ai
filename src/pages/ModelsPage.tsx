import { Navigation } from '@/components/common/Navigation';
import { Models as ModelsContent } from './Models';

export const ModelsPage = () => {
  return (
    <div className="min-h-screen bg-background">
      <Navigation />
      <ModelsContent />
    </div>
  );
};
