// Tool types
export interface Tool {
  name: string;
  description: string;
  parameters?: Record<string, any>;
}
