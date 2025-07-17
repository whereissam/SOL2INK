#!/bin/bash

# Setup script for RAG Developer Assistant embeddings

echo "🚀 Setting up RAG Developer Assistant embeddings..."

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    exit 1
fi

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "📦 Creating Python virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
echo "🔧 Activating virtual environment..."
source venv/bin/activate

# Install dependencies
echo "📚 Installing Python dependencies..."
pip install -r requirements.txt

# Load environment variables
if [ -f ".env" ]; then
    echo "🔧 Loading environment variables..."
    export $(cat .env | xargs)
fi

# Test Qdrant connection
echo "🧪 Testing Qdrant connection..."
python3 test_qdrant.py

if [ $? -eq 0 ]; then
    echo "✅ Qdrant connection test passed!"
    
    # Ask user for codebase directory
    echo ""
    echo "📁 Enter the path to your codebase directory:"
    read -r CODEBASE_DIR
    
    if [ -d "$CODEBASE_DIR" ]; then
        echo "🧠 Embedding codebase: $CODEBASE_DIR"
        echo "This may take a few minutes depending on the size of your codebase..."
        
        python3 embed_codebase.py "$CODEBASE_DIR" \
            --qdrant-url "${QDRANT_URL:-http://localhost:6334}" \
            --qdrant-api-key "${QDRANT_API_KEY}" \
            --collection "${COLLECTION_NAME:-code_knowledge}" \
            --test-query "main function"
        
        if [ $? -eq 0 ]; then
            echo "🎉 Codebase embedding completed successfully!"
            echo "You can now start the Rust server and use the /ask endpoint."
        else
            echo "❌ Codebase embedding failed. Check the logs above."
        fi
    else
        echo "❌ Directory '$CODEBASE_DIR' does not exist."
    fi
else
    echo "❌ Qdrant connection test failed. Please check your configuration."
fi

# Deactivate virtual environment
deactivate

echo "✨ Setup complete!"