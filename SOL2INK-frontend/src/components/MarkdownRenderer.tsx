import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { oneDark, oneLight } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { cn } from '@/lib/utils';
import { useTheme } from '@/components/theme-provider';

interface MarkdownRendererProps {
  content: string;
  className?: string;
}

interface CodeProps {
  node?: any;
  inline?: boolean;
  className?: string;
  children?: React.ReactNode;
}

export function MarkdownRenderer({ content, className }: MarkdownRendererProps) {
  const { theme } = useTheme();
  const isDark = theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

  const CodeBlock = ({ inline, className, children, ...props }: CodeProps) => {
    const match = /language-(\w+)/.exec(className || '');
    return !inline && match ? (
      <SyntaxHighlighter
        style={isDark ? oneDark : oneLight}
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
    );
  };

  return (
    <div className={cn("prose prose-lg prose-gray dark:prose-invert max-w-none", className)}>
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        components={{
          code: CodeBlock,
          pre: ({ children }) => <div className="not-prose my-6">{children}</div>,
          // Enhanced headings with better spacing and mobile-responsive styling
          h1: ({ children }) => (
            <h1 className="text-xl sm:text-2xl lg:text-3xl font-bold mb-4 sm:mb-6 mt-6 sm:mt-8 first:mt-0 text-gray-900 dark:text-gray-100 border-b-2 border-gray-200 dark:border-gray-700 pb-2 sm:pb-3 break-words">
              {children}
            </h1>
          ),
          h2: ({ children }) => (
            <h2 className="text-lg sm:text-xl lg:text-2xl font-semibold mb-3 sm:mb-4 mt-6 sm:mt-8 first:mt-0 text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-700 pb-2 break-words">
              {children}
            </h2>
          ),
          h3: ({ children }) => (
            <h3 className="text-base sm:text-lg lg:text-xl font-semibold mb-2 sm:mb-3 mt-4 sm:mt-6 first:mt-0 text-gray-800 dark:text-gray-200 break-words">
              {children}
            </h3>
          ),
          h4: ({ children }) => (
            <h4 className="text-base sm:text-lg font-semibold mb-2 mt-3 sm:mt-4 first:mt-0 text-gray-700 dark:text-gray-300 break-words">
              {children}
            </h4>
          ),
          
          // Enhanced paragraphs with better spacing
          p: ({ children }) => (
            <p className="mb-4 leading-relaxed text-gray-700 dark:text-gray-300 text-base">
              {children}
            </p>
          ),
          
          // Enhanced lists with better spacing - FIXED list positioning
          ul: ({ children }) => (
            <ul className="mb-4 space-y-1 pl-6 list-disc text-gray-700 dark:text-gray-300">
              {children}
            </ul>
          ),
          ol: ({ children }) => (
            <ol className="mb-4 space-y-1 pl-6 list-decimal text-gray-700 dark:text-gray-300">
              {children}
            </ol>
          ),
          li: ({ children }) => (
            <li className="leading-relaxed mb-1 ml-2">{children}</li>
          ),
          
          // Enhanced tables with better mobile styling
          table: ({ children }) => (
            <div className="overflow-x-auto my-4 sm:my-6 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm">
              <table className="w-full min-w-[300px] border-collapse bg-white dark:bg-gray-800 text-sm sm:text-base">
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
            <th className="border-b border-gray-200 dark:border-gray-600 px-2 sm:px-4 py-2 sm:py-3 text-left font-semibold text-gray-900 dark:text-gray-100 text-xs sm:text-sm">
              {children}
            </th>
          ),
          td: ({ children }) => (
            <td className="border-b border-gray-100 dark:border-gray-700 px-2 sm:px-4 py-2 sm:py-3 text-gray-700 dark:text-gray-300 text-xs sm:text-sm break-words">
              {children}
            </td>
          ),
          
          // Enhanced blockquotes with icons and better styling
          blockquote: ({ children }) => (
            <blockquote className="border-l-4 border-blue-500 bg-blue-50 dark:bg-blue-950/30 pl-4 pr-3 py-3 my-4 rounded-r-lg">
              <div className="flex items-start gap-2">
                <span className="text-blue-500 text-sm">💡</span>
                <div className="text-blue-800 dark:text-blue-200 italic font-medium text-sm">
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
            <hr className="my-8 border-0 h-px bg-gradient-to-r from-transparent via-gray-300 dark:via-gray-600 to-transparent" />
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
  );
}