import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { MarkdownRenderer } from '@/components/MarkdownRenderer';
import { 
  Send, 
  Loader2, 
  Code, 
  ArrowRight, 
  BookOpen, 
  MessageSquare,
  Sparkles,
  AlertTriangle,
  RefreshCw,
  Wifi,
  WifiOff,
  Clock,
  CheckCircle
} from 'lucide-react';

interface MigrationResponse {
  success: boolean;
  data: string;
  error: string | null;
}

interface ErrorState {
  type: 'network' | 'server' | 'timeout' | 'unknown';
  message: string;
  details?: string;
  isRetryable: boolean;
}

interface ConnectionState {
  isConnected: boolean;
  isChecking: boolean;
  lastChecked?: Date;
}

const EXAMPLE_QUERIES = [
  {
    title: "ERC20 Migration",
    query: "How do I migrate ERC20 tokens from Solidity to ink!?",
    icon: <Code className="w-4 h-4" />,
    category: "Token Standards"
  },
  {
    title: "Key Differences",
    query: "What are the key differences between Solidity and ink!?",
    icon: <BookOpen className="w-4 h-4" />,
    category: "Comparison"
  },
  {
    title: "Event Handling",
    query: "Show me event handling examples in both languages",
    icon: <MessageSquare className="w-4 h-4" />,
    category: "Events"
  },
  {
    title: "Multisig Wallets",
    query: "How do I implement multisig wallets in ink!?",
    icon: <Sparkles className="w-4 h-4" />,
    category: "Advanced"
  },
  {
    title: "Storage Migration",
    query: "How do I convert Solidity mappings to ink! storage?",
    icon: <ArrowRight className="w-4 h-4" />,
    category: "Storage"
  },
  {
    title: "Flipper Contract",
    query: "How does the flipper contract work?",
    icon: <Code className="w-4 h-4" />,
    category: "Examples"
  }
];

// Configuration - can be moved to environment variables or config file
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000';
const MAX_RETRIES = parseInt(import.meta.env.VITE_MAX_RETRIES || '3');
const INITIAL_RETRY_DELAY = parseInt(import.meta.env.VITE_INITIAL_RETRY_DELAY || '1000');
const REQUEST_TIMEOUT = parseInt(import.meta.env.VITE_REQUEST_TIMEOUT || '30000');

