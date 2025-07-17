# 🤖 Solidity to ink! Migration Assistant (Rust + Qdrant + Gemini)

A comprehensive RAG-powered migration assistant built with Rust and Shuttle.dev. Ask natural language questions about migrating smart contracts from Solidity to ink! and get accurate, context-aware answers using Retrieval-Augmented Generation (RAG) with Gemini API.

## 🎯 What This System Does

- **📚 10+ Migration Guides**: Complete guides for migrating common contract patterns from Solidity to ink!
- **🔍 180+ Code Examples**: Real Solidity and ink! contract implementations for comparison
- **🧠 AI-Powered Guidance**: RAG system that provides contextual migration advice
- **⚡ Semantic Search**: Find relevant migration patterns and code examples instantly
- **🔄 Side-by-Side Comparisons**: See Solidity and ink! implementations together

## 🚀 Quick Start - How to Use

### 1. **Setup (5 minutes)**

```bash
# 1. Start local Qdrant (in terminal 1)
docker run -p 6334:6334 qdrant/qdrant

# 2. Setup Python environment and embed codebase (in terminal 2)
cd shuttle-backend
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# 3. Embed migration guides and examples (includes both Solidity and ink! examples)
python3 embed_migration_guides.py ../docs/migrations \
  --solidity-examples ../solidity-examples \
  --ink-examples ../ink-examples-main \
  --collection migration_guides

# 4. Add your Gemini API key to .env
echo "GEMINI_API_KEY=your_actual_gemini_api_key_here" >> .env
```

### 2. **Start the Server**

```bash
# Start the Rust server (make sure Qdrant is running first)
cargo run --bin dynavest-shuttle-backend
```

### 3. **Ask Migration Questions!**

```bash
# ✅ CORRECT: GET request with proper URL encoding
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How does the flipper contract work?"

# ✅ CORRECT: POST request with JSON
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How do I migrate ERC20 tokens from Solidity to ink!?"}'  

# ❌ WRONG: This will give "URL rejected: Malformed input" error
# curl "http://localhost:8000/ask?query=How does the flipper contract work?"
```

### 4. **Migration Questions You Can Ask**

- `"How do I migrate ERC20 tokens from Solidity to ink!?"`
- `"What are the key differences between Solidity and ink!?"`
- `"Show me how to implement multisig wallets in ink!"`
- `"How do events work differently in ink! compared to Solidity?"`
- `"How do I convert Solidity mappings to ink! storage?"`
- `"What are the migration steps for escrow contracts?"`
- `"Show me event handling examples in both languages"`

### 5. **Response Formats & Examples**

#### ✅ Successful Response Format
```json
{
  "success": true,
  "data": "## Flipper Contract Explained: Solidity vs. Ink!\n\nThe `flipper` contract is a simple example demonstrating core concepts in both Solidity and Ink!...",
  "error": null
}
```

#### 📄 Pretty Formatted Response (use `jq` for readable output)
```bash
# Get nicely formatted response
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I implement ERC20 tokens in ink!?" | jq -r '.data'
```

#### 📋 Example Migration Response
When you ask about ERC20 migration, you get:

**✅ Comprehensive comparison tables** showing Solidity vs ink! differences
**✅ Step-by-step migration instructions** with actual code examples  
**✅ Before/after code snippets** from real contracts
**✅ Best practices and gotchas** specific to the migration
**✅ Testing and deployment guidance**

#### 🎯 Sample Response for "How do I migrate ERC20 tokens?"
```markdown
## ERC20 Token Migration: Solidity to ink!

### Key Differences Table
| Feature | Solidity | ink! |
|---------|----------|------|
| Storage | mapping(address => uint256) | Mapping<AccountId, Balance> |
| Events  | event Transfer(...) | #[ink(event)] Transfer { ... } |
| Errors  | require(...) | Result<(), Error> |

### Migration Steps
1. **Convert storage structure**
2. **Update function signatures** 
3. **Implement error handling**
4. **Add event definitions**
5. **Update tests**

### Code Examples
**Before (Solidity):**
```solidity
function transfer(address to, uint256 value) public returns (bool) {
    require(balances[msg.sender] >= value);
    // ... rest of implementation
}
```

**After (ink!):**
```rust
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
    let caller = self.env().caller();
    // ... rest of implementation
}
```

### 6. **Getting Formatted Responses**

#### 🎨 Pretty Print with `jq` (Recommended)
```bash
# Install jq if you don't have it: brew install jq (macOS) or apt install jq (Linux)

