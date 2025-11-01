import { ModelSelector } from '@/components/settings/ModelSelector';

export function Settings() {
  return (
    <div className="container mx-auto max-w-4xl space-y-8 p-6">
      <div>
        <h1 className="text-3xl font-bold">Settings</h1>
        <p className="text-muted-foreground mt-2">
          Configure your LLM models and application settings
        </p>
      </div>

      <ModelSelector />
    </div>
  );
}
