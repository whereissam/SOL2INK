#!/bin/bash

# Development script for DynaVest Shuttle Backend

echo "🛠️ Starting DynaVest Shuttle Backend Development..."

# Check if shuttle CLI is installed
if ! command -v shuttle &> /dev/null; then
    echo "❌ Shuttle CLI not found. Installing..."
    curl -sSfL https://www.shuttle.rs/install | bash
    source ~/.bashrc
fi

# Function to run tests
run_tests() {
    echo "🧪 Running tests..."
    cargo test
    echo "✅ Tests completed!"
}

# Function to start development server
start_dev() {
    echo "🚀 Starting development server..."
    
    # Load environment variables from Secrets.toml
    if [ -f "Secrets.toml" ]; then
        echo "🔑 Loading API keys from Secrets.toml..."
        export OPENAI_API_KEY=$(grep "OPENAI_API_KEY" Secrets.toml | cut -d '"' -f 2)
        export QDRANT_URL=$(grep "QDRANT_URL" Secrets.toml | cut -d '"' -f 2)
        export QDRANT_API_KEY=$(grep "QDRANT_API_KEY" Secrets.toml | cut -d '"' -f 2)
        echo "✅ API keys loaded"
    else
        echo "⚠️  Secrets.toml not found. Using default values."
    fi
    
    echo "📍 Server will be available at: http://localhost:8000"
    echo "📋 Available endpoints:"
    echo "  - GET  /health - Health check"
    echo "  - POST /strategies - Save strategy"
    echo "  - GET  /strategies/:account - Get strategies"
    echo "  - GET  /strategies/:account/count - Get count"
    echo "  - GET  /statistics - Platform stats"
    echo "  - POST /chat - AI chat"
    echo "  - POST /rag/query - RAG query"
    echo "  - POST /rag/search - Semantic search"
    echo "  - POST /rag/document - Add document"
    echo "  - GET  /rag/stats - RAG statistics"
    echo ""
    shuttle run
}

# Function to test API endpoints
test_api() {
    echo "🔍 Testing API endpoints..."
    echo "Testing health check..."
    curl -s http://localhost:8000/health | jq '.'
    
    echo -e "\nTesting statistics..."
    curl -s http://localhost:8000/statistics | jq '.'
    
    echo -e "\nTesting strategy creation..."
    curl -s -X POST http://localhost:8000/strategies \
        -H "Content-Type: application/json" \
        -d '{
            "account": "0x1234567890abcdef",
            "strategy": {
                "name": "Test Strategy",
                "risk_level": 5,
                "parameters": "{\"protocol\": \"Aave\", \"asset\": \"USDC\"}"
            }
        }' | jq '.'
    
    echo -e "\nTesting strategy retrieval..."
    curl -s http://localhost:8000/strategies/0x1234567890abcdef | jq '.'
}

# Parse command line arguments
case "$1" in
    "test")
        run_tests
        ;;
    "dev")
        start_dev
        ;;
    "api-test")
        test_api
        ;;
    *)
        echo "Usage: $0 {test|dev|api-test}"
        echo "  test     - Run unit tests"
        echo "  dev      - Start development server"
        echo "  api-test - Test API endpoints (requires server to be running)"
        exit 1
        ;;
esac