# Get clean markdown response without JSON wrapper
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I migrate ERC20 tokens?" | jq -r '.data'

# Get just the success status
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=test" | jq '.success'

# Handle errors gracefully
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=test" | jq -r 'if .error then .error else .data end'
```

#### 📱 Raw JSON Response
```bash
# Get full JSON response (useful for programmatic access)
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=What are the differences between Solidity and ink!?"
```

#### 🐍 Python Client Example
```python
import requests
import json

# Simple query
response = requests.get(
    "http://localhost:8000/ask",
    params={"query": "How do multisig wallets work in ink!?"}
)

data = response.json()
if data["success"]:
    print(data["data"])  # Clean markdown output
else:
    print(f"Error: {data['error']}")
```

#### 🌐 JavaScript/Browser Example
```javascript
// Using fetch API
async function askMigrationQuestion(query) {
    const response = await fetch(`http://localhost:8000/ask?${new URLSearchParams({query})}`);
    const data = await response.json();
    
    if (data.success) {
        return data.data; // Markdown content
    } else {
        throw new Error(data.error);
    }
}

// Usage
askMigrationQuestion("How do I implement events in ink!?")
    .then(answer => console.log(answer))
    .catch(error => console.error(error));
```

### 7. **Test Your Setup**

```bash
# Quick health check
curl http://localhost:8000/health

# Test with simple question
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=Hello" | jq -r '.data'

# Test migration-specific question  
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How does the flipper contract work?" | jq -r '.data'
```

## ⚡ Prerequisites

### Required Tools
- **Docker**: For running Qdrant locally
- **Rust**: Latest stable version (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Python 3.9+**: For embedding codebase
- **Gemini API Key**: Get from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Quick Check
```bash
# Verify tools are installed
docker --version
rustc --version  
python3 --version
```

## 🔧 Step-by-Step Setup

### Step 1: Start Qdrant Database
```bash
# Terminal 1: Start Qdrant (keep this running)
docker run -p 6334:6334 qdrant/qdrant

# You should see: "Qdrant HTTP listening on 6333" and "Qdrant gRPC listening on 6334"
```

### Step 2: Prepare the Codebase
```bash
# Terminal 2: Navigate to project
cd /path/to/your/project/aidoc/shuttle-backend

# Setup Python environment
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# Verify Qdrant connection
python3 test_qdrant.py
```

### Step 3: Add Your API Keys
```bash
# Copy and edit environment file
cp .env.example .env

# Edit .env file and add your Gemini API key:
# GEMINI_API_KEY=your_actual_api_key_here
# QDRANT_URL=http://localhost:6334
```

### Step 4: Embed Your Codebase (if not done)
```bash
# The ink examples are already embedded, but to embed your own code:
python3 embed_codebase.py /path/to/your/code --collection code_knowledge

# Test the embedding worked
python3 test_rag_directly.py
```

### Step 5: Start the RAG Server
```bash
# Build and run the Rust server
cargo build
cargo run --bin dynavest-shuttle-backend

# You should see: "🚀 DynaVest Shuttle Backend is starting..."
```

### Step 6: Test It Works
```bash
# Test health check
curl http://localhost:8000/health

# Ask a question
curl "http://localhost:8000/ask?query=How does the flipper contract work?"
```

## ❗ Troubleshooting

### Common Issues

**1. "Connection refused" when starting server**
```bash
# Make sure Qdrant is running first
docker ps | grep qdrant

# If not running:
docker run -p 6334:6334 qdrant/qdrant
```

**2. "No embeddings found"**
```bash
# Re-run the embedding process
python3 embed_codebase.py ../ink-examples-main --collection code_knowledge
```

**3. "Gemini API key invalid"**
```bash
# Check your .env file has the correct key
cat .env | grep GEMINI_API_KEY

# Get a new key from: https://makersuite.google.com/app/apikey
```

**4. "Cargo run fails"**
```bash
# Make sure you're running the correct binary
cargo run --bin dynavest-shuttle-backend

# Check dependencies
cargo check
```

### Verify Everything Works
```bash
# 1. Check Qdrant has data
curl -X GET "http://localhost:6334/collections/code_knowledge"

