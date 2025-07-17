#!/usr/bin/env python3
"""
Test script to verify Qdrant setup and connection.
"""

import os
import sys
from typing import Optional

try:
    from qdrant_client import QdrantClient
    from sentence_transformers import SentenceTransformer
except ImportError as e:
    print(f"Error importing required packages: {e}")
    print("Install with: pip install qdrant-client sentence-transformers")
    sys.exit(1)

def test_qdrant_connection(qdrant_url: str, api_key: Optional[str] = None):
    """Test connection to Qdrant."""
    print(f"Testing connection to Qdrant at: {qdrant_url}")
    
    try:
        # Create client
        if api_key:
            client = QdrantClient(url=qdrant_url, api_key=api_key)
        else:
            client = QdrantClient(url=qdrant_url)
        
        # Test connection by listing collections
        collections = client.get_collections()
        print(f"✅ Successfully connected to Qdrant!")
        print(f"📊 Found {len(collections.collections)} collections:")
        
        for collection in collections.collections:
            collection_info = client.get_collection(collection.name)
            points_count = collection_info.points_count or 0
            print(f"  - {collection.name}: {points_count} points")
        
        return True
        
    except Exception as e:
        print(f"❌ Failed to connect to Qdrant: {e}")
        return False

def test_sentence_transformer():
    """Test sentence transformer model."""
    print("\n🧠 Testing sentence transformer model...")
    
    try:
        model = SentenceTransformer('all-MiniLM-L6-v2')
        
        # Test embedding
        test_text = "This is a test sentence for embedding."
        embedding = model.encode([test_text])
        
        print(f"✅ Successfully loaded sentence transformer!")
        print(f"📏 Embedding dimension: {len(embedding[0])}")
        
        return True
        
    except Exception as e:
        print(f"❌ Failed to load sentence transformer: {e}")
        return False

def test_basic_search(qdrant_url: str, api_key: Optional[str] = None, collection_name: str = "code_knowledge"):
    """Test basic search functionality."""
    print(f"\n🔍 Testing search in collection: {collection_name}")
    
    try:
        # Create client
        if api_key:
            client = QdrantClient(url=qdrant_url, api_key=api_key)
        else:
            client = QdrantClient(url=qdrant_url)
        
        # Check if collection exists
        collections = client.get_collections()
        collection_names = [c.name for c in collections.collections]
        
        if collection_name not in collection_names:
            print(f"⚠️  Collection '{collection_name}' not found. Available collections: {collection_names}")
            return False
        
        # Get collection info
        collection_info = client.get_collection(collection_name)
        points_count = collection_info.points_count or 0
        
        if points_count == 0:
            print(f"⚠️  Collection '{collection_name}' is empty. Run embed_codebase.py first.")
            return False
        
        print(f"📊 Collection '{collection_name}' contains {points_count} points")
        
        # Test search
        model = SentenceTransformer('all-MiniLM-L6-v2')
        query = "function that handles HTTP requests"
        query_embedding = model.encode([query])[0]
        
        results = client.search(
            collection_name=collection_name,
            query_vector=query_embedding.tolist(),
            limit=3,
            with_payload=True
        )
        
        print(f"✅ Search completed! Found {len(results)} results for: '{query}'")
        
        for i, result in enumerate(results):
            print(f"\n{i+1}. Score: {result.score:.4f}")
            print(f"   File: {result.payload.get('file_path', 'Unknown')}")
            print(f"   Language: {result.payload.get('language', 'Unknown')}")
            print(f"   Content preview: {result.payload.get('content', '')[:100]}...")
        
        return True
        
    except Exception as e:
        print(f"❌ Search test failed: {e}")
        return False

def main():
    """Main test function."""
    print("🧪 RAG System Test Suite")
    print("=" * 50)
    
    # Load configuration from environment
    qdrant_url = os.getenv("QDRANT_URL", "http://localhost:6334")
    qdrant_api_key = os.getenv("QDRANT_API_KEY")
    collection_name = os.getenv("COLLECTION_NAME", "code_knowledge")
    
    print(f"🔧 Configuration:")
    print(f"   Qdrant URL: {qdrant_url}")
    print(f"   API Key: {'***' if qdrant_api_key else 'Not set'}")
    print(f"   Collection: {collection_name}")
    
    # Run tests
    tests_passed = 0
    total_tests = 3
    
    # Test 1: Qdrant connection
    if test_qdrant_connection(qdrant_url, qdrant_api_key):
        tests_passed += 1
    
    # Test 2: Sentence transformer
    if test_sentence_transformer():
        tests_passed += 1
    
    # Test 3: Basic search
    if test_basic_search(qdrant_url, qdrant_api_key, collection_name):
        tests_passed += 1
    
    # Summary
    print(f"\n📋 Test Results: {tests_passed}/{total_tests} passed")
    
    if tests_passed == total_tests:
        print("🎉 All tests passed! Your RAG system is ready.")
    else:
        print("⚠️  Some tests failed. Check the configuration and try again.")
        
    return tests_passed == total_tests

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)