#!/usr/bin/env python3
"""
Test script for the Solidity to ink! training system
"""

import requests
import json
import time

BASE_URL = "http://localhost:8000"  # Adjust if your server runs on a different port

def test_get_contract_pairs():
    """Test getting available contract pairs"""
    print("🔍 Testing contract pairs endpoint...")
    
    response = requests.get(f"{BASE_URL}/training/contract-pairs")
    
    if response.status_code == 200:
        data = response.json()
        if data.get("success"):
            pairs = data.get("data", [])
            print(f"✅ Found {len(pairs)} contract pairs:")
            for pair in pairs:
                print(f"  - {pair}")
        else:
            print(f"❌ API returned error: {data.get('error')}")
    else:
        print(f"❌ Request failed with status {response.status_code}")
        print(f"Response: {response.text}")

def test_embed_contract_pairs():
    """Test embedding contract pairs into vector database"""
    print("\n🧠 Testing contract embedding endpoint...")
    
    response = requests.post(f"{BASE_URL}/training/embed-contracts")
    
    if response.status_code == 200:
        data = response.json()
        if data.get("success"):
            result = data.get("data", {})
            print(f"✅ Embedding successful!")
            print(f"  - Processed pairs: {result.get('processed_pairs', 0)}")
            print(f"  - Document IDs: {len(result.get('document_ids', []))}")
            
            errors = result.get('errors', [])
            if errors:
                print(f"  - Errors: {len(errors)}")
                for error in errors:
                    print(f"    • {error}")
        else:
            print(f"❌ API returned error: {data.get('error')}")
    else:
        print(f"❌ Request failed with status {response.status_code}")
        print(f"Response: {response.text}")

def test_query_erc20_migration():
    """Test querying the trained system about ERC20 migration"""
    print("\n💬 Testing ERC20 migration query...")
    
    query = "How do I implement ERC20 in ink! if I know Solidity?"
    
    response = requests.get(f"{BASE_URL}/ask", params={"query": query})
    
    if response.status_code == 200:
        data = response.json()
        if data.get("success"):
            answer = data.get("data", "")
            print(f"✅ Query successful!")
            print(f"Query: {query}")
            print(f"Answer: {answer[:500]}..." if len(answer) > 500 else f"Answer: {answer}")
        else:
            print(f"❌ API returned error: {data.get('error')}")
    else:
        print(f"❌ Request failed with status {response.status_code}")
        print(f"Response: {response.text}")

def test_query_flipper_migration():
    """Test querying about Flipper contract migration"""
    print("\n💬 Testing Flipper migration query...")
    
    query = "Show me how to convert a Solidity Flipper contract to ink!"
    
    response = requests.get(f"{BASE_URL}/ask", params={"query": query})
    
    if response.status_code == 200:
        data = response.json()
        if data.get("success"):
            answer = data.get("data", "")
            print(f"✅ Query successful!")
            print(f"Query: {query}")
            print(f"Answer: {answer[:500]}..." if len(answer) > 500 else f"Answer: {answer}")
        else:
            print(f"❌ API returned error: {data.get('error')}")
    else:
        print(f"❌ Request failed with status {response.status_code}")
        print(f"Response: {response.text}")

def test_server_health():
    """Test if server is running"""
    print("🏥 Testing server health...")
    
    try:
        response = requests.get(f"{BASE_URL}/health", timeout=5)
        if response.status_code == 200:
            print("✅ Server is running")
            return True
        else:
            print(f"❌ Server returned status {response.status_code}")
            return False
    except requests.exceptions.RequestException as e:
        print(f"❌ Cannot connect to server: {e}")
        return False

def main():
    """Main test function"""
    print("🚀 Testing Solidity to ink! Training System")
    print("=" * 50)
    
    # Test server health
    if not test_server_health():
        print("\n❌ Server is not running. Please start the server first.")
        print("Run: cargo run --bin dynavest-shuttle-backend")
        return
    
    # Test getting contract pairs
    test_get_contract_pairs()
    
    # Test embedding contract pairs
    test_embed_contract_pairs()
    
    # Wait a bit for embedding to complete
    print("\n⏳ Waiting for embedding to complete...")
    time.sleep(2)
    
    # Test querying the trained system
    test_query_erc20_migration()
    test_query_flipper_migration()
    
    print("\n🎉 Training system tests completed!")

if __name__ == "__main__":
    main()