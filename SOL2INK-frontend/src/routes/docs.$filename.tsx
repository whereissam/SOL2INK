import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useState } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter'
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { ArrowLeft, Download, ExternalLink } from 'lucide-react'
import { Link } from '@tanstack/react-router'
import { useTheme } from '@/components/theme-provider'

interface CodeProps {
  node?: any
  inline?: boolean
  className?: string
  children?: React.ReactNode
}

function MarkdownViewer() {
  const { filename } = Route.useParams()
  const [content, setContent] = useState('')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const { theme } = useTheme()
  const isDark = theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)

  useEffect(() => {
    const fetchMarkdown = async () => {
      try {
        setLoading(true)
        setError(null)
        const response = await fetch(`/docs/${filename}`)
        if (!response.ok) {
          throw new Error(`Failed to fetch ${filename}`)
        }
        const text = await response.text()
        setContent(text)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load document')
      } finally {
        setLoading(false)
      }
    }

    fetchMarkdown()
  }, [filename])

  const getDocumentTitle = (filename: string) => {
    const titleMap: Record<string, string> = {
      'SOLIDITY_TO_INK_TUTORIAL.md': 'Solidity to ink! Tutorial',
      'migration_guide_simple_storage.md': 'Simple Storage Migration Guide',
      'migration_guide_flipper.md': 'Flipper Contract Migration Guide',
      'migration_guide_counter.md': 'Counter Contract Migration Guide',
      'migration_guide_erc20.md': 'ERC20 Token Migration Guide',
      'migration_guide_erc721_nft.md': 'ERC721 NFT Migration Guide',
      'migration_guide_erc1155.md': 'ERC1155 Multi-Token Migration Guide',
      'migration_guide_multisig_wallet.md': 'Multisig Wallet Migration Guide',
      'migration_guide_escrow_vesting.md': 'Escrow & Vesting Migration Guide',
      'migration_guide_event_emitter.md': 'Event Emitter Migration Guide',
    }
    return titleMap[filename] || filename.replace(/\.md$/, '').replace(/_/g, ' ')
  }

  const CodeBlock = ({ inline, className, children, ...props }: CodeProps) => {
    const match = /language-(\w+)/.exec(className || '')
    return !inline && match ? (
      <SyntaxHighlighter
        style={isDark ? oneDark : undefined}
        language={match[1]}
        PreTag="div"
        className="rounded-md"
        {...props}
      >
        {String(children).replace(/\n$/, '')}
      </SyntaxHighlighter>
    ) : (
      <code className={`${className} bg-muted px-1 py-0.5 rounded text-sm font-mono`} {...props}>
        {children}
      </code>
    )
  }

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
            <p className="text-muted-foreground">Loading document...</p>
          </div>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <Card className="w-full max-w-md">
            <CardHeader>
              <CardTitle className="text-destructive">Error Loading Document</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground mb-4">{error}</p>
              <div className="flex gap-2">
                <Button asChild variant="outline">
                  <Link to="/migrations">
                    <ArrowLeft className="h-4 w-4 mr-2" />
                    Back to Migrations
                  </Link>
                </Button>
                <Button onClick={() => window.location.reload()}>
                  Try Again
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-6">
        <div className="flex items-center justify-between mb-4">
          <Button asChild variant="outline" size="sm">
            <Link to="/migrations">
              <ArrowLeft className="h-4 w-4 mr-2" />
              Back to Migrations
            </Link>
          </Button>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                const element = document.createElement('a')
                element.href = `/docs/${filename}`
                element.download = filename
                element.click()
              }}
            >
              <Download className="h-4 w-4 mr-2" />
              Download
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => window.open(`/docs/${filename}`, '_blank')}
            >
              <ExternalLink className="h-4 w-4 mr-2" />
              Raw
            </Button>
          </div>
        </div>
        <h1 className="text-3xl font-bold">{getDocumentTitle(filename)}</h1>
      </div>

      <div className="space-y-6">
        {/* Table of Contents */}
        <Card className="bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-blue-950/20 dark:to-indigo-950/20 border-blue-200 dark:border-blue-800">
          <CardHeader className="pb-3">
            <CardTitle className="text-lg font-semibold text-blue-900 dark:text-blue-100 flex items-center gap-2">
              📑 Quick Navigation
            </CardTitle>
          </CardHeader>
          <CardContent className="pt-0">
            <p className="text-sm text-blue-700 dark:text-blue-300">
              This comprehensive tutorial covers migrating from Solidity to ink! with practical examples and best practices.
            </p>
          </CardContent>
        </Card>

        {/* Main Content */}
        <Card className="shadow-sm">
          <CardContent className="p-8">
            <div className="prose prose-lg prose-gray dark:prose-invert max-w-none">
              <ReactMarkdown
                remarkPlugins={[remarkGfm]}
                components={{
                  code: CodeBlock,
                  pre: ({ children }) => <div className="not-prose my-6">{children}</div>,
                  
                  // Enhanced headings with better spacing and styling
                  h1: ({ children }) => (
                    <h1 className="text-4xl font-bold mb-8 mt-12 first:mt-0 text-gray-900 dark:text-gray-100 border-b-2 border-gray-200 dark:border-gray-700 pb-4">
                      {children}
                    </h1>
                  ),
                  h2: ({ children }) => (
                    <h2 className="text-3xl font-semibold mb-6 mt-12 first:mt-0 text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-700 pb-3">
                      {children}
                    </h2>
                  ),
                  h3: ({ children }) => (
                    <h3 className="text-2xl font-semibold mb-4 mt-8 first:mt-0 text-gray-800 dark:text-gray-200">
                      {children}
                    </h3>
                  ),
                  h4: ({ children }) => (
                    <h4 className="text-xl font-semibold mb-3 mt-6 first:mt-0 text-gray-700 dark:text-gray-300">
                      {children}
                    </h4>
                  ),
                  
                  // Enhanced paragraphs with better spacing
                  p: ({ children }) => (
                    <p className="mb-6 leading-relaxed text-gray-700 dark:text-gray-300 text-base">
                      {children}
                    </p>
                  ),
                  
                  // Enhanced lists with better spacing
                  ul: ({ children }) => (
                    <ul className="mb-6 space-y-2 pl-6 list-disc text-gray-700 dark:text-gray-300">
                      {children}
                    </ul>
                  ),
                  ol: ({ children }) => (
                    <ol className="mb-6 space-y-2 pl-6 list-decimal text-gray-700 dark:text-gray-300">
                      {children}
                    </ol>
                  ),
                  li: ({ children }) => (
                    <li className="leading-relaxed mb-2 ml-2">{children}</li>
                  ),
                  
                  // Enhanced tables with better styling
                  table: ({ children }) => (
                    <div className="overflow-x-auto my-8 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm">
                      <table className="w-full border-collapse bg-white dark:bg-gray-800">
                        {children}
                      </table>
                    </div>
                  ),
                  thead: ({ children }) => (
                    <thead className="bg-gray-50 dark:bg-gray-700">
                      {children}
                    </thead>
                  ),
                  th: ({ children }) => (
                    <th className="border-b border-gray-200 dark:border-gray-600 px-6 py-4 text-left font-semibold text-gray-900 dark:text-gray-100">
                      {children}
                    </th>
                  ),
                  td: ({ children }) => (
                    <td className="border-b border-gray-100 dark:border-gray-700 px-6 py-4 text-gray-700 dark:text-gray-300">
                      {children}
                    </td>
                  ),
                  
                  // Enhanced blockquotes with icons and better styling
                  blockquote: ({ children }) => (
                    <blockquote className="border-l-4 border-blue-500 bg-blue-50 dark:bg-blue-950/30 pl-6 pr-4 py-4 my-6 rounded-r-lg">
                      <div className="flex items-start gap-3">
                        <span className="text-blue-500 text-lg">💡</span>
                        <div className="text-blue-800 dark:text-blue-200 italic font-medium">
                          {children}
                        </div>
                      </div>
                    </blockquote>
                  ),
                  
                  // Enhanced links
                  a: ({ href, children }) => (
                    <a
                      href={href}
                      className="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 underline decoration-2 underline-offset-2 transition-colors"
                      target={href?.startsWith('http') ? '_blank' : undefined}
                      rel={href?.startsWith('http') ? 'noopener noreferrer' : undefined}
                    >
                      {children}
                    </a>
                  ),
                  
                  // Horizontal rule styling
                  hr: () => (
                    <hr className="my-12 border-0 h-px bg-gradient-to-r from-transparent via-gray-300 dark:via-gray-600 to-transparent" />
                  ),
                  
                  // Enhanced strong/bold text
                  strong: ({ children }) => (
                    <strong className="font-bold text-gray-900 dark:text-gray-100">
                      {children}
                    </strong>
                  ),
                  
                  // Enhanced emphasis/italic text
                  em: ({ children }) => (
                    <em className="italic text-gray-800 dark:text-gray-200">
                      {children}
                    </em>
                  ),
                }}
              >
                {content}
              </ReactMarkdown>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}

export const Route = createFileRoute('/docs/$filename')({
  component: MarkdownViewer,
})