# 2. Check server is running  
curl http://localhost:8000/health

# 3. Test search works
curl -X POST "http://localhost:8000/rag/search" \
  -H "Content-Type: application/json" \
  -d '{"query": "flipper", "limit": 2}'

# 4. Test full RAG pipeline
curl "http://localhost:8000/ask?query=What is the flipper contract?"
```

## 🎯 What You Can Do

Once set up, you can:

### 💬 **Ask Natural Language Questions**
```bash
# Ask about specific contracts
curl "http://localhost:8000/ask?query=How does the flipper contract work?"

# Ask about implementation patterns  
curl "http://localhost:8000/ask?query=How do I implement an ERC20 token in ink?"

# Ask about best practices
curl "http://localhost:8000/ask?query=What are the security considerations for ink contracts?"
```

### 🔍 **Search Code Examples**
```bash
# Find relevant code chunks
curl -X POST "http://localhost:8000/rag/search" \
  -H "Content-Type: application/json" \
  -d '{"query": "storage mapping AccountId", "limit": 5}'
```

### 📚 **Get Contextual Explanations**
The system will:
1. **Search** through 484 embedded code chunks from ink-examples
2. **Find** the most relevant code for your question
3. **Generate** a natural language explanation using Gemini API
4. **Return** code examples with explanations

### 🧠 **Available Knowledge Base**
The system knows about:
- **Basic Contracts**: flipper, incrementer
- **Token Standards**: ERC20, ERC721, ERC1155  
- **Advanced Patterns**: cross-contract calls, upgradeable contracts
- **Testing**: E2E tests, unit tests
- **Storage**: mappings, lazy storage, optimization
- **Events**: event definitions, topics
- **Error Handling**: custom errors, Result types

## 🎯 Overview

This RAG-powered developer assistant provides:

- **🔍 Semantic Search**: Ask natural language questions about your codebase
- **🧠 RAG System**: Retrieval-Augmented Generation with Gemini API for accurate responses
- **💾 Vector Database**: Qdrant for efficient similarity search and context retrieval
- **🦀 Rust Backend**: Fast, secure API built with Axum and Shuttle.dev
- **📚 Code Understanding**: Embedded ink! smart contract examples ready for queries
- **⚡ Fast Responses**: Sub-second search with intelligent caching
- **🔌 REST API**: Simple `/ask` endpoint for easy integration

### ✅ **Successfully Implemented & Tested**
- **484 code chunks** from ink-examples-main embedded and searchable
- **Gemini API integration** for generating contextual responses  
- **Local Qdrant instance** running and tested
- **Semantic search** finding relevant code with high accuracy
- **REST API endpoints** ready for `/ask` queries

## 🏗️ Architecture

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│   Developer         │────│ RAG Assistant      │────│ Gemini API          │
│   (asks questions)  │    │ (Rust/Axum)        │    │ (text generation)   │
└─────────────────────┘    └─────────────────────┘    └─────────────────────┘
                                        │
                        ┌───────────────┼───────────────┐
                        │               │               │
                        ▼               ▼               ▼
                ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
                │ Code Files  │  │ Qdrant      │  │ OpenAI API  │
                │ (Python     │  │ Vector DB   │  │ (Embeddings)│
                │  embedding) │  │             │  │             │
                │             │  │ • Code      │  │ • text-     │
                │ • Rust      │  │   Chunks    │  │   embedding │
                │ • TypeScript│  │ • Semantic  │  │ • ada-002   │
                │ • JSON/TOML │  │   Search    │  │             │
                │ • Markdown  │  │ • Metadata  │  │             │
                └─────────────┘  └─────────────┘  └─────────────┘
```

### 🔄 RAG Query Flow
```
User Question → Embed Query → Search Qdrant → Retrieve Context → Gemini API → Response
     │              │              │              │              │           │
  "How does    OpenAI ada-002   Vector        Relevant code   Generate      Natural
  flipper      embedding        similarity     chunks from     answer with   language
  work?"       (1536 dims)      search         ink examples    context       response
```

### 💾 Data Architecture

The system uses **2 main data stores**:

#### 1. **Qdrant Vector Database** - Code Knowledge Collection
- **Purpose**: Semantic search through embedded codebase
- **Collection**: `code_knowledge`
- **Distance**: Cosine similarity (for semantic matching)
- **Contents**: 484 code chunks from ink-examples-main
- **Usage**: RAG context retrieval, code search