export function MigrationAssistant() {
  const [query, setQuery] = useState('');
  const [response, setResponse] = useState<MigrationResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<ErrorState | null>(null);
  const [retryCount, setRetryCount] = useState(0);
  const [connectionState, setConnectionState] = useState<ConnectionState>({
    isConnected: false,
    isChecking: false
  });

  // Check backend connection status
  const checkConnection = async () => {
    setConnectionState(prev => ({ ...prev, isChecking: true }));
    
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 5000);
      
      const response = await fetch(`${API_BASE_URL}/health`, {
        method: 'GET',
        signal: controller.signal,
      });
      
      clearTimeout(timeoutId);
      
      setConnectionState({
        isConnected: response.ok,
        isChecking: false,
        lastChecked: new Date()
      });
      
      return response.ok;
    } catch (error) {
      setConnectionState({
        isConnected: false,
        isChecking: false,
        lastChecked: new Date()
      });
      return false;
    }
  };

  // Enhanced submit with retry logic and better error handling
  const handleSubmit = async (queryText: string = query, attempt: number = 0) => {
    if (!queryText.trim()) return;
    
    if (attempt === 0) {
      setLoading(true);
      setResponse(null);
      setError(null);
      setRetryCount(0);
    }

    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), REQUEST_TIMEOUT);
      
      const res = await fetch(`${API_BASE_URL}/ask`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ query: queryText }),
        signal: controller.signal,
      });
      
      clearTimeout(timeoutId);
      
      if (!res.ok) {
        throw new Error(`Server error: ${res.status} ${res.statusText}`);
      }

      const data: MigrationResponse = await res.json();
      setResponse(data);
      setError(null);
      setConnectionState(prev => ({ ...prev, isConnected: true }));
      
    } catch (error: any) {
      let errorState: ErrorState;
      
      if (error.name === 'AbortError') {
        errorState = {
          type: 'timeout',
          message: 'Request timed out. The server may be overloaded.',
          details: `Request took longer than ${REQUEST_TIMEOUT / 1000} seconds`,
          isRetryable: true
        };
      } else if (error.message?.includes('Failed to fetch') || error.code === 'NETWORK_ERROR') {
        errorState = {
          type: 'network',
          message: 'Cannot connect to the backend server.',
          details: 'Make sure the backend is running on localhost:8000',
          isRetryable: true
        };
        setConnectionState(prev => ({ ...prev, isConnected: false }));
      } else if (error.message?.includes('Server error')) {
        errorState = {
          type: 'server',
          message: 'Server encountered an error.',
          details: error.message,
          isRetryable: true
        };
      } else {
        errorState = {
          type: 'unknown',
          message: 'An unexpected error occurred.',
          details: error.message || 'Unknown error',
          isRetryable: true
        };
      }
      
      // Auto-retry logic for retryable errors
      if (errorState.isRetryable && attempt < MAX_RETRIES) {
        const delay = INITIAL_RETRY_DELAY * Math.pow(2, attempt); // Exponential backoff
        setRetryCount(attempt + 1);
        
        setTimeout(() => {
          handleSubmit(queryText, attempt + 1);
        }, delay);
        
        return;
      }
      
      setError(errorState);
      setResponse({
        success: false,
        data: '',
        error: errorState.message
      });
    } finally {
      if (attempt === 0 || attempt >= MAX_RETRIES) {
        setLoading(false);
      }
    }
  };

  const handleRetry = () => {
    if (query.trim()) {
      handleSubmit(query);
    }
  };

  const handleExampleClick = (exampleQuery: string) => {
    setQuery(exampleQuery);
    handleSubmit(exampleQuery);
  };

  // Check connection on component mount and periodically
  useEffect(() => {
    checkConnection();
    
    // Check connection every 30 seconds
    const interval = setInterval(checkConnection, 30000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="max-w-6xl mx-auto p-6 space-y-6">
      {/* Header with Connection Status */}
      <div className="text-center space-y-4">
        <div className="flex items-center justify-center gap-3">
          <div className="p-3 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl">
            <Code className="w-8 h-8 text-white" />
          </div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
            Solidity to ink! Migration Assistant
          </h1>
        </div>
        
        {/* Connection Status Indicator */}
        <div className="flex items-center justify-center gap-2">
          {connectionState.isChecking ? (
            <div className="flex items-center gap-2 text-gray-500">
              <Loader2 className="w-4 h-4 animate-spin" />
              <span className="text-sm">Checking connection...</span>
            </div>
          ) : connectionState.isConnected ? (
            <div className="flex items-center gap-2 text-green-600">
              <Wifi className="w-4 h-4" />
              <span className="text-sm">Backend connected</span>
            </div>
          ) : (
            <div className="flex items-center gap-2 text-red-500">
              <WifiOff className="w-4 h-4" />
              <span className="text-sm">Backend disconnected</span>
              <Button
                variant="outline"
                size="sm"
                onClick={checkConnection}
                className="ml-2 h-6 px-2 text-xs"
              >
                <RefreshCw className="w-3 h-3 mr-1" />
                Retry
              </Button>
            </div>
          )}
        </div>
        
        <p className="text-gray-600 dark:text-gray-300 text-lg max-w-2xl mx-auto">
          Get instant, AI-powered guidance for migrating your smart contracts from Solidity to ink!. 
          Ask questions and receive formatted responses with code examples, comparisons, and best practices.
        </p>
      </div>

      {/* Quick Examples */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Sparkles className="w-5 h-5 text-yellow-500" />
            Quick Examples
          </CardTitle>
          <CardDescription>
            Click any example below to see how the migration assistant works
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            {EXAMPLE_QUERIES.map((example, index) => (
              <Button
                key={index}
                variant="outline"
                className="h-auto p-4 flex flex-col items-start text-left space-y-2 hover:bg-gray-50 dark:hover:bg-gray-800"
                onClick={() => handleExampleClick(example.query)}
                disabled={loading}
              >
                <div className="flex items-center gap-2 w-full">
                  {example.icon}
                  <span className="font-medium text-sm">{example.title}</span>
                  <Badge variant="secondary" className="ml-auto text-xs">
                    {example.category}
                  </Badge>
                </div>
                <p className="text-xs text-gray-500 dark:text-gray-400 line-clamp-2">
                  {example.query}
                </p>
              </Button>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Query Input */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <MessageSquare className="w-5 h-5 text-blue-500" />
            Ask Your Migration Question
          </CardTitle>
          <CardDescription>
            Type your question about migrating from Solidity to ink! and get detailed guidance
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <Textarea
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="e.g., How do I migrate ERC721 contracts from Solidity to ink!?"
            className="min-h-[100px] resize-none"
            disabled={loading}
            onKeyDown={(e) => {
              if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
                handleSubmit();
              }
            }}
          />
          <div className="flex items-center justify-between">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Press Cmd/Ctrl + Enter to submit
            </p>
            <Button 
              onClick={() => handleSubmit()} 
              disabled={loading || !query.trim() || !connectionState.isConnected}
              className="min-w-[120px]"
            >
              {loading ? (
                <>
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                  Thinking...
                </>
              ) : (
                <>
                  <Send className="w-4 h-4 mr-2" />
                  Ask Question
                </>
              )}
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Error State with Retry */}
      {error && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <AlertTriangle className="w-5 h-5 text-red-500" />
              Connection Error
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="bg-red-50 dark:bg-red-900/20 rounded-lg p-4 border border-red-200 dark:border-red-800 space-y-4">
              <div>
                <p className="text-red-700 dark:text-red-300 font-medium">
                  {error.message}
                </p>
                {error.details && (
                  <p className="text-red-600 dark:text-red-400 text-sm mt-1">
                    {error.details}
                  </p>
                )}
              </div>
              
              {error.isRetryable && (
                <div className="flex items-center gap-3">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={handleRetry}
                    disabled={loading}
                    className="border-red-300 text-red-700 hover:bg-red-50"
                  >
                    {loading ? (
                      <>
                        <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                        Retrying...
                      </>
                    ) : (
                      <>
                        <RefreshCw className="w-4 h-4 mr-2" />
                        Try Again
                      </>
                    )}
                  </Button>
                  
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={checkConnection}
                    disabled={connectionState.isChecking}
                    className="border-blue-300 text-blue-700 hover:bg-blue-50"
                  >
                    <Wifi className="w-4 h-4 mr-2" />
                    Test Connection
                  </Button>
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Response */}
      {response && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              {response.success ? (
                <>
                  <CheckCircle className="w-5 h-5 text-green-500" />
                  Migration Guidance
                </>
              ) : (
                <>
                  <AlertTriangle className="w-5 h-5 text-red-500" />
                  Error
                </>
              )}
            </CardTitle>
            {response.success && (
              <CardDescription>
                AI-generated migration guidance with code examples and best practices
              </CardDescription>
            )}
          </CardHeader>
          <CardContent>
            {response.success ? (
              <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-6 border">
                <MarkdownRenderer content={response.data} />
              </div>
            ) : (
              <div className="bg-red-50 dark:bg-red-900/20 rounded-lg p-4 border border-red-200 dark:border-red-800">
                <p className="text-red-700 dark:text-red-300 flex items-center gap-2">
                  <AlertTriangle className="w-4 h-4" />
                  {response.error}
                </p>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Enhanced Loading State */}
      {loading && !response && (
        <Card>
          <CardContent className="flex items-center justify-center py-12">
            <div className="text-center space-y-4">
              <Loader2 className="w-8 h-8 animate-spin mx-auto text-blue-500" />
              <div className="space-y-2">
                <p className="font-medium">
                  {retryCount > 0 ? `Retrying... (${retryCount}/${MAX_RETRIES})` : 'Processing your migration question...'}
                </p>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  {retryCount > 0 
                    ? 'Previous attempt failed, trying again with exponential backoff'
                    : 'Searching through migration guides and generating a comprehensive response'
                  }
                </p>
                {retryCount > 0 && (
                  <div className="flex items-center justify-center gap-2 text-xs text-gray-400">
                    <Clock className="w-3 h-3" />
                    <span>Waiting {INITIAL_RETRY_DELAY * Math.pow(2, retryCount - 1) / 1000}s before retry</span>
                  </div>
                )}
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Footer */}
      <div className="text-center py-6 border-t border-gray-200 dark:border-gray-700">
        <div className="space-y-2">
          <p className="text-sm text-gray-500 dark:text-gray-400">
            Powered by RAG (Retrieval-Augmented Generation) with 180+ embedded code examples and migration guides
          </p>
          {connectionState.lastChecked && (
            <p className="text-xs text-gray-400">
              Last connection check: {connectionState.lastChecked.toLocaleTimeString()}
            </p>
          )}
        </div>
      </div>
    </div>
  );
}