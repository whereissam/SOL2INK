# 📊 SOL2INK Dataset Sharing Guide

## Overview
This guide explains how to share your Solidity to ink! migration dataset and AI system with other developers.

## 🎯 What You're Sharing

### **Dataset Contents:**
- **12 ink! smart contract examples** with semantic embeddings
- **Migration guides** and best practices
- **Rich metadata** (topics, categories, file paths, timestamps)
- **Vector embeddings** for semantic search
- **AI-powered Q&A system** using Gemini API

### **System Components:**
- **RAG System** - Retrieval-Augmented Generation for contextual responses
- **Semantic Search** - Find relevant code examples instantly
- **REST API** - Easy integration with any application
- **Frontend Interface** - Web-based migration assistant

## 🚀 Sharing Options

### **Option 1: Public API Access (Easiest for Users)**

**Benefits:**
- Users don't need to set up anything
- You control the dataset and API keys
- Easy to use for developers
- No technical barriers

**How to set up:**
1. Deploy your backend to a public service (Shuttle, Vercel, etc.)
2. Share your public URL
3. Provide API documentation

**Example usage:**
```bash
# Health check
curl https://your-domain.com/health

# Ask migration questions
curl -X POST "https://your-domain.com/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How do I migrate ERC20 from Solidity to ink!?"}'

# Search your dataset
curl -X POST "https://your-domain.com/rag/search" \
  -H "Content-Type: application/json" \
  -d '{"query": "flipper contract", "limit": 3}'
```

### **Option 2: Complete Setup Package (Full Control)**

**Benefits:**
- Users have full control over their instance
- Can customize and extend the dataset
- No dependency on your hosted service
- Can use their own API keys

**What you provide:**
1. **Your codebase** - Complete RAG system
2. **Dataset export** - Pre-embedded examples
3. **Setup instructions** - Step-by-step guide
4. **Docker setup** - Easy deployment

**Setup for others:**
```bash
# Clone your repository
git clone https://github.com/your-username/sol2ink-dataset
cd sol2ink-dataset

# Start with Docker (recommended)
docker-compose up -d

# Or manual setup
./setup.sh
```

### **Option 3: Dataset Export for Other AI Systems**

**Benefits:**
- Works with any AI system (OpenAI, Claude, etc.)
- Flexible integration
- Can be used with custom RAG systems

**Export formats:**
- **JSON** - Structured data with metadata
- **CSV** - Simple tabular format
- **Markdown** - Human-readable documentation
- **Vector embeddings** - For semantic search

## 📋 Implementation Steps

### **For Public API (Option 1):**

1. **Deploy your backend:**
   ```bash
   # Using Shuttle (current setup)
   shuttle deploy
   
   # Or using Docker
   docker build -t sol2ink-backend .
   docker run -p 8000:8000 sol2ink-backend
   ```

2. **Create API documentation:**
   ```markdown
   # SOL2INK Migration API
   
   Base URL: https://your-domain.com
   
   ## Endpoints:
   - GET /health - Health check
   - POST /ask - Ask migration questions
   - POST /rag/search - Search code examples
   - GET /rag/stats - Get dataset statistics
   ```

3. **Share the URL and documentation**

### **For Complete Setup (Option 2):**

1. **Create export script:**
   ```bash
   #!/bin/bash
   # export-dataset.sh
   
   echo "📦 Exporting SOL2INK Dataset..."
   
   # Export embedded data from Qdrant
   curl -X POST "http://localhost:8000/rag/search" \
     -H "Content-Type: application/json" \
     -d '{"query": "", "limit": 1000}' > dataset.json
   
   echo "✅ Dataset exported to dataset.json"
   ```

2. **Create setup script:**
   ```bash
   #!/bin/bash
   # setup.sh
   
   echo "🚀 Setting up SOL2INK Migration Assistant..."
   
   # Install dependencies
   cargo build
   
   # Start Qdrant
   docker run -d -p 6334:6334 qdrant/qdrant
   
   # Load dataset
   ./load-dataset.sh
   
   # Start backend
   cargo run
   ```