#### 2. **PostgreSQL Database** (Shuttle Managed) 
- **Purpose**: Application metadata and logs
- **Usage**: User sessions, API logs, system configuration

## 🚀 API Endpoints

### 🎯 Primary Endpoint: Ask Questions About Code

```bash
# GET method with query parameter
GET /ask?query=How does the flipper contract work?

# POST method with JSON body
POST /ask
Content-Type: application/json
{
  "query": "How do I implement an ERC20 token in ink?"
}
```

**Response Format:**
```json
{
  "success": true,
  "data": "The flipper contract is a simple smart contract that stores a boolean value and provides functions to toggle and read this value. Here's how it works...",
  "error": null
}
```

### 🔍 Semantic Search & RAG Endpoints

```bash
# Search code chunks without AI generation
POST /rag/search
Content-Type: application/json
{
  "query": "storage struct definition",
  "limit": 5,
  "score_threshold": 0.7
}

# RAG query with AI response generation
POST /rag/query  
Content-Type: application/json
{
  "query": "How do cross-contract calls work?",
  "limit": 3
}

# Add new documents to knowledge base
POST /rag/document
Content-Type: application/json
{
  "text": "Smart contract code or documentation to add..."
}

# Get system statistics
GET /rag/stats
```

### 🏥 Health Check
```
GET /health
```
```
POST /strategies
Content-Type: application/json

{
  "account": "0x1234...5678",
  "strategy": {
    "name": "My DeFi Strategy",
    "risk_level": 7,
    "parameters": "{\"protocol\": \"Aave\", \"asset\": \"USDC\"}"
  }
}
```

```
GET /strategies/{account_id}
```
Returns all strategies for a specific account.

```
GET /strategies/{account_id}/count
```
Returns the number of strategies for an account.

```
PUT /strategies/{strategy_id}
Content-Type: application/json

{
  "account": "0x1234...5678",
  "strategy": {
    "name": "Updated Strategy",
    "risk_level": 8,
    "parameters": "{\"protocol\": \"Compound\", \"asset\": \"ETH\"}"
  }
}
```

```
DELETE /strategies/{strategy_id}
Content-Type: application/json

{
  "account": "0x1234...5678",
  "strategy_id": "uuid-string"
}
```

### Cross-Chain Functionality
```
POST /cross-chain/strategy
Content-Type: application/json

{
  "account": "0x1234...5678",
  "risk_level": 5,
  "investment_amount": 10000.0,
  "preferred_chains": ["Ethereum", "Polygon"]
}
```

```
GET /cross-chain/opportunities/{risk_level}
```
Returns cross-chain opportunities for a specific risk level.

### AI Chat and RAG System
```
POST /chat
Content-Type: application/json

{
  "message": "What are the best DeFi strategies for low risk?",
  "user_id": "user-123",
  "session_id": "session-456"
}
```

```
POST /rag/search
Content-Type: application/json

{
  "query": "yield farming strategies",
  "limit": 5,
  "score_threshold": 0.7
}
```

```
POST /rag/query
Content-Type: application/json

{
  "query": "What is impermanent loss in DeFi?",
  "limit": 3
}
```

```
POST /rag/document
Content-Type: application/json

{
  "text": "DeFi yield farming involves providing liquidity to decentralized protocols in exchange for rewards..."
}
```

```
GET /rag/stats
```
Returns RAG system statistics (document count, cache size, etc.).

### DeFi Information (Python Backend Compatible)
```
POST /defiInfo
Content-Type: application/json

{
  "input_text": "What is the current APY for USDC on Aave?"
}
```

### Crypto Prices
```
GET /crypto/prices/{tokens}
```
Example: `/crypto/prices/BTC,ETH,USDC`

### Contract Interactions
```
POST /contract/strategy
Content-Type: application/json

{
  "name": "Aave USDC Strategy",
  "description": "Supply USDC to Aave",
  "risk_level": 3,
  "expected_apy": 5.2
}
```

```
POST /contract/invest
Content-Type: application/json

{
  "strategy_id": 1,
  "amount": 1000.0,
  "token": "USDC"
}
```

```
POST /contract/withdraw
Content-Type: application/json

{
  "strategy_id": 1,
  "amount": 500.0,
  "token": "USDC"
}
```

