import { createFileRoute, Link } from '@tanstack/react-router'
import { useEffect, useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { ExternalLink, FileText, Code, BookOpen } from 'lucide-react'

interface MigrationGuide {
  title: string
  filename: string
  description: string
  category: 'tutorial' | 'guide' | 'example'
  difficulty: 'beginner' | 'intermediate' | 'advanced'
}

const migrationGuides: MigrationGuide[] = [
  {
    title: 'Solidity to ink! Tutorial',
    filename: 'SOLIDITY_TO_INK_TUTORIAL.md',
    description: 'Complete tutorial for migrating from Solidity to ink! smart contracts',
    category: 'tutorial',
    difficulty: 'beginner'
  },
  {
    title: 'Simple Storage Migration',
    filename: 'migration_guide_simple_storage.md',
    description: 'Basic storage contract migration example',
    category: 'guide',
    difficulty: 'beginner'
  },
  {
    title: 'Flipper Contract Migration',
    filename: 'migration_guide_flipper.md',
    description: 'Boolean state toggle contract migration',
    category: 'guide',
    difficulty: 'beginner'
  },
  {
    title: 'Counter Contract Migration',
    filename: 'migration_guide_counter.md',
    description: 'Simple counter contract with increment/decrement',
    category: 'guide',
    difficulty: 'beginner'
  },
  {
    title: 'ERC20 Token Migration',
    filename: 'migration_guide_erc20.md',
    description: 'Fungible token standard migration guide',
    category: 'guide',
    difficulty: 'intermediate'
  },
  {
    title: 'ERC721 NFT Migration',
    filename: 'migration_guide_erc721_nft.md',
    description: 'Non-fungible token standard migration',
    category: 'guide',
    difficulty: 'intermediate'
  },
  {
    title: 'ERC1155 Multi-Token Migration',
    filename: 'migration_guide_erc1155.md',
    description: 'Multi-token standard migration guide',
    category: 'guide',
    difficulty: 'advanced'
  },
  {
    title: 'Multisig Wallet Migration',
    filename: 'migration_guide_multisig_wallet.md',
    description: 'Multi-signature wallet contract migration',
    category: 'guide',
    difficulty: 'advanced'
  },
  {
    title: 'Escrow & Vesting Migration',
    filename: 'migration_guide_escrow_vesting.md',
    description: 'Time-locked token release mechanisms',
    category: 'guide',
    difficulty: 'advanced'
  },
  {
    title: 'Event Emitter Migration',
    filename: 'migration_guide_event_emitter.md',
    description: 'Event logging and emission patterns',
    category: 'guide',
    difficulty: 'intermediate'
  }
]

function MigrationGuides() {
  const [guides, setGuides] = useState<MigrationGuide[]>([])
  const [selectedCategory, setSelectedCategory] = useState<string>('all')
  const [selectedDifficulty, setSelectedDifficulty] = useState<string>('all')

  useEffect(() => {
    setGuides(migrationGuides)
  }, [])

  const filteredGuides = guides.filter(guide => {
    const categoryMatch = selectedCategory === 'all' || guide.category === selectedCategory
    const difficultyMatch = selectedDifficulty === 'all' || guide.difficulty === selectedDifficulty
    return categoryMatch && difficultyMatch
  })

  const getDifficultyColor = (difficulty: string) => {
    switch (difficulty) {
      case 'beginner': return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100'
      case 'intermediate': return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-100'
      case 'advanced': return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-100'
      default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-100'
    }
  }

  const getCategoryIcon = (category: string) => {
    switch (category) {
      case 'tutorial': return <BookOpen className="h-4 w-4" />
      case 'guide': return <FileText className="h-4 w-4" />
      case 'example': return <Code className="h-4 w-4" />
      default: return <FileText className="h-4 w-4" />
    }
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-4">Migration Guides</h1>
        <p className="text-muted-foreground mb-6">
          Comprehensive guides for migrating Solidity smart contracts to ink! on Polkadot
        </p>
        
        <div className="flex flex-wrap gap-4 mb-6">
          <div className="flex items-center space-x-2">
            <label className="text-sm font-medium">Category:</label>
            <select 
              value={selectedCategory} 
              onChange={(e) => setSelectedCategory(e.target.value)}
              className="px-3 py-1 border rounded-md text-sm bg-background"
            >
              <option value="all">All</option>
              <option value="tutorial">Tutorials</option>
              <option value="guide">Guides</option>
              <option value="example">Examples</option>
            </select>
          </div>
          
          <div className="flex items-center space-x-2">
            <label className="text-sm font-medium">Difficulty:</label>
            <select 
              value={selectedDifficulty} 
              onChange={(e) => setSelectedDifficulty(e.target.value)}
              className="px-3 py-1 border rounded-md text-sm bg-background"
            >
              <option value="all">All</option>
              <option value="beginner">Beginner</option>
              <option value="intermediate">Intermediate</option>
              <option value="advanced">Advanced</option>
            </select>
          </div>
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {filteredGuides.map((guide, index) => (
          <Card key={index} className="hover:shadow-lg transition-shadow">
            <CardHeader>
              <div className="flex items-start justify-between">
                <div className="flex items-center space-x-2">
                  {getCategoryIcon(guide.category)}
                  <CardTitle className="text-lg">{guide.title}</CardTitle>
                </div>
                <Badge className={getDifficultyColor(guide.difficulty)}>
                  {guide.difficulty}
                </Badge>
              </div>
              <CardDescription>{guide.description}</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-between">
                <Badge variant="outline" className="text-xs">
                  {guide.category}
                </Badge>
                <Button
                  variant="outline"
                  size="sm"
                  asChild
                  className="flex items-center space-x-1"
                >
                  <Link to={`/docs/${guide.filename}`}>
                    <ExternalLink className="h-3 w-3" />
                    <span>View Guide</span>
                  </Link>
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {filteredGuides.length === 0 && (
        <div className="text-center py-12">
          <FileText className="h-16 w-16 text-muted-foreground mx-auto mb-4" />
          <h3 className="text-lg font-semibold mb-2">No guides found</h3>
          <p className="text-muted-foreground">
            No migration guides match your current filters. Try adjusting the category or difficulty filters.
          </p>
        </div>
      )}
    </div>
  )
}

export const Route = createFileRoute('/migrations')({
  component: MigrationGuides,
})