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
- **TailwindCSS v4** - Utility-first CSS framework
- **shadcn/ui** - Beautiful, accessible component library
- **React Markdown** - Markdown rendering with syntax highlighting

## 🚀 Quick Start

### Prerequisites

- Node.js 20.19.0+ or 22.12.0+
- SOL2INK Backend running on `localhost:8000` (see [shuttle-backend README](../shuttle-backend/README.md))

### Installation & Setup

```bash
# 1. Clone and navigate
git clone <repository-url>
cd SOL2INK-frontend

# 2. Install dependencies
npm install

# 3. Start development server
npm run dev

# 4. Open http://localhost:5173
```

### Start Backend

The frontend requires the backend to be running:

```bash
cd ../shuttle-backend
cargo run
```

## 🎨 Usage

### Migration Assistant Interface

1. **Quick Examples** - Click any example query for instant results
2. **Custom Queries** - Type migration questions in the textarea
3. **Keyboard Shortcuts** - Use `Cmd/Ctrl + Enter` to submit
4. **Error Recovery** - Automatic retries with manual fallback options
5. **Connection Status** - Real-time backend connectivity in header

### Example Queries

- "How do I migrate ERC20 tokens from Solidity to ink!?"
- "What are the key differences between Solidity and ink!?"
- "Show me event handling examples in both languages"
- "How do I implement multisig wallets in ink!?"
- "How do I convert Solidity mappings to ink! storage?"

## 🔧 Development

### Available Scripts

- `npm run dev` - Start development server (port 5173)
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint for code quality

### Key Components

- **MigrationAssistant** - Main component with query handling and error management
- **MarkdownRenderer** - Rich markdown rendering with syntax highlighting
- **Connection Management** - Real-time backend connectivity monitoring
- **Error Handling** - Comprehensive error categorization and retry logic

## 🚀 Deployment

### Production Build

```bash
npm run build
```

Output will be in the `dist/` directory, ready for static hosting.

### Environment Configuration

For production deployment:
- Configure backend API URL (default: `http://localhost:8000`)
- Set up environment-specific variables as needed

## 🆘 Troubleshooting

**Backend Connection Failed**
- Ensure backend is running on `localhost:8000`
- Check if Qdrant database is running
- Verify network connectivity

**Build Errors**
- Clear node_modules: `rm -rf node_modules && npm install`
- Clear Vite cache: `npx vite --force`

**For detailed testing and integration setup, see [Backend README](../shuttle-backend/README.md)**

---

**Built with ❤️ for the Polkadot ecosystem**