```
GET /contract/strategies/{user_address}
```
Returns user's contract strategies.

### Platform Statistics
```
GET /statistics
```
Returns platform-wide statistics.

## 🛠️ Local Development

### Prerequisites

1. **Install Rust**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install Shuttle CLI**:
```bash
curl -sSfL https://www.shuttle.rs/install | bash
```

3. **Login to Shuttle**:
```bash
shuttle auth login
```

4. **Get API Keys**:
   - **Qdrant**: Use provided credentials or create at [Qdrant Cloud](https://cloud.qdrant.io/)

### Quick Setup

```bash
# Clone and navigate to the project
cd shuttle-backend

# Set up your API keys in Secrets.toml
cp Secrets.toml.example Secrets.toml
# Edit Secrets.toml with your actual API keys

# Build the project
cargo build

# Start the development server
shuttle run

# The server will be available at http://localhost:8000
```

### 🔑 Required Configuration

Edit `Secrets.toml` with your actual API keys:

```toml
# Required: OpenAI API Key for AI features
OPENAI_API_KEY = "sk-your-actual-openai-api-key"

# Required: Qdrant Vector Database
QDRANT_URL = "https://your-cluster.qdrant.io:6334"
QDRANT_API_KEY = "your-qdrant-api-key"

# Optional: Smart Contract Configuration
CONTRACT_ADDRESS = "your-contract-address"
RPC_URL = "wss://moonbeam-alpha.api.onfinality.io/public-ws"
```

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test -- --test-threads=1
```

## 📦 Deployment

### Deploy to Shuttle

```bash
# Deploy using the script
./deploy.sh

# Or manually:
shuttle deploy
```

### Environment Variables

Set these via Shuttle CLI:
```bash
shuttle project env set CONTRACT_ADDRESS="your-contract-address"
shuttle project env set RPC_URL="wss://moonbeam-alpha.api.onfinality.io/public-ws"
shuttle project env set OPENAI_API_KEY="your-openai-api-key"
shuttle project env set QDRANT_URL="your-qdrant-cloud-url"
shuttle project env set QDRANT_API_KEY="your-qdrant-api-key"
```

### Secrets.toml (for local development)
```toml
OPENAI_API_KEY = "sk-your-openai-api-key-here"
QDRANT_URL = "https://your-qdrant-cluster.qdrant.io:6334"
QDRANT_API_KEY = "your-qdrant-api-key"
```

## 🔧 Configuration

### Multi-Database Configuration

The system manages three different storage systems:

#### PostgreSQL Schema
```sql
-- User strategies table
CREATE TABLE strategies (
    id UUID PRIMARY KEY,
    account_id VARCHAR(66) NOT NULL,
    name VARCHAR(255) NOT NULL,
    risk_level INTEGER NOT NULL CHECK (risk_level >= 1 AND risk_level <= 10),
    parameters TEXT NOT NULL,
    contract_strategy_id INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- Additional tables for users, positions, transactions
-- (Created automatically by database migrations)
```

#### Qdrant Collections
```rust
// Knowledge Collection (DeFi documents)
Collection: "defi_knowledge"
Vector Size: 1536 (OpenAI ada-002 embeddings)
Distance: Cosine (document similarity)

// Cache Collection (AI responses)
Collection: "defi_knowledge_cache"  
Vector Size: 1536
Distance: Euclidean (exact matching)
```

#### Sample Data
The system automatically populates with 12 DeFi knowledge documents covering:
- Yield farming strategies
- Impermanent loss
- Popular protocols (Aave, Compound, Uniswap)
- Risk management
- Staking and liquid staking
- Flash loans
- Cross-chain DeFi

## 🔗 Frontend Integration

### Environment Variables

Update your frontend environment variables:

```bash
# .env.local
NEXT_PUBLIC_SHUTTLE_API_URL=https://your-app-name.shuttleapp.rs
NEXT_PUBLIC_CHATBOT_URL=https://your-app-name.shuttleapp.rs
```

### Usage Examples

#### Strategy Management
```javascript
// Save a DeFi strategy
const response = await fetch(`${API_URL}/strategies`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    account: userAddress,
    strategy: {
      name: "Aave USDC Yield Strategy",
      risk_level: 5,
      parameters: JSON.stringify({
        protocol: "Aave",
        asset: "USDC",
        amount: 1000
      })
    }
  })
});

