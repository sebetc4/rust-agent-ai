import { Component, ErrorInfo, ReactNode } from 'react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { AlertCircle } from 'lucide-react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
    errorInfo: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, errorInfo: null };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary caught an error:', error, errorInfo);
    this.setState({
      error,
      errorInfo,
    });
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="min-h-screen bg-background flex items-center justify-center p-4">
          <Card className="p-8 max-w-2xl">
            <div className="flex items-start gap-4">
              <AlertCircle className="h-6 w-6 text-destructive flex-shrink-0 mt-0.5" />
              <div className="flex-1">
                <h2 className="text-lg font-semibold mb-2">Something went wrong</h2>
                <p className="text-sm text-muted-foreground mb-4">
                  The application encountered an unexpected error:
                </p>
                {this.state.error && (
                  <pre className="text-xs bg-muted p-3 rounded overflow-auto mb-4">
                    {this.state.error.toString()}
                  </pre>
                )}
                {this.state.errorInfo && (
                  <details className="text-xs">
                    <summary className="cursor-pointer mb-2 font-medium">
                      Component Stack
                    </summary>
                    <pre className="bg-muted p-3 rounded overflow-auto">
                      {this.state.errorInfo.componentStack}
                    </pre>
                  </details>
                )}
                <Button
                  onClick={() => {
                    this.setState({ hasError: false, error: null, errorInfo: null });
                    window.location.reload();
                  }}
                  className="mt-4"
                >
                  Reload Application
                </Button>
              </div>
            </div>
          </Card>
        </div>
      );
    }

    return this.props.children;
  }
}
