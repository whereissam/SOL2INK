# 🚀 DynaVest Shuttle Backend Setup Guide

## 📋 Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **Shuttle CLI** - Install with `cargo install cargo-shuttle`
3. **Gemini API Key** - Required for AI features
4. **Qdrant Cloud Account** - For vector database (or local Qdrant)

## 🔑 API Key Setup

### 1. Gemini API Key

1. Go to [Gemini API Keys](https://aistudio.google.com/prompts/new_chat)
2. Create a new API key
3. Copy the key (starts with ``)
4. Replace the placeholder in `Secrets.toml`:

```toml
GEMINI_API_KEY = "your-actual-openai-api-key-here"
```

### 2. Qdrant Setup

**Option A: Use Existing Qdrant Cloud (Recommended)**
- Your `Secrets.toml` already has a configured Qdrant cluster
- No additional setup needed

**Option B: Local Qdrant (for development)**
```bash
# Run Qdrant locally with Docker
docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant

# Update Secrets.toml
QDRANT_URL = "http://localhost:6334"
QDRANT_API_KEY = ""  # Leave empty for local instance
```

**Option C: Create New Qdrant Cloud Account**
1. Go to [Qdrant Cloud](https://cloud.qdrant.io/)
2. Create a free account
3. Create a new cluster
4. Copy the cluster URL and API key to `Secrets.toml`

## 🛠️ Local Development Setup

### 1. Install Dependencies
```bash
cd shuttle-backend
cargo build
```

### 2. Set up Environment
```bash
# Copy the example environment file
cp .env.example .env

# Edit .env with your preferred settings (optional)
# The main configuration is in Secrets.toml
```

### 3. Start Development Server
```bash
# Start the shuttle development server
shuttle run

# Or use the dev script
./dev.sh
```

The server will start on `http://localhost:8000`

## 🧪 Testing the Setup

### 1. Quick Health Check
```bash
curl http://localhost:8000/health
```

### 2. Test AI Chat
```bash
curl -X POST http://localhost:8000/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "What is DeFi yield farming?",
    "user_id": "test-user"
  }'
```

### 3. Test RAG System
```bash
# Test semantic search
curl -X POST http://localhost:8000/rag/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is impermanent loss?",
    "limit": 3
  }'
```

### 4. Run Complete Test Suite
```bash
# Make the test script executable
chmod +x test_api_examples.sh

# Run all API tests
./test_api_examples.sh
```

## 🚀 Deployment

### 1. Deploy to Shuttle
```bash
# Login to Shuttle
shuttle auth login

# Deploy the application
shuttle deploy
```

### 2. Set Production Secrets
```bash
# Set Gemini API key
shuttle project env set GEMINI_API_KEY="sk-your-actual-key"

# Set Qdrant credentials
shuttle project env set QDRANT_URL="your-qdrant-url"
shuttle project env set QDRANT_API_KEY="your-qdrant-api-key"
```

## 🔧 Configuration Options

### Environment Variables (Optional)

You can override settings using environment variables:

```bash
# Database configuration
export DATABASE_URL="postgresql://user:password@localhost/db"

# Contract configuration
export CONTRACT_ADDRESS="your-contract-address"
export RPC_URL="wss://moonbeam-alpha.api.onfinality.io/public-ws"

# Server configuration
export PORT=8000
export RUST_LOG=info
```

### Secrets.toml Format

```toml
# Required: Gemini API Key
GEMINI_API_KEY = "your-actual-gemini-api-key"

# Required: Qdrant Configuration
QDRANT_URL = "https://your-cluster.qdrant.io:6334"
QDRANT_API_KEY = "your-qdrant-api-key"

# Optional: Contract Configuration
CONTRACT_ADDRESS = "your-contract-address"
RPC_URL = "wss://moonbeam-alpha.api.onfinality.io/public-ws"
```

## 🐛 Troubleshooting

### Common Issues

1. **"Runtime Resource Initialization phase failed"**
   - Check that your database connection string is correct
   - Ensure PostgreSQL is running (handled by Shuttle locally)

2. **"Gemini API error"**
   - Verify your Gemini API key is valid
   - Check you have credits in your Gemini account

3. **"Qdrant connection failed"**
   - Verify Qdrant URL and API key are correct
   - Check that your Qdrant cluster is running

4. **"Compilation errors"**
   - Run `cargo clean` and then `cargo build`
   - Make sure you have the latest Rust version

### Getting Help

- Check the [Shuttle documentation](https://docs.shuttle.dev/)
- Review the API documentation in README.md
- Run `shuttle --help` for CLI options

## 📈 Production Considerations

1. **Rate Limiting**: The current setup doesn't include rate limiting
2. **Monitoring**: Add logging and monitoring for production use
3. **Scaling**: Shuttle handles scaling automatically
4. **Security**: Ensure all API keys are properly secured
5. **Backup**: Consider backing up your Qdrant collections

## 🎯 Next Steps

1. Set up your OpenAI API key in `Secrets.toml`
2. Test locally with `shuttle run`
3. Deploy to production with `shuttle deploy`
4. Integrate with your frontend application
5. Add custom DeFi knowledge documents via the `/rag/document` endpoint

---

**Need help?** Check the main README.md for detailed API documentation and examples.