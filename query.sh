#!/bin/bash

# Usage: ./query.sh "your query here"
if [ $# -eq 0 ]; then
    echo "Usage: $0 \"your query\""
    exit 1
fi

echo "Querying: $1"
echo "================================"
curl -G "http://localhost:8000/ask" --data-urlencode "query=$1" 2>/dev/null | jq -r '.data'