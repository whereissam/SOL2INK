import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { MarkdownRenderer } from '@/components/MarkdownRenderer';
import { 
  FileText, 
  BookOpen, 
  ArrowRight, 
  Code, 
  Coins,
  Shield,
  Loader2,
  Search
} from 'lucide-react';

interface MigrationGuide {
  id: string;
  title: string;
  description: string;
  category: string;
  difficulty: 'Beginner' | 'Intermediate' | 'Advanced';
  filename: string;
  content?: string;
}

const migrationGuides: MigrationGuide[] = [
  {
    id: 'flipper',
    title: 'Flipper Contract',
    description: 'Simple boolean storage contract - perfect for beginners',
    category: 'Basic',
    difficulty: 'Beginner',
    filename: 'migration_guide_flipper.md'
  },
  {
    id: 'counter',
    title: 'Counter Contract',
    description: 'Increment/decrement counter with access control',
    category: 'Basic',
    difficulty: 'Beginner',
    filename: 'migration_guide_counter.md'
  },
  {
    id: 'simple-storage',
    title: 'Simple Storage',
    description: 'Basic data storage and retrieval patterns',
    category: 'Storage',
    difficulty: 'Beginner',
    filename: 'migration_guide_simple_storage.md'
  },
  {
    id: 'erc20',
    title: 'ERC20 Token',
    description: 'Fungible token standard implementation',
    category: 'Token',
    difficulty: 'Intermediate',
    filename: 'migration_guide_erc20.md'
  },
  {
    id: 'erc721',
    title: 'ERC721 NFT',
    description: 'Non-fungible token standard implementation',
    category: 'Token',
    difficulty: 'Intermediate',
    filename: 'migration_guide_erc721_nft.md'
  },
  {
    id: 'erc1155',
    title: 'ERC1155 Multi-Token',
    description: 'Multi-token standard for both fungible and non-fungible tokens',
    category: 'Token',
    difficulty: 'Advanced',
    filename: 'migration_guide_erc1155.md'
  },
  {
    id: 'multisig',
    title: 'Multisig Wallet',
    description: 'Multi-signature wallet with approval mechanism',
    category: 'Security',
    difficulty: 'Advanced',
    filename: 'migration_guide_multisig_wallet.md'
  },
  {
    id: 'escrow',
    title: 'Escrow & Vesting',
    description: 'Time-locked asset management and vesting schedules',
    category: 'DeFi',
    difficulty: 'Advanced',
    filename: 'migration_guide_escrow_vesting.md'
  },
  {
    id: 'events',
    title: 'Event Emitter',
    description: 'Event emission and listening patterns',
    category: 'Events',
    difficulty: 'Intermediate',
    filename: 'migration_guide_event_emitter.md'
  },
  {
    id: 'tutorial',
    title: 'Complete Tutorial',
    description: 'Comprehensive Solidity to ink! migration guide',
    category: 'Tutorial',
    difficulty: 'Beginner',
    filename: 'SOLIDITY_TO_INK_TUTORIAL.md'
  }
];

const categoryIcons = {
  'Basic': <Code className="w-4 h-4" />,
  'Storage': <FileText className="w-4 h-4" />,
  'Token': <Coins className="w-4 h-4" />,
  'Security': <Shield className="w-4 h-4" />,
  'DeFi': <ArrowRight className="w-4 h-4" />,
  'Events': <BookOpen className="w-4 h-4" />,
  'Tutorial': <BookOpen className="w-4 h-4" />
};

const difficultyColors = {
  'Beginner': 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200',
  'Intermediate': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200',
  'Advanced': 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'
};

