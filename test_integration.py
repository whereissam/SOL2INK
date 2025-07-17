#!/usr/bin/env python3
"""
Test script to verify frontend-backend integration for SOL2INK Migration Assistant.
"""

import requests
import json
import time
import sys

def test_backend_endpoints():
    """Test backend endpoints that the frontend uses."""
    base_url = "http://localhost:8000"
    
    print("🧪 Testing Backend Endpoints")
    print("=" * 50)
    
    # Test health endpoint
    print("\n1. Testing /health endpoint...")
    try:
        response = requests.get(f"{base_url}/health", timeout=5)
        if response.status_code == 200:
            data = response.json()
            if data["success"]:
                print(f"✅ Health check: {data['data']}")
            else:
                print(f"❌ Health check failed: {data.get('error', 'Unknown error')}")
        else:
            print(f"❌ Health check HTTP error: {response.status_code}")
    except Exception as e:
        print(f"❌ Health check exception: {e}")
        print("⚠️  Backend might not be running. Start it with:")
        print("   cd shuttle-backend && cargo run")
        return False
    
    # Test ask endpoint (this is what the frontend uses)
    print("\n2. Testing /ask endpoint...")
    test_query = "How do I migrate ERC20 tokens from Solidity to ink?"
    
    try:
        response = requests.post(f"{base_url}/ask", 
                               json={"query": test_query}, 
                               timeout=30)
        if response.status_code == 200:
            data = response.json()
            if data["success"]:
                print(f"✅ Ask endpoint: Response received ({len(data['data'])} chars)")
                print(f"   Preview: {data['data'][:100]}...")
            else:
                print(f"❌ Ask endpoint error: {data.get('error', 'Unknown error')}")
        else:
            print(f"❌ Ask endpoint HTTP error: {response.status_code}")
    except Exception as e:
        print(f"❌ Ask endpoint exception: {e}")
    
    print("\n✅ Backend endpoint testing completed!")
    return True

def test_frontend_connection():
    """Test if frontend can be reached."""
    frontend_url = "http://localhost:5173"  # Default Vite dev server port
    
    print("\n🌐 Testing Frontend Connection")
    print("=" * 50)
    
    try:
        response = requests.get(frontend_url, timeout=5)
        if response.status_code == 200:
            print(f"✅ Frontend reachable at {frontend_url}")
            return True
        else:
            print(f"❌ Frontend HTTP error: {response.status_code}")
    except Exception as e:
        print(f"❌ Frontend connection failed: {e}")
        print("⚠️  Frontend might not be running. Start it with:")
        print("   cd SOL2INK-frontend && npm run dev")
    
    return False

def test_integration_workflow():
    """Test the complete integration workflow."""
    print("\n🔄 Testing Integration Workflow")
    print("=" * 50)
    
    # Simulate frontend request to backend
    backend_url = "http://localhost:8000"
    
    # Test the exact request the frontend makes
    test_queries = [
        "How do I migrate ERC20 tokens from Solidity to ink?",
        "What are the key differences between Solidity and ink!?",
        "Show me event handling examples in both languages"
    ]
    
    for i, query in enumerate(test_queries, 1):
        print(f"\n{i}. Testing query: {query[:50]}...")
        
        try:
            # This mimics what the frontend MigrationAssistant component does
            response = requests.post(f"{backend_url}/ask", 
                                   json={"query": query},
                                   headers={'Content-Type': 'application/json'},
                                   timeout=30)
            
            if response.status_code == 200:
                data = response.json()
                if data["success"] and data["data"]:
                    print(f"✅ Query {i}: Success ({len(data['data'])} chars)")
                else:
                    print(f"❌ Query {i}: {data.get('error', 'No data returned')}")
            else:
                print(f"❌ Query {i}: HTTP {response.status_code}")
                
        except Exception as e:
            print(f"❌ Query {i}: Exception - {e}")
        
        time.sleep(1)  # Rate limiting

def main():
    """Main test function."""
    print("🚀 SOL2INK Migration Assistant Integration Test")
    print("=" * 60)
    
    # Test backend
    backend_ok = test_backend_endpoints()
    
    # Test frontend
    frontend_ok = test_frontend_connection()
    
    if backend_ok:
        # Test integration workflow
        test_integration_workflow()
    
    print("\n📊 Test Summary")
    print("=" * 30)
    print(f"Backend:  {'✅ OK' if backend_ok else '❌ FAIL'}")
    print(f"Frontend: {'✅ OK' if frontend_ok else '❌ FAIL'}")
    
    if backend_ok and frontend_ok:
        print("\n🎉 Integration test completed successfully!")
        print("\n📝 Next steps:")
        print("1. Open frontend at http://localhost:5173")
        print("2. Test the migration assistant with various queries")
        print("3. Verify error handling by stopping the backend")
        print("4. Check connection status indicator")
    else:
        print("\n⚠️  Some issues found. Please check the services.")
        
    return 0 if backend_ok else 1

if __name__ == "__main__":
    sys.exit(main())