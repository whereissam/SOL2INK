#!/bin/bash

# Deployment verification script for DynaVest Shuttle Backend

echo "🔍 Verifying DynaVest Shuttle Backend Deployment..."

# Default URL (can be overridden with environment variable)
API_URL="${SHUTTLE_API_URL:-https://dynavest-shuttle-backend.shuttleapp.rs}"

echo "📡 Testing API at: $API_URL"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to test an endpoint
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local expected_status=$4
    
    echo -n "Testing $method $endpoint... "
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "%{http_code}" -o /tmp/response.json "$API_URL$endpoint" 2>/dev/null)
    elif [ "$method" = "POST" ]; then
        response=$(curl -s -w "%{http_code}" -o /tmp/response.json -X POST -H "Content-Type: application/json" -d "$data" "$API_URL$endpoint" 2>/dev/null)
    fi
    
    http_code="${response: -3}"
    
    if [ "$http_code" = "$expected_status" ]; then
        echo -e "${GREEN}✅ PASS${NC} (HTTP $http_code)"
        if [ -f /tmp/response.json ]; then
            echo "   Response: $(cat /tmp/response.json | head -c 100)..."
        fi
    else
        echo -e "${RED}❌ FAIL${NC} (HTTP $http_code, expected $expected_status)"
        if [ -f /tmp/response.json ]; then
            echo "   Response: $(cat /tmp/response.json)"
        fi
    fi
    
    echo ""
}

# Test deployment readiness
echo "🏥 Health Check"
test_endpoint "GET" "/health" "" "200"

echo "📊 Statistics Endpoint"
test_endpoint "GET" "/statistics" "" "200"

echo "📦 Strategy Endpoints"
# Test GET strategies for a sample account
test_endpoint "GET" "/strategies/0x1234567890123456789012345678901234567890" "" "200"

# Test POST strategy (create)
strategy_data='{
  "account": "0x1234567890123456789012345678901234567890",
  "strategy": {
    "name": "Deployment Test Strategy",
    "risk_level": 5,
    "parameters": "This is a test strategy created during deployment verification"
  }
}'

test_endpoint "POST" "/strategies" "$strategy_data" "201"

echo "🔗 CORS Configuration"
echo -n "Testing CORS preflight... "
cors_response=$(curl -s -w "%{http_code}" -o /tmp/cors.json -X OPTIONS -H "Origin: https://dynavest.app" -H "Access-Control-Request-Method: POST" "$API_URL/strategies" 2>/dev/null)
cors_code="${cors_response: -3}"

if [ "$cors_code" = "200" ] || [ "$cors_code" = "204" ]; then
    echo -e "${GREEN}✅ PASS${NC} (HTTP $cors_code)"
else
    echo -e "${YELLOW}⚠️  CORS might not be configured${NC} (HTTP $cors_code)"
fi

echo ""

# Summary
echo "📋 Deployment Verification Summary"
echo "=================================="
echo "API URL: $API_URL"
echo "Timestamp: $(date)"

# Check if the main URL is accessible
if curl -s --head "$API_URL/health" | head -n 1 | grep -q "200 OK"; then
    echo -e "Status: ${GREEN}✅ DEPLOYMENT VERIFIED${NC}"
    echo ""
    echo "🎉 Your DynaVest Shuttle Backend is live and working!"
    echo ""
    echo "Next steps:"
    echo "1. Update your frontend NEXT_PUBLIC_SHUTTLE_API_URL to: $API_URL"
    echo "2. Test the frontend integration"
    echo "3. Configure any production environment variables"
else
    echo -e "Status: ${RED}❌ DEPLOYMENT ISSUES DETECTED${NC}"
    echo ""
    echo "Troubleshooting steps:"
    echo "1. Check if the backend is deployed: shuttle project status"
    echo "2. Check deployment logs: shuttle logs"
    echo "3. Verify environment variables: shuttle project env list"
    echo "4. Re-deploy if necessary: ./deploy.sh"
fi

echo ""
echo "For more help, visit: https://docs.shuttle.rs"

# Cleanup
rm -f /tmp/response.json /tmp/cors.json