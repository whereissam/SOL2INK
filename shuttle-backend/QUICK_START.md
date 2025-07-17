# 🚀 DynaVest Backend Quick Start Guide

## ✅ Current Status

**Great news!** Your setup is working perfectly:

- ✅ **Qdrant Connection**: Successfully connected to your cloud instance
- ✅ **Database Collections**: Found existing `defi_knowledge` collection
- ✅ **Build Status**: All dependencies compiled successfully

## 🎯 How to Run the Server

### Option 1: Using the Run Script (Recommended)
```bash
./run_server.sh
```

### Option 2: Using Dev Script
```bash
./dev.sh dev
```

### Option 3: Manual Setup
```bash
# Export environment variables
export GEMINI_API_KEY=""
export QDRANT_URL=""
export QDRANT_API_KEY=""

# Start the server
shuttle run
```

## 🧪 Test Your Setup

### 1. Basic Health Check
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

### 3. Test RAG Query
```bash
curl -X POST http://localhost:8000/rag/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is impermanent loss?",
    "limit": 3
  }'
```

### 4. Test Semantic Search
```bash
curl -X POST http://localhost:8000/rag/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "yield farming strategies",
    "limit": 5,
    "score_threshold": 0.7
  }'
```

### 5. Run All Tests
```bash
./test_api_examples.sh
```

## 📊 What's Working

Your backend now includes:

1. **PostgreSQL Database** - For user strategies and application data
2. **Qdrant Vector Database** - For AI-powered semantic search
3. **OpenAI Integration** - For chat and embeddings
4. **RAG System** - Retrieval-Augmented Generation for contextual responses
5. **Semantic Caching** - Cost optimization for repeated queries
6. **Pre-loaded Knowledge** - 12 DeFi strategy documents

## 🔧 Available APIs

- **Strategy Management**: `/strategies` (CRUD operations)
- **AI Chat**: `/chat` (Contextual AI responses)
- **RAG Queries**: `/rag/query` (AI with knowledge base)
- **Semantic Search**: `/rag/search` (Vector similarity search)
- **Document Management**: `/rag/document` (Add knowledge)
- **Statistics**: `/rag/stats` (System metrics)
- **Cross-chain**: `/cross-chain/*` (Multi-chain strategies)
- **Contract Integration**: `/contract/*` (ink! smart contracts)

## 🎉 Next Steps

1. **Start the server** with `./run_server.sh`
2. **Test the APIs** with the provided curl commands
3. **Integrate with your frontend** using the JavaScript examples in README.md
4. **Deploy to production** with `shuttle deploy`

## 🆘 If You See Errors

- **Database Migration Errors**: Normal if tables already exist
- **Qdrant Collection Warnings**: Collections will be created automatically
- **OpenAI API Errors**: Check your API key and account credits
- **Connection Refused**: Make sure your API keys are properly exported

## 📈 Performance Notes

- **First startup** may take 30-60 seconds to initialize collections
- **Sample data** is loaded automatically on startup
- **Semantic cache** will improve performance over time
- **API responses** should be sub-second after initialization

---

**🎯 You're all set!** Your DynaVest backend is configured and ready to power your AI-driven DeFi platform.