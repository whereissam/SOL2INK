#!/bin/bash

# DynaVest Shuttle Backend API Test Examples
# Run with: chmod +x test_api_examples.sh && ./test_api_examples.sh

API_URL="http://localhost:8000"

echo "🚀 Testing DynaVest Shuttle Backend API"
echo "========================================"

# Test Health Check
echo "1. Health Check"
curl -s "$API_URL/health" | jq .
echo -e "\n"

# Test Chat with AI
echo "2. AI Chat Test"
curl -s -X POST "$API_URL/chat" \
  -H "Content-Type: application/json" \
  -d '{
    "message": "What are the best DeFi strategies for beginners?",
    "user_id": "test-user-123",
    "session_id": "test-session-456"
  }' | jq .
echo -e "\n"

# Test RAG Query
echo "3. RAG Query Test"
curl -s -X POST "$API_URL/rag/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is impermanent loss in DeFi?",
    "limit": 3
  }' | jq .
echo -e "\n"

# Test Add Document to RAG
echo "4. Add Document to RAG"
curl -s -X POST "$API_URL/rag/document" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "DeFi yield farming is a strategy where cryptocurrency holders provide liquidity to decentralized protocols in exchange for rewards. Common protocols include Uniswap, Aave, and Compound."
  }' | jq .
echo -e "\n"

# Test Semantic Search
echo "5. Semantic Search Test"
curl -s -X POST "$API_URL/rag/search" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "yield farming strategies",
    "limit": 3,
    "score_threshold": 0.7
  }' | jq .
echo -e "\n"

# Test RAG Stats
echo "6. RAG System Statistics"
curl -s "$API_URL/rag/stats" | jq .
echo -e "\n"

# Test DeFi Info (Python backend compatible)
echo "7. DeFi Information Query"
curl -s -X POST "$API_URL/defiInfo" \
  -H "Content-Type: application/json" \
  -d '{
    "input_text": "What is the current APY for USDC on Aave?"
  }' | jq .
echo -e "\n"

# Test Crypto Prices
echo "8. Crypto Prices"
curl -s "$API_URL/crypto/prices/BTC,ETH,USDC" | jq .
echo -e "\n"

# Test Save Strategy
echo "9. Save Strategy"
curl -s -X POST "$API_URL/strategies" \
  -H "Content-Type: application/json" \
  -d '{
    "account": "0x1234567890abcdef1234567890abcdef12345678",
    "strategy": {
      "name": "Test DeFi Strategy",
      "risk_level": 5,
      "parameters": "{\"protocol\": \"Aave\", \"asset\": \"USDC\", \"amount\": 1000}"
    }
  }' | jq .
echo -e "\n"

# Test Get Strategies
echo "10. Get Strategies"
curl -s "$API_URL/strategies/0x1234567890abcdef1234567890abcdef12345678" | jq .
echo -e "\n"

# Test Cross-Chain Strategy
echo "11. Cross-Chain Strategy Generation"
curl -s -X POST "$API_URL/cross-chain/strategy" \
  -H "Content-Type: application/json" \
  -d '{
    "account": "0x1234567890abcdef1234567890abcdef12345678",
    "risk_level": 5,
    "investment_amount": 10000.0,
    "preferred_chains": ["Ethereum", "Polygon"]
  }' | jq .
echo -e "\n"

# Test Platform Statistics
echo "12. Platform Statistics"
curl -s "$API_URL/statistics" | jq .
echo -e "\n"

echo "✅ All API tests completed!"
echo "📚 Check the README.md for more detailed API documentation"