// Get user strategies
const strategies = await fetch(`${API_URL}/strategies/${userAddress}`);
```

#### AI Chat Integration
```javascript
// Chat with AI about DeFi strategies
const chatResponse = await fetch(`${API_URL}/chat`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    message: "What are the best low-risk DeFi strategies?",
    user_id: userId,
    session_id: sessionId
  })
});
```

#### Semantic Search
```javascript
// Search DeFi knowledge base
const searchResults = await fetch(`${API_URL}/rag/search`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    query: "impermanent loss AMM",
    limit: 5,
    score_threshold: 0.7
  })
});
```

#### RAG-Powered Queries
```javascript
// Get AI response with context
const ragResponse = await fetch(`${API_URL}/rag/query`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    query: "How do I minimize impermanent loss in Uniswap?",
    limit: 3
  })
});
```

## 📊 Features

### Current Implementation
- ✅ RESTful API with proper error handling
- ✅ PostgreSQL database integration
- ✅ Qdrant vector database integration
- ✅ AI-powered chat with OpenAI
- ✅ RAG (Retrieval-Augmented Generation) system
- ✅ Semantic search capabilities
- ✅ Semantic caching for cost optimization
- ✅ Document ingestion and embedding
- ✅ Cross-chain strategy generation
- ✅ CORS support for frontend
- ✅ Input validation and sanitization
- ✅ Comprehensive logging
- ✅ Unit and integration tests
- ✅ Automatic database migrations

### Future Enhancements
- 🔄 Full ink! contract integration with subxt
- 🔄 Real-time WebSocket updates
- 🔄 Advanced caching with Redis
- 🔄 Rate limiting and authentication
- 🔄 Metrics and monitoring
- 🔄 Strategy analytics and insights
- 🔄 Multi-language embedding support
- 🔄 Custom fine-tuned models

## 🔒 Security

### Multi-Layer Security
- **Input Validation**: Comprehensive validation for all API endpoints
- **SQL Injection Prevention**: Parameterized queries with SQLx
- **API Key Management**: Secure secret management via Shuttle
- **CORS Configuration**: Proper frontend access control
- **Rate Limiting**: Request size limits and timeout handling
- **Vector Database Security**: Qdrant API key authentication
- **OpenAI API Security**: Secure token management

### Data Protection
- **Sensitive Data**: Never log API keys or user secrets
- **Database Encryption**: PostgreSQL with encrypted connections
- **Vector Data**: Qdrant with TLS encryption
- **Environment Isolation**: Separate dev/prod configurations

## 📈 Performance

### Database Optimization
- **PostgreSQL**: Efficient queries with proper indexing
- **Connection Pooling**: Optimized database connections
- **Vector Search**: Sub-second semantic search with Qdrant
- **Semantic Caching**: Reduces OpenAI API calls by 70-90%

### System Performance
- **Async Operations**: Non-blocking I/O with Tokio
- **Efficient Serialization**: Optimized JSON handling
- **Memory Management**: Rust's zero-cost abstractions
- **Horizontal Scaling**: Automatic scaling via Shuttle.dev

### Cost Optimization
- **Smart Caching**: Semantic cache reduces AI API costs
- **Efficient Embeddings**: Reuse embeddings across queries
- **Resource Pooling**: Shared database connections
- **Lazy Loading**: Load data only when needed

## 🧪 Testing

### Unit Tests
```bash
cargo test test_strategy_validation
cargo test test_health_check
```

### Integration Tests
```bash
# Test with real database
cargo test --test integration_tests
```

### API Testing

#### Quick Test Script
```bash
# Run comprehensive API tests
./test_api_examples.sh
```

#### Manual Testing Examples
```bash
# Test health check
curl -X GET http://localhost:8000/health

# Test AI chat
curl -X POST http://localhost:8000/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What are the best DeFi strategies?", "user_id": "test-user"}'

# Test RAG query with context
curl -X POST http://localhost:8000/rag/query \
  -H "Content-Type: application/json" \
  -d '{"query": "What is impermanent loss?", "limit": 3}'

# Test semantic search
curl -X POST http://localhost:8000/rag/search \
  -H "Content-Type: application/json" \
  -d '{"query": "yield farming", "limit": 5, "score_threshold": 0.7}'

