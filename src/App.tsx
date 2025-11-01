import "./index.css";
import { useState } from "react";
import { useLLMStore } from "@/stores/llm";
import { useHydration } from "@/hooks/useHydration";
import { 
  ChatPage, 
  ModelsPage, 
  SettingsPage, 
  ModelLoadingPage, 
  ModelErrorPage 
} from "@/pages";

type Page = 'chat' | 'models' | 'settings';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('chat');
  const { isLoading: isModelLoading, error: modelError } = useLLMStore();
  const { isHydrated, error: hydrationError } = useHydration();

  // Show loading while hydrating app state
  if (!isHydrated || isModelLoading) {
    return <ModelLoadingPage />;
  }

  // Show error if hydration failed critically
  if (hydrationError) {
    return <ModelErrorPage error={hydrationError} />;
  }

  // Show model error if loading failed
  if (modelError) {
    return <ModelErrorPage error={modelError} />;
  }

  switch (currentPage) {
    case 'models':
      return <ModelsPage onPageChange={setCurrentPage} />;
    case 'settings':
      return <SettingsPage onPageChange={setCurrentPage} />;
    case 'chat':
    default:
      return <ChatPage onPageChange={setCurrentPage} />;
  }
}

export default App;
