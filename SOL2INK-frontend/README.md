# SOL2INK Migration Assistant Frontend

The SOL2INK Migration Assistant that helps developers migrate smart contracts from Solidity to ink!. 

![](https://i.imgur.com/La8gAlS.png)

![](https://i.imgur.com/L4k2avn.png)

## 🎯 Overview

The SOL2INK Migration Assistant provides AI-powered guidance for migrating smart contracts from Solidity to ink!. It features an intuitive interface with real-time backend connectivity, robust error handling, and comprehensive migration examples.

## ✨ Features

- 🤖 **AI-Powered Migration Guidance** - Get instant help with Solidity to ink! migrations
- 📡 **Real-time Backend Connectivity** - Live connection status with automatic health checks
- 🔄 **Smart Retry Logic** - Exponential backoff retry system for failed requests
- ⚡ **Enhanced Error Handling** - Detailed error categorization and user feedback
- 📱 **Responsive Design** - Works seamlessly on desktop and mobile devices
- 🎨 **Modern UI Components** - Built with shadcn/ui and TailwindCSS
- 🚀 **Performance Optimized** - Fast loading with Vite and React 19
- 📝 **Markdown Rendering** - Rich formatting for code examples and documentation

## 🛠 Tech Stack

- **React 19** - Latest React with modern features
- **TypeScript** - Type safety and better DX
- **Vite** - Lightning fast build tool and dev server
- **TanStack Router** - Type-safe file-based routing
- **TanStack React Query** - Server state management and caching
- **TailwindCSS v4** - Utility-first CSS framework
- **shadcn/ui** - Beautiful, accessible component library
- **Lucide React** - Modern icon library
- **React Markdown** - Markdown rendering with syntax highlighting
- **React Syntax Highlighter** - Code syntax highlighting

## 🚀 Getting Started

### Prerequisites

- Node.js 20.19.0+ or 22.12.0+
- npm, yarn, or bun
- SOL2INK Backend running on `localhost:8000`

### Installation

1. **Clone and navigate to the project:**
   ```bash
   git clone <repository-url>
   cd SOL2INK-frontend
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Start the development server:**
   ```bash
   npm run dev
   ```

4. **Open [http://localhost:5173](http://localhost:5173) in your browser**

### Backend Integration

The frontend connects to the SOL2INK backend API at `http://localhost:8000`. Make sure the backend is running:

```bash
cd ../shuttle-backend
cargo run
```

## 📋 Available Scripts

- `npm run dev` - Start development server (port 5173)
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint for code quality

## 🏗 Project Structure

```
src/
├── components/
│   ├── ui/                    # shadcn/ui components
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── textarea.tsx
│   │   └── badge.tsx
│   ├── MigrationAssistant.tsx # Main migration assistant component
│   ├── MarkdownRenderer.tsx   # Markdown rendering with syntax highlighting
│   ├── theme-provider.tsx     # Theme context provider
│   └── theme-toggle.tsx       # Dark/light mode toggle
├── lib/
│   └── utils.ts              # Utility functions and cn helper
├── pages/
│   ├── Home.tsx              # Home page component
│   └── About.tsx             # About page component
├── routes/                   # TanStack Router file-based routes
│   ├── __root.tsx            # Root layout with navigation
│   ├── index.tsx             # Home route (/)
│   ├── about.tsx             # About route (/about)
│   └── features.tsx          # Features route (/features)
├── App.tsx                   # Main app component
├── main.tsx                  # App entry point
└── index.css                 # Global styles and Tailwind imports
```

## 🔧 Key Components

### MigrationAssistant

The main component that handles:
- Query input and submission
- Backend communication with retry logic
- Error handling and user feedback
- Connection status monitoring
- Loading states and progress indicators

**Features:**
- Real-time backend connectivity status
- Automatic retry with exponential backoff
- Comprehensive error categorization
- Quick example queries for common migration patterns
- Markdown rendering for rich responses

### Error Handling

Robust error handling system that categorizes and handles:
- **Network errors** - Connection failures, DNS issues
- **Server errors** - Backend API errors, 5xx responses
- **Timeout errors** - Request timeouts (30s default)
- **Unknown errors** - Unexpected failures

### Connection Management

- **Real-time monitoring** - Connection status updates every 30 seconds
- **Health checks** - Tests `/health` endpoint availability
- **Visual indicators** - Clear connected/disconnected status
- **Manual retry** - Users can manually test connections

## 🎨 Styling & Theming

- **TailwindCSS v4** - Latest version with PostCSS plugin
- **Dark/Light mode** - Automatic theme switching support
- **Component variants** - Using `class-variance-authority`
- **Responsive design** - Mobile-first approach
- **Custom animations** - Smooth transitions and loading states

## 🤖 Migration Assistant Usage

1. **Quick Examples** - Click any example query to see instant results
2. **Custom Queries** - Type your migration questions in the textarea
3. **Keyboard Shortcuts** - Use `Cmd/Ctrl + Enter` to submit quickly
4. **Error Recovery** - Automatic retries with manual fallback options
5. **Connection Monitoring** - Real-time backend status in the header

### Example Queries

- "How do I migrate ERC20 tokens from Solidity to ink!?"
- "What are the key differences between Solidity and ink!?"
- "Show me event handling examples in both languages"
- "How do I implement multisig wallets in ink!?"
- "How do I convert Solidity mappings to ink! storage?"

## 🔗 API Integration

The frontend integrates with the following backend endpoints:

- `GET /health` - Health check and connectivity testing
- `POST /ask` - Main migration assistant queries
- `GET /ask?query=...` - Alternative query endpoint

All requests include:
- 30-second timeout
- Automatic retry logic (up to 3 attempts)
- Exponential backoff delay
- Comprehensive error handling

## 🧪 Testing

Run the integration test to verify frontend-backend connectivity:

```bash
# From the project root
python3 test_integration.py
```

This tests:
- Backend health endpoint
- Ask endpoint functionality
- Complete integration workflow
- Error handling scenarios

## 🚀 Deployment

### Production Build

```bash
npm run build
```

The build output will be in the `dist/` directory, ready for deployment to any static hosting service.

### Environment Variables

For production deployment, configure:
- Backend API URL (default: `http://localhost:8000`)
- Any additional environment-specific settings

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run linting (`npm run lint`)
5. Test the build (`npm run build`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

MIT License - feel free to use this project for your smart contract migration needs!

## 🆘 Troubleshooting

### Common Issues

**Backend Connection Failed**
- Ensure the backend is running on `localhost:8000`
- Check if Qdrant database is running
- Verify network connectivity

**Build Errors**
- Clear node_modules: `rm -rf node_modules && npm install`
- Update dependencies: `npm update`
- Check TypeScript configuration

**Development Server Issues**
- Try a different port: `npm run dev -- --port 3000`
- Clear Vite cache: `npx vite --force`

### Getting Help

- Check the integration test results: `python3 test_integration.py`
- Review browser console for detailed error messages
- Verify backend logs for API-related issues

---

**Built with ❤️ for the Polkadot ecosystem**