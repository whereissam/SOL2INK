#!/bin/bash

# SOL2INK Backend Startup Script
# This script starts the backend on the configured port

# Load environment variables
source .env

# Override port if specified
export PORT=${PORT:-8000}

echo "🚀 Starting SOL2INK Backend on port $PORT"
echo "   Make sure Qdrant is running on port 6334"
echo "   docker run -p 6334:6334 qdrant/qdrant"
echo ""

# Start the backend
cargo run --bin dynavest-shuttle-backend