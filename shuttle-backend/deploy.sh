#!/bin/bash

# Deployment script for DynaVest Shuttle Backend

echo "🚀 Deploying DynaVest Shuttle Backend..."

# Check if shuttle CLI is installed
if ! command -v shuttle &> /dev/null; then
    echo "❌ Shuttle CLI not found. Installing..."
    curl -sSfL https://www.shuttle.rs/install | bash
    source ~/.bashrc
fi

# Check if user is logged in
if ! shuttle auth status &> /dev/null; then
    echo "🔐 Please login to Shuttle first:"
    shuttle auth login
fi

# Set environment variables
echo "🔧 Setting environment variables..."
shuttle project env set CONTRACT_ADDRESS="${CONTRACT_ADDRESS:-5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY}"
shuttle project env set RPC_URL="${RPC_URL:-wss://moonbeam-alpha.api.onfinality.io/public-ws}"

# Deploy the backend
echo "📦 Deploying backend..."
shuttle deploy

if [ $? -eq 0 ]; then
    echo "✅ Backend deployed successfully!"
    echo ""
    echo "🎉 Your DynaVest Shuttle Backend is now live!"
    echo "📋 API endpoints available:"
    echo "  - GET  /health - Health check"
    echo "  - POST /strategies - Save strategy"
    echo "  - GET  /strategies/:account - Get strategies"
    echo "  - GET  /strategies/:account/count - Get count"
    echo "  - GET  /statistics - Platform stats"
    echo ""
    echo "🔗 Update your frontend NEXT_PUBLIC_SHUTTLE_API_URL to point to the deployed URL"
else
    echo "❌ Deployment failed!"
    exit 1
fi