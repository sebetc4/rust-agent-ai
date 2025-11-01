import { NavLink } from 'react-router-dom';
import { Button } from '@/components/ui/button';
import { Home, MessageSquare, Download, Settings } from 'lucide-react';
import { Path } from '@/constants';

export const Navigation = () => {
  return (
    <nav className="border-b bg-card">
      <div className="max-w-7xl mx-auto px-4 py-3 flex gap-2">
        <NavLink to={Path.HOME}>
          {({ isActive }) => (
            <Button variant={isActive ? 'default' : 'ghost'}>
              <Home className="h-4 w-4 mr-2" />
              Accueil
            </Button>
          )}
        </NavLink>
        <NavLink to={Path.CHAT}>
          {({ isActive }) => (
            <Button variant={isActive ? 'default' : 'ghost'}>
              <MessageSquare className="h-4 w-4 mr-2" />
              Chat
            </Button>
          )}
        </NavLink>
        <NavLink to={Path.MODELS}>
          {({ isActive }) => (
            <Button variant={isActive ? 'default' : 'ghost'}>
              <Download className="h-4 w-4 mr-2" />
              Models
            </Button>
          )}
        </NavLink>
        <NavLink to={Path.SETTINGS}>
          {({ isActive }) => (
            <Button variant={isActive ? 'default' : 'ghost'}>
              <Settings className="h-4 w-4 mr-2" />
              Settings
            </Button>
          )}
        </NavLink>
      </div>
    </nav>
  );
};
