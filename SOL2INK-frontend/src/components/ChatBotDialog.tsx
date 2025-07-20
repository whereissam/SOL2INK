import React, { useState, useRef, useEffect, useCallback, useMemo } from 'react';
import { X, Send, MessageCircle, ExternalLink, AlertCircle, ChevronDown } from 'lucide-react';
import { getApiUrl, API_CONFIG } from '../config/api';
import { useApiHealth } from '../hooks/useApiHealth';
import { useMobileOptimizations } from '../hooks/useMobileOptimizations';
import './ChatBot.css';

interface Message {
  id: string;
  type: 'user' | 'ai';
  content: string;
  timestamp: Date;
  sources?: Source[];
  followUps?: string[];
}

interface Source {
  title: string;
  url: string;
}

interface ApiResponse {
  object: string;
  data: {
    query: string;
    summary: string;
    examples: CodeExample[];
    help_text: string;
  } | string;
  error: any;
}

interface CodeExample {
  title: string;
  description?: string;
  code: string;
  source_file?: string;
  relevance_score: number;
}

const ChatBotDialog: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isMinimized, setIsMinimized] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const chatContainerRef = useRef<HTMLDivElement>(null);
  const { isHealthy, isChecking } = useApiHealth();
  const { isMobile, isKeyboardOpen, preventBackgroundScroll } = useMobileOptimizations();

  // Smooth scroll to bottom
  const scrollToBottom = useCallback(() => {
    if (messagesEndRef.current) {
      messagesEndRef.current.scrollIntoView({ 
        behavior: 'smooth',
        block: 'end'
      });
    }
  }, []);

  useEffect(() => {
    if (messages.length > 0) {
      const timer = setTimeout(scrollToBottom, 100);
      return () => clearTimeout(timer);
    }
  }, [messages, scrollToBottom]);

  useEffect(() => {
    if (isOpen && !isMinimized && inputRef.current) {
      const timer = setTimeout(() => {
        inputRef.current?.focus();
      }, 300); // Delay to ensure smooth animation
      return () => clearTimeout(timer);
    }
  }, [isOpen, isMinimized]);

  // Memoized follow-up question generator
  const generateFollowUps = useCallback((query: string): string[] => {
    const lowerQuery = query.toLowerCase();
    
    if (lowerQuery.includes('erc20') || lowerQuery.includes('token')) {
      return [
        'How do I migrate ERC721 NFTs?',
        'What are PSP22 libraries?',
        'How to handle token events?'
      ];
    } else if (lowerQuery.includes('erc721') || lowerQuery.includes('nft')) {
      return [
        'How do I migrate ERC20 tokens?',
        'What are ink! storage patterns?',
        'How to implement NFT metadata?'
      ];
    } else if (lowerQuery.includes('storage') || lowerQuery.includes('state')) {
      return [
        'How do mappings work in ink!?',
        'What are storage optimization tips?',
        'How to handle large datasets?'
      ];
    } else {
      return [
        'How do I get started with migration?',
        'What are the main differences between Solidity and ink!?',
        'Where can I find code examples?'
      ];
    }
  }, []);

  const sendMessage = useCallback(async (query: string) => {
    if (!query.trim() || !isHealthy) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      type: 'user',
      content: query,
      timestamp: new Date(),
    };

    setMessages(prev => [...prev, userMessage]);
    setInputValue('');
    setIsLoading(true);

    try {
      const response = await fetch(getApiUrl(API_CONFIG.ENDPOINTS.ASK), {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ query }),
      });

      const data: ApiResponse = await response.json();
      
      let aiContent = '';
      let sources: Source[] = [];
      let followUps: string[] = [];

      if (data.error) {
        aiContent = 'Sorry, I encountered an error processing your question. Please try again.';
      } else {
        if (typeof data.data === 'string') {
          aiContent = data.data;
        } else if (data.data) {
          aiContent = data.data.summary || data.data.help_text || 'No response available';
          
          // Extract sources from examples
          if (data.data.examples && data.data.examples.length > 0) {
            sources = data.data.examples.map(example => ({
              title: example.title,
              url: example.source_file ? `#${example.source_file}` : '#documentation'
            }));
          }

          followUps = generateFollowUps(query);
        }
      }

      const aiMessage: Message = {
        id: (Date.now() + 1).toString(),
        type: 'ai',
        content: aiContent,
        timestamp: new Date(),
        sources,
        followUps,
      };

      setMessages(prev => [...prev, aiMessage]);
    } catch (error) {
      console.error('Error sending message:', error);
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        type: 'ai',
        content: 'Sorry, I couldn\'t connect to the server. Please check your connection and try again.',
        timestamp: new Date(),
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  }, [isHealthy, generateFollowUps]);

  const handleSend = useCallback(() => {
    sendMessage(inputValue);
  }, [inputValue, sendMessage]);

  const handleKeyPress = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }, [handleSend]);

  const handleFollowUpClick = useCallback((question: string) => {
    setInputValue(question);
    sendMessage(question);
  }, [sendMessage]);

  const toggleOpen = useCallback(() => {
    setIsOpen(prev => {
      const newOpen = !prev;
      // Prevent background scrolling on mobile when chat is open
      preventBackgroundScroll(newOpen);
      return newOpen;
    });
    setIsMinimized(false);
  }, [preventBackgroundScroll]);

  const toggleMinimize = useCallback(() => {
    setIsMinimized(prev => !prev);
  }, []);

  // Memoized empty state component
  const EmptyState = useMemo(() => (
    <div className="text-gray-500 text-center py-8 px-4">
      {isHealthy ? (
        <>
          <MessageCircle size={48} className="mx-auto mb-4 text-gray-300" />
          <p className="text-sm mb-2">Ask me anything about migrating from Solidity to ink!</p>
          <p className="text-xs text-gray-400">Try: "How do I migrate an ERC20 token?"</p>
        </>
      ) : (
        <>
          <AlertCircle size={48} className="mx-auto mb-4 text-red-300" />
          <p className="text-sm text-red-500 mb-1">Unable to connect to AI service</p>
          <p className="text-xs text-red-400">Please check that the backend is running</p>
        </>
      )}
    </div>
  ), [isHealthy]);

  // Floating button
  if (!isOpen) {
    return (
      <button
        onClick={toggleOpen}
        className="fixed bottom-4 right-4 sm:bottom-6 sm:right-6 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white p-3 sm:p-4 rounded-full shadow-lg hover:shadow-xl transition-all duration-300 z-50 flex items-center gap-2 group touch-manipulation transform hover:scale-105 active:scale-95"
        aria-label="Open AI Chat"
      >
        <MessageCircle size={20} className="sm:w-6 sm:h-6" />
        <span className="hidden group-hover:inline-block whitespace-nowrap text-sm pr-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
          Ask AI
        </span>
      </button>
    );
  }

  // Mobile-first responsive sizing with keyboard adjustment
  const chatHeight = useMemo(() => {
    if (isMinimized) return 'h-14';
    if (isMobile && isKeyboardOpen) return 'h-[50vh]';
    if (isMobile) return 'h-[70vh]';
    return 'h-[500px] max-h-[80vh]';
  }, [isMinimized, isMobile, isKeyboardOpen]);

  const chatClasses = `
    chatbot-container fixed
    ${isMobile ? 'bottom-2 right-2 left-2' : 'bottom-4 right-4 sm:bottom-6 sm:right-6'}
    ${isMobile ? 'w-auto' : 'w-[calc(100vw-2rem)] max-w-sm sm:max-w-md lg:max-w-lg'}
    ${chatHeight}
    bg-white border border-gray-200 rounded-2xl shadow-2xl
    z-50 flex flex-col
    transition-all duration-300 ease-out
    transform ${isOpen ? 'scale-100 opacity-100' : 'scale-95 opacity-0'}
  `;

  return (
    <div className={chatClasses}>
      {/* Header */}
      <div className="flex items-center justify-between p-3 sm:p-4 border-b bg-gradient-to-r from-blue-600 to-blue-700 text-white rounded-t-2xl">
        <div className="flex items-center gap-2 min-w-0 flex-1">
          <MessageCircle size={18} className="flex-shrink-0" />
          <h3 className="font-semibold text-sm sm:text-base truncate">AI Assistant</h3>
          {!isChecking && (
            <div className="flex items-center gap-1 flex-shrink-0">
              <div className={`w-2 h-2 rounded-full ${isHealthy ? 'bg-green-400' : 'bg-red-400'} animate-pulse`}></div>
              {!isHealthy && <AlertCircle size={12} className="text-red-200" />}
            </div>
          )}
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={toggleMinimize}
            className="hover:bg-blue-600 p-1.5 rounded-lg transition-colors touch-manipulation"
            aria-label={isMinimized ? "Maximize chat" : "Minimize chat"}
          >
            <ChevronDown 
              size={16} 
              className={`transform transition-transform duration-200 ${isMinimized ? 'rotate-180' : ''}`} 
            />
          </button>
          <button
            onClick={() => {
              setIsOpen(false);
              preventBackgroundScroll(false);
            }}
            className="hover:bg-blue-600 p-1.5 rounded-lg transition-colors touch-manipulation"
            aria-label="Close chat"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      {/* Messages - Hidden when minimized */}
      {!isMinimized && (
        <>
          <div 
            ref={chatContainerRef}
            className="flex-1 overflow-y-auto overscroll-contain chat-messages"
            style={{ 
              scrollBehavior: 'smooth',
              WebkitOverflowScrolling: 'touch'
            }}
          >
            <div className="p-3 sm:p-4 space-y-3">
              {messages.length === 0 ? EmptyState : null}

              {messages.map((message) => (
                <div
                  key={message.id}
                  className={`flex ${message.type === 'user' ? 'justify-end' : 'justify-start'} message-enter`}
                >
                  <div
                    className={`max-w-[85%] sm:max-w-[80%] rounded-2xl px-3 py-2 ${
                      message.type === 'user'
                        ? 'bg-blue-600 text-white rounded-br-md'
                        : 'bg-gray-100 text-gray-800 rounded-bl-md'
                    } shadow-sm`}
                  >
                    <p className="text-sm leading-relaxed whitespace-pre-wrap">{message.content}</p>
                    
                    {/* Sources */}
                    {message.sources && message.sources.length > 0 && (
                      <div className="mt-3 pt-2 border-t border-gray-200/50">
                        <p className="text-xs font-medium mb-2 text-gray-600">Sources:</p>
                        <div className="space-y-1">
                          {message.sources.map((source, index) => (
                            <a
                              key={index}
                              href={source.url}
                              className="flex items-center gap-1.5 text-xs text-blue-600 hover:text-blue-800 hover:underline p-1 -m-1 rounded touch-manipulation"
                              target="_blank"
                              rel="noopener noreferrer"
                            >
                              <ExternalLink size={10} className="flex-shrink-0" />
                              <span className="truncate">{source.title}</span>
                            </a>
                          ))}
                        </div>
                      </div>
                    )}

                    {/* Follow-up Questions */}
                    {message.followUps && message.followUps.length > 0 && (
                      <div className="mt-3 pt-2 border-t border-gray-200/50">
                        <p className="text-xs font-medium mb-2 text-gray-600">Follow-up questions:</p>
                        <div className="space-y-1.5">
                          {message.followUps.map((question, index) => (
                            <button
                              key={index}
                              onClick={() => handleFollowUpClick(question)}
                              className="interactive-button touch-target block w-full text-left text-xs bg-white border border-gray-300 rounded-lg px-2 py-1.5 hover:bg-gray-50 active:bg-gray-100 transition-colors touch-manipulation"
                            >
                              {question}
                            </button>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              ))}

              {isLoading && (
                <div className="flex justify-start message-enter">
                  <div className="bg-gray-100 rounded-2xl rounded-bl-md px-3 py-2 max-w-[80%] shadow-sm">
                    <div className="flex items-center gap-1">
                      <div className="w-2 h-2 bg-gray-400 rounded-full typing-dot"></div>
                      <div className="w-2 h-2 bg-gray-400 rounded-full typing-dot"></div>
                      <div className="w-2 h-2 bg-gray-400 rounded-full typing-dot"></div>
                    </div>
                  </div>
                </div>
              )}

              <div ref={messagesEndRef} />
            </div>
          </div>

          {/* Input */}
          <div className="p-3 sm:p-4 border-t bg-gray-50/80 backdrop-blur-sm backdrop-blur-fallback rounded-b-2xl">
            <div className="flex gap-2">
              <input
                ref={inputRef}
                type="text"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyPress={handleKeyPress}
                placeholder={isHealthy ? "Ask about smart contract migration..." : "AI service unavailable"}
                className="chat-input flex-1 px-3 py-2.5 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm disabled:bg-gray-100 disabled:text-gray-500 transition-all duration-200 focus-visible"
                disabled={isLoading || !isHealthy}
                maxLength={500}
              />
              <button
                onClick={handleSend}
                disabled={isLoading || !inputValue.trim() || !isHealthy}
                className="interactive-button touch-target bg-blue-600 hover:bg-blue-700 active:bg-blue-800 disabled:bg-gray-400 text-white p-2.5 rounded-xl transition-all duration-200 flex items-center justify-center touch-manipulation transform hover:scale-105 active:scale-95 disabled:transform-none focus-visible"
                aria-label="Send message"
              >
                <Send size={16} />
              </button>
            </div>
            {inputValue.length > 400 && (
              <p className="text-xs text-gray-500 mt-1 text-right">
                {inputValue.length}/500
              </p>
            )}
          </div>
        </>
      )}
    </div>
  );
};

export default React.memo(ChatBotDialog);