#\!/bin/bash
# Script to run the server with proper environment variables

export GEMINI_API_KEY="AIzaSyBxCDT1vtb0h220IiO7AjobnjhdVVMRx-c"
export QDRANT_URL="http://localhost:6334"
export RUST_LOG="info"

echo "Starting server with Gemini API only..."
echo "GEMINI_API_KEY: $GEMINI_API_KEY"
echo "QDRANT_URL: $QDRANT_URL"

# Start the server using shuttle run
shuttle run
EOF < /dev/null