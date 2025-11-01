import { Navigation } from '@/components/common/Navigation';
import { Settings as SettingsContent } from './Settings';

type Page = 'chat' | 'models' | 'settings';

interface SettingsPageProps {
  onPageChange: (page: Page) => void;
}

export function SettingsPage({ onPageChange }: SettingsPageProps) {
  return (
    <div className="min-h-screen bg-background">
      <Navigation currentPage="settings" onPageChange={onPageChange} />
      <SettingsContent />
    </div>
  );
}
