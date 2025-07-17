#!/bin/bash

echo "🔧 Setting up API Keys from Secrets.toml..."

# Read secrets from Secrets.toml and export them
if [ -f "Secrets.toml" ]; then
    echo "📁 Found Secrets.toml file"
    
    # Extract and export environment variables
    export OPENAI_API_KEY=$(grep "OPENAI_API_KEY" Secrets.toml | cut -d '"' -f 2)
    export QDRANT_URL=$(grep "QDRANT_URL" Secrets.toml | cut -d '"' -f 2)
    export QDRANT_API_KEY=$(grep "QDRANT_API_KEY" Secrets.toml | cut -d '"' -f 2)
    
    echo "✅ Environment variables exported"
    echo "OPENAI_API_KEY: ${OPENAI_API_KEY:0:10}..."
    echo "QDRANT_URL: $QDRANT_URL"
    echo "QDRANT_API_KEY: ${QDRANT_API_KEY:0:10}..."
    
    # Test the connection
    echo "🧪 Testing connection..."
    cargo run --bin test_connection
    
    echo "🚀 You can now run the server with:"
    echo "export OPENAI_API_KEY='$OPENAI_API_KEY'"
    echo "export QDRANT_URL='$QDRANT_URL'"
    echo "export QDRANT_API_KEY='$QDRANT_API_KEY'"
    echo "shuttle run"
else
    echo "❌ Secrets.toml file not found"
    echo "Please create Secrets.toml with your API keys"
fi