# 🤖 SOL2INK Migration Assistant

AI-powered assistant for migrating smart contracts from Solidity to ink!. Built with React frontend, Rust backend, and RAG system using Qdrant + Gemini API.

![Frontend Interface](https://i.imgur.com/La8gAlS.png)

![Migration Assistant](https://i.imgur.com/L4k2avn.png)

![SOL2INK](https://i.imgur.com/cjwG43G.png)

## 🎯 Overview

The SOL2INK Migration Assistant helps developers seamlessly migrate smart contracts from Solidity to ink! through:

- **🤖 AI-Powered Guidance** - Ask natural language questions about migration patterns
- **📚 180+ Code Examples** - Real Solidity and ink! contract implementations for comparison  
- **🔍 Semantic Search** - Find relevant migration patterns and code examples instantly
- **📝 Rich Documentation** - Comprehensive migration guides with side-by-side comparisons
- **⚡ Real-time Interface** - Modern React frontend with live backend connectivity

## 🚀 Quick Start

### 1. Configuration Setup (Optional)

The project uses centralized configuration in `config.json`. Default ports are:
- Backend: 8000 (managed by Shuttle)
- Frontend: 5173  
- Qdrant: 6334

```bash
# To change ports, run the setup script
python3 setup-config.py

# Or manually edit config.json
```

### 2. Start Backend (5 minutes)

```bash
# 1. Start Qdrant database
docker run -p 6334:6334 qdrant/qdrant

# 2. Setup and run backend
cd shuttle-backend
python3 -m venv venv && source venv/bin/activate
pip install -r requirements.txt

# 3. Add your Gemini API key
echo "GEMINI_API_KEY=your_api_key_here" >> .env

# 4. Start Rust server
cargo run
```

### 3. Start Frontend

```bash
cd SOL2INK-frontend
npm install
npm run dev
```

### 4. Open & Ask Questions!

Open [http://localhost:5173](http://localhost:5173) and ask migration questions like:

- "How do I migrate ERC20 tokens from Solidity to ink!?"
- "What are the key differences between Solidity and ink!?"
- "Show me event handling examples in both languages"
- "How do I implement multisig wallets in ink!?"

## 🏗 Project Structure

```
├── SOL2INK-frontend/          # React frontend with migration interface
│   ├── src/components/        # UI components and migration assistant
│   ├── README.md             # Frontend setup and usage guide
│   └── package.json          # Dependencies and scripts
│
├── shuttle-backend/           # Rust backend with RAG system
│   ├── src/                  # Rust API server and RAG implementation
│   ├── README.md             # Detailed backend setup and API docs
│   ├── requirements.txt      # Python dependencies for embeddings
│   └── Cargo.toml           # Rust dependencies
│
├── docs/                     # Migration guides and documentation
├── solidity-examples/        # Sample Solidity contracts for reference
├── ink-examples-main/        # Sample ink! contracts for reference
└── test_integration.py       # Integration test script
```

## ✨ Key Features

- **Frontend** - Modern React app with real-time error handling and retry logic
- **Backend** - Rust API with RAG system, vector database, and Gemini integration
- **Migration Knowledge** - Embedded guides covering ERC20, ERC721, events, storage, and more
- **Smart Error Handling** - Automatic retries, connection monitoring, timeout handling
- **Rich Responses** - Markdown formatting with syntax-highlighted code examples

## 🔧 Architecture

```
Frontend (React) ←→ Backend (Rust) ←→ Qdrant (Vector DB) ←→ Gemini API
                                   ↗
                           Migration Guides + Code Examples
```

## 🔧 Configuration

The project uses centralized configuration files:

- **`config.json`** - Main configuration file with port settings
- **`config.py`** - Python configuration loader 
- **`setup-config.py`** - Interactive configuration setup script
- **`SOL2INK-frontend/.env`** - Frontend environment variables

**Change ports easily:**
```bash
# Interactive setup
python3 setup-config.py

# Or edit config.json directly
{
  "backend": {"port": 8000},
  "frontend": {"port": 5173},
  "qdrant": {"port": 6334}
}
```

## 📚 Documentation

- **[Frontend README](SOL2INK-frontend/README.md)** - React app setup, usage, and component details
- **[Backend README](shuttle-backend/README.md)** - Complete backend setup, API docs, testing, and deployment
- **[Integration Test](test_integration.py)** - Test frontend-backend connectivity

## 🧪 Testing Integration

Test the complete system:

```bash
# Test backend endpoints and frontend connectivity
python3 test_integration.py
```

## 💡 Example Usage

### Via Frontend Interface
1. Open http://localhost:5173
2. Click example queries or type custom questions
3. Get AI-powered migration guidance with code examples

### Via API (curl)
```bash
# Ask migration questions directly
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How do I migrate ERC20 tokens from Solidity to ink!?"}'
```

## 🛠 Prerequisites

- **Node.js 20+** - For frontend development
- **Rust** - For backend API server  
- **Docker** - For Qdrant vector database
- **Python 3.9+** - For embedding scripts
- **Gemini API Key** - Get from [Google AI Studio](https://makersuite.google.com/app/apikey)

## 🚀 Deployment

- **Frontend** - Static hosting (Vercel, Netlify, etc.)
- **Backend** - Shuttle.dev deployment with managed PostgreSQL and Qdrant

## 🤝 Contributing

1. Fork the repository
2. Choose frontend (React/TypeScript) or backend (Rust) development
3. Follow setup guides in respective README files
4. Submit pull requests with improvements

## 📄 License

MIT License - Built with ❤️ for the Polkadot ecosystem

---

**Ready to migrate from Solidity to ink!? Start with the [Frontend Setup](SOL2INK-frontend/README.md) or [Backend Setup](shuttle-backend/README.md)** 🚀