# Test adding document to knowledge base
curl -X POST http://localhost:8000/rag/document \
  -H "Content-Type: application/json" \
  -d '{"text": "DeFi yield farming involves providing liquidity to decentralized protocols..."}'

# Test DeFi info (Python backend compatible)
curl -X POST http://localhost:8000/defiInfo \
  -H "Content-Type: application/json" \
  -d '{"input_text": "What is the current APY for USDC on Aave?"}'

# Test crypto prices
curl -X GET http://localhost:8000/crypto/prices/BTC,ETH,USDC

# Test strategy management
curl -X POST http://localhost:8000/strategies \
  -H "Content-Type: application/json" \
  -d '{"account":"0x123","strategy":{"name":"Test","risk_level":5,"parameters":"{}"}}'

# Test cross-chain strategy
curl -X POST http://localhost:8000/cross-chain/strategy \
  -H "Content-Type: application/json" \
  -d '{"account":"0x123","risk_level":5,"investment_amount":10000.0}'
```

## 📋 Dependencies

### Core Framework
- **shuttle-runtime**: Shuttle.dev runtime
- **shuttle-axum**: Web framework integration
- **shuttle-shared-db**: PostgreSQL database
- **shuttle-qdrant**: Qdrant vector database integration
- **axum**: Modern web framework
- **sqlx**: Async SQL toolkit
- **serde**: Serialization framework
- **tokio**: Async runtime
- **tracing**: Structured logging

### AI and Search
- **qdrant-client**: Qdrant vector database client
- **openai-api-rs**: OpenAI API client for embeddings and chat
- **anyhow**: Error handling
- **uuid**: UUID generation

### Blockchain Integration
- **subxt**: Substrate/Polkadot client
- **ethers**: Ethereum client
- **reqwest**: HTTP client

### Data Processing
- **chrono**: Date/time handling
- **rust_decimal**: Decimal arithmetic
- **rand**: Random number generation

## 🚀 Deployment Guide

### Local Development
```bash
# 1. Set up API keys in Secrets.toml
# 2. Build and run
cargo build
shuttle run
```

### Production Deployment
```bash
# Deploy to Shuttle
shuttle deploy

# Set production secrets
shuttle project env set OPENAI_API_KEY="sk-your-key"
shuttle project env set QDRANT_URL="https://your-cluster.qdrant.io:6334"
shuttle project env set QDRANT_API_KEY="your-key"
```

## 🔄 Data Flow

### RAG Query Processing
```
User Query → Cache Check → Knowledge Search → OpenAI → Cache Store → Response
     ↓            ↓             ↓            ↓         ↓          ↓
  "yield      Cache Miss    Find docs    Generate   Store      Return
  farming"    (Qdrant)     (Qdrant)     response   result     to user
                                        (OpenAI)   (Qdrant)
```

### Strategy Persistence
```
AI Recommendation → User Approval → Database Storage → Contract Interaction
        ↓                ↓               ↓                    ↓
   "Aave USDC         User clicks     PostgreSQL           ink! Contract
   strategy"          "Save"          strategies           (Moonbeam)
