import { useEffect, useState } from 'react';
import { useLLMStore } from '../stores/llm';
import { useSessionStore } from '../stores/session';

export const useHydration = () => {
  const [isHydrated, setIsHydrated] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const hydrateModel = useLLMStore(state => state.hydrateFromSettings);
  const loadSessions = useSessionStore(state => state.loadSessions);

  useEffect(() => {
    let isMounted = true;

    async function hydrate() {
      try {
        console.log('🔄 Hydrating app state from persistence...');
        
        await hydrateModel();
        await loadSessions();
        
        if (isMounted) {
          console.log('✅ Hydration complete');
          setIsHydrated(true);
        }
      } catch (err) {
        console.error('❌ Hydration failed:', err);
        if (isMounted) {
          setError(err instanceof Error ? err.message : 'Hydration failed');
          setIsHydrated(true); // Still mark as hydrated to prevent infinite loading
        }
      }
    }

    hydrate();

    return () => {
      isMounted = false;
    };
  }, []); // Run only once on mount

  return { isHydrated, error };
}
