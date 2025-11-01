import { Navigation } from '@/components/common/Navigation';
import { Settings as SettingsContent } from './Settings';

export const SettingsPage = () => {
  return (
    <div className="min-h-screen bg-background">
      <Navigation />
      <SettingsContent />
    </div>
  );
};