```

## 🧠 AI Features

### Semantic Search
- **Vector Embeddings**: OpenAI text-embedding-ada-002
- **Similarity Matching**: Cosine similarity for documents
- **Context Retrieval**: Relevant DeFi knowledge for queries
- **Metadata Filtering**: Search by category, difficulty, topic

### Smart Caching
- **Query Similarity**: Detect similar questions
- **Cost Reduction**: Avoid duplicate API calls
- **Performance**: Sub-second responses for cached queries
- **Cache Invalidation**: Automatic cleanup of old entries

### RAG System
- **Context Injection**: Add relevant docs to prompts
- **Knowledge Base**: Pre-loaded DeFi expertise
- **Dynamic Updates**: Add new knowledge via API
- **Quality Control**: Scored results with thresholds

## 🤝 Contributing

### Development Setup
1. Fork the repository
2. Set up development environment (see SETUP_GUIDE.md)
3. Create a feature branch
4. Run tests: `cargo test`
5. Submit a pull request

### Code Standards
- Follow Rust best practices
- Add tests for new features
- Update documentation
- Use semantic commit messages

## 📞 Support

### Documentation
- **Setup Guide**: [SETUP_GUIDE.md](./SETUP_GUIDE.md) - Complete setup instructions
- **API Testing**: [test_api_examples.sh](./test_api_examples.sh) - Comprehensive API tests
- **Integration Guide**: [../docs/INTEGRATION-PLAN.md](../docs/INTEGRATION-PLAN.md) - Full integration plan

### Community
- **GitHub Issues**: [DynaVest Repository](https://github.com/LI-YONG-QI/agentic-hack)
- **Shuttle Discord**: [Shuttle Community](https://discord.gg/shuttle)
- **Documentation**: [Shuttle.dev Docs](https://docs.shuttle.rs)

### Quick Links
- **OpenAI API**: [OpenAI Platform](https://platform.openai.com/api-keys)
- **Qdrant Cloud**: [Qdrant Cloud Console](https://cloud.qdrant.io/)
- **Polkadot Docs**: [Polkadot Developer Hub](https://polkadot.network/development/)

---

## 🎯 Summary

**DynaVest Shuttle Backend** is a production-ready AI-powered DeFi platform featuring:

✅ **Semantic Search** with Qdrant vector database  
✅ **Smart Caching** for cost optimization  
✅ **RAG System** for contextual AI responses  
✅ **Multi-Database Architecture** (PostgreSQL + Qdrant)  
✅ **Polkadot Integration** with ink! smart contracts  
✅ **Comprehensive API** with 15+ endpoints  
✅ **Auto-Scaling** deployment on Shuttle.dev  

Built with ❤️ using Rust, Shuttle.dev, and modern AI technology for the Polkadot ecosystem.

---

## 🔗 Quick Reference: curl Commands

### ✅ Correct Testing Commands

```bash
# 1. Health check
curl http://localhost:8000/health

# 2. Basic migration question (with jq for clean output)
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How does the flipper contract work?" | jq -r '.data'

# 3. ERC20 migration question
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I migrate ERC20 tokens from Solidity to ink!?" | jq -r '.data'

# 4. Comparison question
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=What are the key differences between Solidity and ink!?" | jq -r '.data'

# 5. Event handling question
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=Show me event handling examples" | jq -r '.data'

# 6. Multisig wallet question
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I implement multisig wallets in ink!?" | jq -r '.data'

# 7. POST method (alternative)
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How do I convert Solidity mappings to ink! storage?"}' | jq -r '.data'
```

### ❌ Common Mistakes to Avoid

```bash
# DON'T: This will fail with "URL rejected: Malformed input"
curl "http://localhost:8000/ask?query=How does the flipper contract work?"

# DON'T: Missing Content-Type header for POST
curl -X POST "http://localhost:8000/ask" -d '{"query": "test"}'
```

### 🐚 **Shell Compatibility (zsh/bash)**

**Problem**: `!` character causes issues in zsh
```bash
# ❌ This fails in zsh: "no such event"
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I implement ERC20 tokens in ink!?" | jq -r '.data'
```

**Solutions**:
```bash
# ✅ Solution 1: Use single quotes
curl -X GET 'http://localhost:8000/ask' -G \
  --data-urlencode 'query=How do I implement ERC20 tokens in ink!?' | jq -r '.data'

# ✅ Solution 2: Escape the exclamation mark
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I implement ERC20 tokens in ink\!" | jq -r '.data'

# ✅ Solution 3: Use POST method (no URL encoding issues)
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How do I implement ERC20 tokens in ink!?"}' | jq -r '.data'

# ✅ Solution 4: Omit the exclamation mark
curl -X GET "http://localhost:8000/ask" -G \
  --data-urlencode "query=How do I implement ERC20 tokens in ink" | jq -r '.data'
```

### 🎯 Response Types You'll Get

- **📊 Comparison Tables**: Side-by-side Solidity vs ink! feature comparisons
- **📝 Step-by-step Guides**: Detailed migration instructions
- **💻 Code Examples**: Real before/after contract implementations  
- **⚠️ Best Practices**: Security, testing, and optimization tips
- **🚀 Deployment Guidance**: How to deploy and test migrated contracts

### 🛠️ Troubleshooting

**"Connection refused"**: Make sure your Rust server is running with `cargo run --bin dynavest-shuttle-backend`

**"Gemini API slow/unavailable"**: API rate limiting - wait a moment and try again

**Empty/error responses**: Check that Qdrant is running with `docker ps | grep qdrant`

---

**Ready to migrate your smart contracts? Start asking questions!** 🚀