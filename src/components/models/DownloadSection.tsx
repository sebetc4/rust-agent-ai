import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { CheckCircle2, Download, Loader2, XCircle } from 'lucide-react';

interface DownloadSectionProps {
  isDownloading: boolean;
  downloadProgress: number;
  downloadStatus: {
    success: boolean;
    message: string;
    path?: string;
  } | null;
  onDownload: () => void;
}

export function DownloadSection({
  isDownloading,
  downloadProgress,
  downloadStatus,
  onDownload,
}: DownloadSectionProps) {
  return (
    <div className="pt-4 border-t space-y-3">
      <Button onClick={onDownload} disabled={isDownloading} className="w-full">
        {isDownloading ? (
          <>
            <Loader2 className="h-4 w-4 mr-2 animate-spin" />
            Downloading...
          </>
        ) : (
          <>
            <Download className="h-4 w-4 mr-2" />
            Download Selected File
          </>
        )}
      </Button>

      {isDownloading && (
        <div className="space-y-2">
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>Progress</span>
            <span>{downloadProgress}%</span>
          </div>
          <Progress value={downloadProgress} className="h-2" />
        </div>
      )}

      {downloadStatus && (
        <Card
          className={`p-3 ${
            downloadStatus.success
              ? 'bg-green-50 dark:bg-green-950 border-green-200 dark:border-green-800'
              : 'bg-red-50 dark:bg-red-950 border-red-200 dark:border-red-800'
          }`}
        >
          <div className="flex items-start gap-2">
            {downloadStatus.success ? (
              <CheckCircle2 className="h-5 w-5 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
            ) : (
              <XCircle className="h-5 w-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
            )}
            <div className="flex-1 min-w-0">
              <p
                className={`text-sm font-medium ${
                  downloadStatus.success
                    ? 'text-green-900 dark:text-green-100'
                    : 'text-red-900 dark:text-red-100'
                }`}
              >
                {downloadStatus.message}
              </p>
              {downloadStatus.path && (
                <p className="text-xs text-green-700 dark:text-green-300 mt-1 truncate">
                  {downloadStatus.path}
                </p>
              )}
            </div>
          </div>
        </Card>
      )}
    </div>
  );
}