export function MigrationGuides() {
  const [selectedGuide, setSelectedGuide] = useState<MigrationGuide | null>(null);
  const [guideContent, setGuideContent] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<string>('All');

  const categories = ['All', ...Array.from(new Set(migrationGuides.map(g => g.category)))];

  const filteredGuides = migrationGuides.filter(guide => {
    const matchesSearch = guide.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         guide.description.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesCategory = selectedCategory === 'All' || guide.category === selectedCategory;
    return matchesSearch && matchesCategory;
  });

  const loadGuideContent = async (guide: MigrationGuide) => {
    setLoading(true);
    setSelectedGuide(guide);
    
    try {
      // Try to fetch from the backend first
      const response = await fetch(`/api/migrations/${guide.filename}`);
      if (response.ok) {
        const content = await response.text();
        setGuideContent(content);
      } else {
        // Fallback to a sample content
        setGuideContent(generateSampleContent(guide));
      }
    } catch (error) {
      console.error('Error loading guide:', error);
      setGuideContent(generateSampleContent(guide));
    } finally {
      setLoading(false);
    }
  };

  const generateSampleContent = (guide: MigrationGuide) => {
    return `# ${guide.title} Migration Guide

## Overview
${guide.description}

## Key Differences

### Solidity Implementation
\`\`\`solidity
// Solidity example for ${guide.title}
// This is a placeholder - actual content will be loaded from the migration files
\`\`\`

### ink! Implementation
\`\`\`rust
// ink! example for ${guide.title}
// This is a placeholder - actual content will be loaded from the migration files
\`\`\`

## Migration Steps

1. **Setup**: Initialize your ink! project
2. **Structure**: Convert contract structure
3. **Logic**: Implement business logic
4. **Testing**: Write comprehensive tests
5. **Deployment**: Deploy to Substrate chain

## Best Practices

- Use Rust's type system for safety
- Implement proper error handling
- Write comprehensive tests
- Follow ink! conventions

## Additional Resources

- [ink! Documentation](https://use.ink/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot Documentation](https://polkadot.network/development/)
`;
  };

  return (
    <div className="max-w-7xl mx-auto p-6">
      <div className="text-center mb-8">
        <h1 className="text-4xl font-bold mb-4">Migration Guides</h1>
        <p className="text-lg text-muted-foreground max-w-2xl mx-auto">
          Step-by-step guides for migrating your Solidity contracts to ink!. 
          Choose from our collection of practical examples and tutorials.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Sidebar */}
        <div className="lg:col-span-1">
          <div className="sticky top-6 space-y-4">
            {/* Search */}
            <div className="relative">
              <Search className="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
              <input
                type="text"
                placeholder="Search guides..."
                className="w-full pl-10 pr-4 py-2 border border-border rounded-lg bg-background"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
            </div>

            {/* Category Filter */}
            <div className="flex flex-wrap gap-2">
              {categories.map(category => (
                <button
                  key={category}
                  onClick={() => setSelectedCategory(category)}
                  className={`px-3 py-1 rounded-full text-sm transition-colors ${
                    selectedCategory === category
                      ? 'bg-primary text-primary-foreground'
                      : 'bg-muted hover:bg-muted/80'
                  }`}
                >
                  {category}
                </button>
              ))}
            </div>

            {/* Guide List */}
            <div className="space-y-3">
              {filteredGuides.map(guide => (
                <Card 
                  key={guide.id}
                  className={`cursor-pointer transition-all hover:shadow-md ${
                    selectedGuide?.id === guide.id ? 'ring-2 ring-primary' : ''
                  }`}
                  onClick={() => loadGuideContent(guide)}
                >
                  <CardHeader className="pb-3">
                    <div className="flex items-start gap-3">
                      <div className="p-2 bg-muted rounded-lg">
                        {categoryIcons[guide.category as keyof typeof categoryIcons]}
                      </div>
                      <div className="flex-1 min-w-0">
                        <CardTitle className="text-base line-clamp-1">
                          {guide.title}
                        </CardTitle>
                        <CardDescription className="text-sm line-clamp-2">
                          {guide.description}
                        </CardDescription>
                      </div>
                    </div>
                    <div className="flex items-center gap-2 pt-2">
                      <Badge variant="secondary" className="text-xs">
                        {guide.category}
                      </Badge>
                      <Badge 
                        className={`text-xs ${difficultyColors[guide.difficulty]}`}
                      >
                        {guide.difficulty}
                      </Badge>
                    </div>
                  </CardHeader>
                </Card>
              ))}
            </div>
          </div>
        </div>

        {/* Main Content */}
        <div className="lg:col-span-2">
          {selectedGuide ? (
            <Card className="h-full">
              <CardHeader>
                <div className="flex items-center gap-3">
                  <div className="p-2 bg-muted rounded-lg">
                    {categoryIcons[selectedGuide.category as keyof typeof categoryIcons]}
                  </div>
                  <div>
                    <CardTitle className="text-xl">{selectedGuide.title}</CardTitle>
                    <CardDescription>{selectedGuide.description}</CardDescription>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant="secondary">{selectedGuide.category}</Badge>
                  <Badge 
                    className={difficultyColors[selectedGuide.difficulty]}
                  >
                    {selectedGuide.difficulty}
                  </Badge>
                </div>
              </CardHeader>
              <CardContent>
                {loading ? (
                  <div className="flex items-center justify-center py-12">
                    <Loader2 className="w-8 h-8 animate-spin" />
                    <span className="ml-2">Loading migration guide...</span>
                  </div>
                ) : (
                  <div className="prose prose-slate dark:prose-invert max-w-none">
                    <MarkdownRenderer content={guideContent} />
                  </div>
                )}
              </CardContent>
            </Card>
          ) : (
            <Card className="h-full flex items-center justify-center">
              <CardContent className="text-center py-12">
                <BookOpen className="w-16 h-16 text-muted-foreground mx-auto mb-4" />
                <h3 className="text-lg font-semibold mb-2">Select a Migration Guide</h3>
                <p className="text-muted-foreground">
                  Choose a guide from the left sidebar to start learning how to migrate your contracts.
                </p>
              </CardContent>
            </Card>
          )}
        </div>
      </div>
    </div>
  );
}