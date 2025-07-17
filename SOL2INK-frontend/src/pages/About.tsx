import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { 
  Code, 
  BookOpen, 
  Zap, 
  Shield, 
  Users, 
  Database,
  Brain,
  ArrowRight
} from 'lucide-react';

export function About() {
  return (
    <div className="max-w-4xl mx-auto p-6 space-y-8">
      {/* Header */}
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold">About SOL2INK Migration Assistant</h1>
        <p className="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
          An AI-powered tool to help developers seamlessly migrate smart contracts from Solidity to ink!
        </p>
      </div>

      {/* Features */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Brain className="w-5 h-5 text-blue-500" />
              AI-Powered Guidance
            </CardTitle>
            <CardDescription>
              RAG-based system with 180+ embedded code examples and migration guides
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              Our system uses Retrieval-Augmented Generation to provide contextual, accurate migration advice 
              based on real code examples from both Solidity and ink! ecosystems.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Code className="w-5 h-5 text-green-500" />
              Code Examples
            </CardTitle>
            <CardDescription>
              Side-by-side comparisons with working implementations
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              Get detailed before/after code examples showing how Solidity patterns translate to ink!, 
              with syntax highlighting and best practices included.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <BookOpen className="w-5 h-5 text-purple-500" />
              Migration Guides
            </CardTitle>
            <CardDescription>
              10+ comprehensive guides covering common contract patterns
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              From simple contracts like Flipper to complex patterns like ERC1155 and Multisig wallets, 
              our guides cover the most important migration scenarios.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Zap className="w-5 h-5 text-yellow-500" />
              Fast Responses
            </CardTitle>
            <CardDescription>
              Sub-second search with intelligent caching
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              Powered by Qdrant vector database and Rust backend for lightning-fast semantic search 
              through migration knowledge base.
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Technology Stack */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Database className="w-5 h-5 text-indigo-500" />
            Technology Stack
          </CardTitle>
          <CardDescription>
            Built with modern, high-performance technologies
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="text-center space-y-2">
              <div className="w-12 h-12 bg-orange-100 dark:bg-orange-900 rounded-lg flex items-center justify-center mx-auto">
                <Code className="w-6 h-6 text-orange-600" />
              </div>
              <div>
                <p className="font-medium">Rust Backend</p>
                <p className="text-xs text-gray-500">Axum + Shuttle</p>
              </div>
            </div>
            <div className="text-center space-y-2">
              <div className="w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center mx-auto">
                <Database className="w-6 h-6 text-blue-600" />
              </div>
              <div>
                <p className="font-medium">Qdrant</p>
                <p className="text-xs text-gray-500">Vector Database</p>
              </div>
            </div>
            <div className="text-center space-y-2">
              <div className="w-12 h-12 bg-purple-100 dark:bg-purple-900 rounded-lg flex items-center justify-center mx-auto">
                <Brain className="w-6 h-6 text-purple-600" />
              </div>
              <div>
                <p className="font-medium">Gemini API</p>
                <p className="text-xs text-gray-500">AI Generation</p>
              </div>
            </div>
            <div className="text-center space-y-2">
              <div className="w-12 h-12 bg-cyan-100 dark:bg-cyan-900 rounded-lg flex items-center justify-center mx-auto">
                <ArrowRight className="w-6 h-6 text-cyan-600" />
              </div>
              <div>
                <p className="font-medium">React</p>
                <p className="text-xs text-gray-500">Frontend</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Supported Patterns */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Shield className="w-5 h-5 text-green-500" />
            Supported Migration Patterns
          </CardTitle>
          <CardDescription>
            Contract types and patterns we can help you migrate
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-2">
            {[
              'Basic Contracts',
              'ERC20 Tokens', 
              'ERC721 NFTs',
              'ERC1155 Multi-tokens',
              'Multisig Wallets',
              'Event Handling',
              'Storage Patterns',
              'Access Control',
              'Escrow Contracts',
              'Vesting Contracts',
              'Error Handling',
              'Testing Patterns'
            ].map((pattern) => (
              <Badge key={pattern} variant="secondary" className="text-xs">
                {pattern}
              </Badge>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* How it Works */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Users className="w-5 h-5 text-pink-500" />
            How It Works
          </CardTitle>
          <CardDescription>
            The migration assistant process in simple steps
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
                1
              </div>
              <div>
                <p className="font-medium">Ask Your Question</p>
                <p className="text-sm text-gray-600 dark:text-gray-300">
                  Type any migration question - from basic concepts to complex contract patterns
                </p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
                2
              </div>
              <div>
                <p className="font-medium">Semantic Search</p>
                <p className="text-sm text-gray-600 dark:text-gray-300">
                  The system searches through 180+ embedded examples to find relevant content
                </p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
                3
              </div>
              <div>
                <p className="font-medium">AI Generation</p>
                <p className="text-sm text-gray-600 dark:text-gray-300">
                  AI generates comprehensive guidance with code examples and best practices
                </p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold">
                4
              </div>
              <div>
                <p className="font-medium">Formatted Response</p>
                <p className="text-sm text-gray-600 dark:text-gray-300">
                  Receive beautifully formatted markdown with syntax highlighting and tables
                </p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Footer */}
      <div className="text-center py-6 border-t border-gray-200 dark:border-gray-700">
        <p className="text-sm text-gray-500 dark:text-gray-400">
          Built with ❤️ for the Polkadot ecosystem • Open source migration assistant
        </p>
      </div>
    </div>
  );
}