3. **Package everything:**
   ```
   sol2ink-dataset/
   ├── README.md
   ├── DATASET_SHARING_GUIDE.md
   ├── setup.sh
   ├── export-dataset.sh
   ├── load-dataset.sh
   ├── docker-compose.yml
   ├── src/
   ├── dataset.json
   └── examples/
   ```

### **For Dataset Export (Option 3):**

1. **Create export utilities:**
   ```rust
   // Add to your backend
   #[derive(Serialize)]
   struct DatasetExport {
       documents: Vec<Document>,
       metadata: ExportMetadata,
   }
   
   async fn export_dataset() -> Result<DatasetExport, Error> {
       // Export logic here
   }
   ```

2. **Multiple export formats:**
   ```bash
   # Export as JSON
   curl http://localhost:8000/export/json > sol2ink-dataset.json
   
   # Export as CSV
   curl http://localhost:8000/export/csv > sol2ink-dataset.csv
   
   # Export as Markdown
   curl http://localhost:8000/export/markdown > sol2ink-dataset.md
   ```

## 🔧 Technical Requirements for Users

### **For API Usage (Option 1):**
- None! Just curl or HTTP client

### **For Complete Setup (Option 2):**
- **Rust** - For building the backend
- **Docker** - For Qdrant database
- **Gemini API Key** - For AI responses
- **4GB RAM** - For running the system

### **For Dataset Export (Option 3):**
- **AI Service** - OpenAI, Claude, or similar
- **Vector Database** - Qdrant, Pinecone, or similar
- **Programming Language** - Any language with HTTP client

## 📚 Documentation to Provide

### **API Documentation:**
```markdown
# SOL2INK Migration API

## Authentication
No authentication required for public API.

## Endpoints

### POST /ask
Ask migration questions and get AI-powered responses.

**Request:**
```json
{
  "query": "How do I migrate ERC20 from Solidity to ink!?"
}
```

**Response:**
```json
{
  "success": true,
  "data": "To migrate an ERC20 token from Solidity to ink!...",
  "error": null
}
```

### POST /rag/search
Search through code examples and documentation.

**Request:**
```json
{
  "query": "flipper contract",
  "limit": 3,
  "score_threshold": 0.7
}
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "content": "// Flipper contract code...",
      "score": 0.95,
      "metadata": {
        "contract_type": "basic",
        "category": "flipper",
        "language": "rust"
      }
    }
  ]
}
```
```

### **Usage Examples:**
```bash
# JavaScript/Node.js
const response = await fetch('https://your-domain.com/ask', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    query: 'How do I create a flipper contract in ink!?'
  })
});

# Python
import requests
response = requests.post('https://your-domain.com/ask', json={
    'query': 'How do I create a flipper contract in ink!?'
})

# cURL
curl -X POST "https://your-domain.com/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How do I create a flipper contract in ink!?"}'
```

## 🌟 Recommended Approach

**For maximum adoption, I recommend Option 1 (Public API):**

1. **Deploy your backend publicly** using Shuttle or similar
2. **Create simple API documentation** 
3. **Share the URL** with the community
4. **Provide usage examples** in multiple languages

This approach has the lowest barrier to entry and will get the most users.

## 📈 Analytics & Usage

Consider adding:
- **Usage analytics** - Track popular queries
- **Rate limiting** - Prevent abuse
- **API keys** - For heavy users
- **Caching** - Improve performance

## 🤝 Community Sharing

Share your dataset on:
- **GitHub** - Open source the code
- **Hugging Face** - Dataset repository
- **Reddit** - r/rust, r/polkadot
- **Discord** - ink! community
- **Twitter** - Developer community

## 📝 License

Consider adding an appropriate license:
- **MIT** - Most permissive
- **Apache 2.0** - Good for commercial use
- **CC BY 4.0** - For datasets

---

**Your SOL2INK dataset is valuable to the community! Choose the sharing method that best fits your goals and technical requirements.**