#!/usr/bin/env python3
"""
SOL2INK Dataset Export Tool
Export embedded dataset from Qdrant for sharing with others
"""

import requests
import json
import os
from datetime import datetime
import argparse

def export_dataset(backend_url="http://localhost:8000", output_dir="exported_dataset"):
    """Export the complete SOL2INK dataset from the backend"""
    
    print("🚀 Starting SOL2INK Dataset Export...")
    
    # Create output directory
    os.makedirs(output_dir, exist_ok=True)
    
    # 1. Get dataset statistics
    print("📊 Getting dataset statistics...")
    stats_response = requests.get(f"{backend_url}/rag/stats")
    if stats_response.status_code == 200:
        stats = stats_response.json()
        print(f"   Found {stats['data']['regular_documents']} documents")
        print(f"   Found {stats['data']['cached_responses']} cached responses")
    else:
        print(f"   ❌ Failed to get stats: {stats_response.status_code}")
        return False
    
    # 2. Export all documents using search with broad query
    print("📁 Exporting all documents...")
    search_response = requests.post(
        f"{backend_url}/rag/search",
        json={
            "query": "contract",  # Broad query to get all documents
            "limit": 1000,  # Large limit to get everything
            "score_threshold": -1.0  # Very low threshold to include all
        }
    )
    
    if search_response.status_code == 200:
        search_data = search_response.json()
        documents = search_data.get('data', [])
        if documents is None:
            documents = []
        print(f"   ✅ Exported {len(documents)} documents")
        
        # Save documents as JSON
        documents_file = os.path.join(output_dir, "documents.json")
        with open(documents_file, 'w', encoding='utf-8') as f:
            json.dump(documents, f, indent=2, ensure_ascii=False)
        print(f"   💾 Saved to {documents_file}")
        
    else:
        print(f"   ❌ Failed to export documents: {search_response.status_code}")
        return False
    
    # 3. Export sample queries and responses
    print("🤖 Testing sample queries...")
    sample_queries = [
        "What is ink?",
        "How do I create a flipper contract?",
        "How do I migrate from Solidity to ink?",
        "Show me ERC20 token example",
        "What are the differences between Solidity and ink?"
    ]
    
    sample_responses = []
    for query in sample_queries:
        print(f"   🔍 Testing: {query}")
        response = requests.post(
            f"{backend_url}/ask",
            json={"query": query}
        )
        if response.status_code == 200:
            data = response.json()
            if data.get('success'):
                sample_responses.append({
                    "query": query,
                    "response": data.get('data', ''),
                    "timestamp": datetime.now().isoformat()
                })
                print(f"      ✅ Got response ({len(data.get('data', ''))} chars)")
            else:
                print(f"      ❌ Failed: {data.get('error', 'Unknown error')}")
        else:
            print(f"      ❌ HTTP Error: {response.status_code}")
    
    # Save sample responses
    samples_file = os.path.join(output_dir, "sample_responses.json")
    with open(samples_file, 'w', encoding='utf-8') as f:
        json.dump(sample_responses, f, indent=2, ensure_ascii=False)
    print(f"   💾 Saved {len(sample_responses)} sample responses to {samples_file}")
    
    # 4. Create metadata file
    print("📝 Creating metadata...")
    metadata = {
        "export_timestamp": datetime.now().isoformat(),
        "source_backend": backend_url,
        "dataset_name": "SOL2INK Migration Assistant Dataset",
        "description": "Solidity to ink! smart contract migration dataset with embedded examples",
        "total_documents": len(documents),
        "total_sample_responses": len(sample_responses),
        "collections": {
            "code_knowledge": "Smart contract code examples",
            "migration_guides": "Migration documentation",
            "defi_knowledge": "DeFi protocol information"
        },
        "document_types": list(set(doc.get('metadata', {}).get('category', 'unknown') for doc in documents)),
        "languages": list(set(doc.get('metadata', {}).get('language', 'unknown') for doc in documents)),
        "file_structure": {
            "documents.json": "All embedded documents with metadata and embeddings",
            "sample_responses.json": "Sample AI responses for testing",
            "metadata.json": "Dataset information and statistics",
            "readme.md": "Usage instructions and examples"
        }
    }
    
    metadata_file = os.path.join(output_dir, "metadata.json")
    with open(metadata_file, 'w', encoding='utf-8') as f:
        json.dump(metadata, f, indent=2, ensure_ascii=False)
    print(f"   💾 Saved metadata to {metadata_file}")
    
    # 5. Create README for the exported dataset
    print("📋 Creating README...")
    readme_content = f"""# SOL2INK Migration Assistant Dataset

**Exported:** {datetime.now().isoformat()}
**Source:** {backend_url}

## 📊 Dataset Overview

This dataset contains **{len(documents)} embedded documents** for Solidity to ink! smart contract migration assistance.

### Content Types:
- **Smart Contract Examples**: ink! contract code with explanations
- **Migration Guides**: Step-by-step migration documentation  
- **Code Comparisons**: Side-by-side Solidity vs ink! examples
- **Best Practices**: Security and optimization guidance

### Document Categories:
{', '.join(metadata['document_types'])}

### Languages:
{', '.join(metadata['languages'])}

## 📁 Files Structure

- `documents.json` - All embedded documents with metadata and semantic embeddings
- `sample_responses.json` - Sample AI responses for testing
- `metadata.json` - Dataset statistics and information
- `readme.md` - This file

## 🚀 How to Use This Dataset

### Option 1: With Your Own RAG System
```python
import json

# Load the documents
with open('documents.json', 'r') as f:
    documents = json.load(f)

# Each document has:
# - content: The actual text/code
# - score: Relevance score (if from search)
# - metadata: Category, language, file_path, etc.

for doc in documents:
    print(f"Category: {{doc['metadata']['category']}}")
    print(f"Content: {{doc['content'][:100]}}...")
    print("---")
```

### Option 2: With OpenAI/ChatGPT
```python
import openai

# Load documents and use as context
with open('documents.json', 'r') as f:
    documents = json.load(f)

# Create context from relevant documents
context = "\\n\\n".join([doc['content'] for doc in documents[:5]])

response = openai.ChatCompletion.create(
    model="gpt-4",
    messages=[
        {{"role": "system", "content": f"You are a Solidity to ink! migration assistant. Use this context: {{context}}"}},
        {{"role": "user", "content": "How do I create a flipper contract in ink!?"}}
    ]
)
```

### Option 3: With Qdrant Vector Database
```python
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct
import json

# Setup Qdrant client
client = QdrantClient("localhost", port=6334)

# Create collection
client.create_collection(
    collection_name="sol2ink_migration",
    vectors_config=VectorParams(size=1536, distance=Distance.COSINE),
)

# Load and insert documents
with open('documents.json', 'r') as f:
    documents = json.load(f)

# Insert documents (you'll need to generate embeddings)
points = []
for i, doc in enumerate(documents):
    # Note: You'll need to generate embeddings for doc['content']
    # using OpenAI embedding API or similar
    points.append(PointStruct(
        id=i,
        vector=doc.get('vector', [0.0] * 1536),  # Placeholder
        payload=doc['metadata']
    ))

client.upsert(collection_name="sol2ink_migration", points=points)
```

## 🔧 API Endpoints (Original System)

If you want to recreate the full system:

```bash
# Health check
GET /health

# Ask migration questions
POST /ask
{{"query": "How do I migrate ERC20 from Solidity to ink!?"}}

# Search documents
POST /rag/search
{{"query": "flipper contract", "limit": 5}}

# Get statistics
GET /rag/stats
```

## 📚 Example Usage

### Query Examples:
- "How do I create a flipper contract in ink!?"
- "What are the differences between Solidity and ink!?"
- "Show me ERC20 token migration example"
- "How do I handle events in ink!?"
- "What is the ink! storage system?"

### Expected Response Format:
```json
{{
  "success": true,
  "data": "Detailed migration guidance with code examples...",
  "error": null
}}
```

## 🤝 Contributing

This dataset is designed to help developers migrate from Solidity to ink!. Feel free to:
- Add more examples
- Improve documentation
- Create better embeddings
- Build new interfaces

## 📄 License

Please respect the original license and attribution when using this dataset.

---

**Generated by SOL2INK Dataset Export Tool**
**Dataset contains {len(documents)} documents across {len(set(doc.get('metadata', {}).get('category', 'unknown') for doc in documents))} categories**
"""
    
    readme_file = os.path.join(output_dir, "readme.md")
    with open(readme_file, 'w', encoding='utf-8') as f:
        f.write(readme_content)
    print(f"   💾 Saved README to {readme_file}")
    
    # 6. Create usage examples
    print("💡 Creating usage examples...")
    examples = {
        "python_openai": """
import openai
import json

# Load the dataset
with open('documents.json', 'r') as f:
    documents = json.load(f)

# Find relevant documents
def find_relevant_docs(query, documents, max_docs=3):
    # Simple keyword matching (you can improve this)
    relevant = []
    for doc in documents:
        if any(word.lower() in doc['content'].lower() for word in query.split()):
            relevant.append(doc)
    return relevant[:max_docs]

# Ask a question
query = "How do I create a flipper contract?"
relevant_docs = find_relevant_docs(query, documents)
context = "\\n\\n".join([doc['content'] for doc in relevant_docs])

response = openai.ChatCompletion.create(
    model="gpt-4",
    messages=[
        {"role": "system", "content": f"You are a Solidity to ink! migration assistant. Use this context: {context}"},
        {"role": "user", "content": query}
    ]
)

print(response.choices[0].message.content)
""",
        "javascript_fetch": """
// Load the dataset (in Node.js or browser)
const fs = require('fs');
const documents = JSON.parse(fs.readFileSync('documents.json', 'utf8'));

// Simple search function
function searchDocuments(query, limit = 5) {
    return documents
        .filter(doc => doc.content.toLowerCase().includes(query.toLowerCase()))
        .slice(0, limit);
}

// Example usage
const results = searchDocuments('flipper contract');
console.log(`Found ${results.length} relevant documents`);
results.forEach(doc => {
    console.log(`Category: ${doc.metadata.category}`);
    console.log(`Content: ${doc.content.substring(0, 100)}...`);
    console.log('---');
});
""",
        "curl_examples": """
# If you recreate the API, these are the endpoints:

# Health check
curl http://localhost:8000/health

# Ask a question
curl -X POST "http://localhost:8000/ask" \\
  -H "Content-Type: application/json" \\
  -d '{"query": "How do I create a flipper contract?"}'

# Search documents
curl -X POST "http://localhost:8000/rag/search" \\
  -H "Content-Type: application/json" \\
  -d '{"query": "flipper", "limit": 3}'

# Get statistics
curl http://localhost:8000/rag/stats
"""
    }
    
    examples_file = os.path.join(output_dir, "usage_examples.json")
    with open(examples_file, 'w', encoding='utf-8') as f:
        json.dump(examples, f, indent=2, ensure_ascii=False)
    print(f"   💾 Saved usage examples to {examples_file}")
    
    print(f"\n🎉 Dataset export completed successfully!")
    print(f"📁 Output directory: {output_dir}")
    print(f"📊 Total files created: {len(os.listdir(output_dir))}")
    print(f"💾 Dataset size: {len(documents)} documents")
    
    print(f"\n📋 Files created:")
    for file in os.listdir(output_dir):
        file_path = os.path.join(output_dir, file)
        size = os.path.getsize(file_path)
        print(f"   - {file} ({size:,} bytes)")
    
    print(f"\n🚀 You can now share the '{output_dir}' folder with others!")
    print("   They can use the documents with any AI system (OpenAI, Claude, etc.)")
    
    return True

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Export SOL2INK dataset')
    parser.add_argument('--backend-url', default='http://localhost:8000', help='Backend URL')
    parser.add_argument('--output-dir', default='exported_dataset', help='Output directory')
    
    args = parser.parse_args()
    
    success = export_dataset(args.backend_url, args.output_dir)
    exit(0 if success else 1)