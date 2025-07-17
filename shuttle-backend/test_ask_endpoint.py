#!/usr/bin/env python3
"""
Test script to verify the /ask endpoint works with embedded ink examples.
"""

import requests
import json
import time

def test_ask_endpoint():
    """Test the /ask endpoint with ink smart contract queries."""
    base_url = "http://localhost:8000"
    
    # Test queries about ink smart contracts
    test_queries = [
        "What is the flipper contract and how does it work?",
        "Show me how to implement an ERC20 token in ink",
        "How do I create a storage struct in ink smart contracts?",
        "What are the main functions in the incrementer contract?",
        "How do cross-contract calls work in ink?"
    ]
    
    print("🧪 Testing /ask endpoint with ink smart contract queries")
    print("=" * 60)
    
    for i, query in enumerate(test_queries, 1):
        print(f"\n{i}. Query: {query}")
        print("-" * 40)
        
        # Test GET endpoint
        try:
            response = requests.get(f"{base_url}/ask", params={"query": query}, timeout=30)
            if response.status_code == 200:
                data = response.json()
                if data["success"]:
                    print(f"✅ GET /ask: {data['data'][:200]}...")
                else:
                    print(f"❌ GET /ask Error: {data.get('error', 'Unknown error')}")
            else:
                print(f"❌ GET /ask HTTP Error: {response.status_code}")
        except Exception as e:
            print(f"❌ GET /ask Exception: {e}")
        
        # Test POST endpoint
        try:
            response = requests.post(f"{base_url}/ask", 
                                   json={"query": query}, 
                                   timeout=30)
            if response.status_code == 200:
                data = response.json()
                if data["success"]:
                    print(f"✅ POST /ask: {data['data'][:200]}...")
                else:
                    print(f"❌ POST /ask Error: {data.get('error', 'Unknown error')}")
            else:
                print(f"❌ POST /ask HTTP Error: {response.status_code}")
        except Exception as e:
            print(f"❌ POST /ask Exception: {e}")
        
        # Small delay between requests
        time.sleep(1)
    
    print("\n🏁 Test completed!")

if __name__ == "__main__":
    test_ask_endpoint()