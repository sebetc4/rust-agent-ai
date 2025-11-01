import { Button } from '@/components/ui/button';
import { MessageSquare, Download, Settings } from 'lucide-react';

type Page = 'chat' | 'models' | 'settings';

interface NavigationProps {
  currentPage: Page;
  onPageChange: (page: Page) => void;
}

export function Navigation({ currentPage, onPageChange }: NavigationProps) {
  return (
    <nav className="border-b bg-card">
      <div className="max-w-7xl mx-auto px-4 py-3 flex gap-2">
        <Button 
          variant={currentPage === 'chat' ? 'default' : 'ghost'} 
          onClick={() => onPageChange('chat')}
        >
          <MessageSquare className="h-4 w-4 mr-2" />
          Chat
        </Button>
        <Button 
          variant={currentPage === 'models' ? 'default' : 'ghost'} 
          onClick={() => onPageChange('models')}
        >
          <Download className="h-4 w-4 mr-2" />
          Models
        </Button>
        <Button 
          variant={currentPage === 'settings' ? 'default' : 'ghost'} 
          onClick={() => onPageChange('settings')}
        >
          <Settings className="h-4 w-4 mr-2" />
          Settings
        </Button>
      </div>
    </nav>
  );
}
