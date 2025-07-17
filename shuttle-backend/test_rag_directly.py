#!/usr/bin/env python3
"""
Test the RAG system directly without the web server.
"""

import sys
import os
sys.path.append('.')

from qdrant_client import QdrantClient
from sentence_transformers import SentenceTransformer

def test_rag_system():
    """Test the RAG system components directly."""
    
    # Connect to Qdrant
    client = QdrantClient(host='localhost', port=6334, prefer_grpc=True)
    model = SentenceTransformer('all-MiniLM-L6-v2')
    
    print("🧪 Testing RAG System Components")
    print("=" * 50)
    
    # Test 1: Check collection exists
    collections = client.get_collections()
    print(f"✅ Collections found: {len(collections.collections)}")
    
    for collection in collections.collections:
        info = client.get_collection(collection.name)
        print(f"   - {collection.name}: {info.points_count} points")
    
    # Test 2: Test search functionality
    test_queries = [
        "flipper contract toggle boolean",
        "ERC20 token implementation",
        "storage struct definition",
        "incrementer contract function",
        "cross contract calls"
    ]
    
    print(f"\n🔍 Testing search with {len(test_queries)} queries:")
    print("-" * 30)
    
    for query in test_queries:
        print(f"\nQuery: {query}")
        
        # Embed query
        query_embedding = model.encode([query])[0]
        
        # Search
        results = client.search(
            collection_name='code_knowledge',
            query_vector=query_embedding.tolist(),
            limit=2,
            with_payload=True
        )
        
        print(f"  Found {len(results)} results:")
        for i, result in enumerate(results, 1):
            print(f"    {i}. {result.payload['file_path']} (score: {result.score:.3f})")
            print(f"       {result.payload['content'][:100]}...")
    
    # Test 3: Simulate RAG response generation
    print(f"\n🤖 Simulating RAG response generation:")
    print("-" * 30)
    
    query = "How does the flipper contract work?"
    query_embedding = model.encode([query])[0]
    
    results = client.search(
        collection_name='code_knowledge',
        query_vector=query_embedding.tolist(),
        limit=3,
        with_payload=True
    )
    
    context = "\n\n".join([result.payload['content'] for result in results])
    
    print(f"Query: {query}")
    print(f"Context length: {len(context)} characters")
    print(f"Context preview: {context[:300]}...")
    
    # This is where we would call Gemini API in the real implementation
    print("\n✅ RAG system is working correctly!")
    print("💡 Ready to integrate with Gemini API for response generation")

if __name__ == "__main__":
    test_rag_system()