# 🔌 API Usage Examples

This document provides comprehensive examples of how to use the RAG-based developer assistant API endpoints.

## 🎯 Primary /ask Endpoint

### GET Request Examples

```bash
# Basic question about ink smart contracts
curl -G "http://localhost:8000/ask" \
  --data-urlencode "query=How does the flipper contract work?"

# Question about specific implementation
curl -G "http://localhost:8000/ask" \
  --data-urlencode "query=Show me how to implement storage in ink smart contracts"

# Question about patterns and best practices  
curl -G "http://localhost:8000/ask" \
  --data-urlencode "query=What are the common patterns in ink contract development?"
```

### POST Request Examples

```bash
# Basic POST request
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "How do I create an ERC20 token in ink?"
  }'

# Complex query about cross-contract calls
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "Explain how cross-contract calls work in ink and show me an example"
  }'

# Question about testing
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "How do I write end-to-end tests for ink smart contracts?"
  }'
```

### Expected Response Format

```json
{
  "success": true,
  "data": "The flipper contract is a simple smart contract that demonstrates the basic structure of an ink! contract. It stores a boolean value in its storage and provides two main functions:\n\n1. **Constructor (`new`)**: Initializes the contract with a starting boolean value\n2. **flip()**: Toggles the boolean value between true and false\n3. **get()**: Returns the current boolean value\n\nHere's the basic structure:\n\n```rust\n#[ink::contract]\npub mod flipper {\n    #[ink(storage)]\n    pub struct Flipper {\n        value: bool,\n    }\n    \n    impl Flipper {\n        #[ink(constructor)]\n        pub fn new(init_value: bool) -> Self {\n            Self { value: init_value }\n        }\n        \n        #[ink(message)]\n        pub fn flip(&mut self) {\n            self.value = !self.value;\n        }\n        \n        #[ink(message)]\n        pub fn get(&self) -> bool {\n            self.value\n        }\n    }\n}\n```\n\nThis contract is often used as a \"Hello World\" example for ink! development.",
  "error": null
}
```

## 🔍 Semantic Search Examples

### Search for Code Patterns

```bash
# Find storage-related code
curl -X POST "http://localhost:8000/rag/search" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "storage struct definition",
    "limit": 5,
    "score_threshold": 0.7
  }'

# Find ERC token implementations
curl -X POST "http://localhost:8000/rag/search" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "ERC20 token implementation transfer function",
    "limit": 3,
    "score_threshold": 0.6
  }'

# Find contract interaction patterns
curl -X POST "http://localhost:8000/rag/search" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "cross contract calls ink",
    "limit": 4,
    "score_threshold": 0.8
  }'
```

### Search Response Format

```json
{
  "success": true,
  "data": [
    {
      "content": "#[ink(storage)]\npub struct Flipper {\n    value: bool,\n}\n\nimpl Flipper {\n    #[ink(constructor)]\n    pub fn new(init_value: bool) -> Self {\n        Self { value: init_value }\n    }",
      "score": 0.8234,
      "metadata": {
        "file_path": "flipper/lib.rs",
        "language": "rust",
        "file_type": ".rs",
        "start_line": "1",
        "end_line": "25"
      }
    }
  ],
  "error": null
}
```

## 🧠 RAG Query Examples

### Get AI-Generated Responses with Context

```bash
# Detailed explanation with context
curl -X POST "http://localhost:8000/rag/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is the difference between #[ink(message)] and #[ink(constructor)]?",
    "limit": 3
  }'

# Best practices query
curl -X POST "http://localhost:8000/rag/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What are the security considerations when writing ink smart contracts?",
    "limit": 5
  }'

# Implementation guidance
curl -X POST "http://localhost:8000/rag/query" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "How do I handle errors and events in ink smart contracts?",
    "limit": 4
  }'
```

## 📚 Adding Documents to Knowledge Base

### Add New Code Documentation

```bash
# Add a new code example
curl -X POST "http://localhost:8000/rag/document" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "// Custom NFT Implementation\n// This contract demonstrates how to create a basic NFT (Non-Fungible Token)\n// using the ink! framework with proper ownership and transfer mechanisms.\n\n#[ink::contract]\nmod custom_nft {\n    use ink::storage::Mapping;\n    \n    #[ink(storage)]\n    pub struct CustomNft {\n        owner: Mapping<u32, AccountId>,\n        token_approvals: Mapping<u32, AccountId>,\n        next_token_id: u32,\n    }\n    \n    #[ink(event)]\n    pub struct Transfer {\n        #[ink(topic)]\n        from: Option<AccountId>,\n        #[ink(topic)]\n        to: Option<AccountId>,\n        #[ink(topic)]\n        token_id: u32,\n    }\n}"
  }'

# Add best practices documentation
curl -X POST "http://localhost:8000/rag/document" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "# ink! Smart Contract Best Practices\n\n## 1. Storage Optimization\n- Use `Mapping` for key-value storage instead of `Vec` when possible\n- Pack struct fields efficiently to minimize storage costs\n- Consider lazy storage patterns for large datasets\n\n## 2. Error Handling\n- Use custom error types with `#[ink::scale_derive(Encode, Decode)]`\n- Return `Result<T, Error>` from fallible functions\n- Provide meaningful error messages for debugging\n\n## 3. Events and Logging\n- Emit events for important state changes\n- Use `#[ink(topic)]` for indexed event parameters\n- Keep event data minimal to reduce costs"
  }'
```

## 📊 System Statistics

### Get Knowledge Base Statistics

```bash
# Check system status
curl -X GET "http://localhost:8000/rag/stats"

# Expected response
{
  "success": true,
  "data": {
    "regular_documents": 484,
    "cached_responses": 0
  },
  "error": null
}
```

## 🏥 Health Check

### Verify System Status

```bash
# Basic health check
curl -X GET "http://localhost:8000/health"

# Expected response
{
  "success": true,
  "data": "DynaVest Shuttle Backend is running!",
  "error": null
}
```

## 🐍 Python Client Examples

### Using the API from Python

```python
import requests
import json

class RAGAssistant:
    def __init__(self, base_url="http://localhost:8000"):
        self.base_url = base_url
    
    def ask(self, question):
        """Ask a question and get an AI-generated response."""
        response = requests.post(
            f"{self.base_url}/ask",
            json={"query": question},
            headers={"Content-Type": "application/json"}
        )
        return response.json()
    
    def search(self, query, limit=5, threshold=0.7):
        """Search for relevant code chunks."""
        response = requests.post(
            f"{self.base_url}/rag/search",
            json={
                "query": query,
                "limit": limit,
                "score_threshold": threshold
            },
            headers={"Content-Type": "application/json"}
        )
        return response.json()
    
    def add_document(self, text):
        """Add a new document to the knowledge base."""
        response = requests.post(
            f"{self.base_url}/rag/document",
            json={"text": text},
            headers={"Content-Type": "application/json"}
        )
        return response.json()

# Usage examples
assistant = RAGAssistant()

# Ask a question
result = assistant.ask("How do I implement an incrementer contract in ink?")
print(result["data"])

# Search for specific code
search_results = assistant.search("storage mapping AccountId")
for result in search_results["data"]:
    print(f"File: {result['metadata']['file_path']}")
    print(f"Score: {result['score']}")
    print(f"Content: {result['content'][:100]}...")
    print()

# Add new documentation
assistant.add_document("""
// Advanced ink! Pattern: Proxy Contract
// This pattern allows for contract upgrades while maintaining state
#[ink::contract]
mod proxy {
    use ink::env::call::{build_call, ExecutionInput, Selector};
    
    #[ink(storage)]
    pub struct Proxy {
        admin: AccountId,
        implementation: AccountId,
    }
}
""")
```

## 🌐 JavaScript/TypeScript Examples

### Frontend Integration

```typescript
interface RAGResponse {
  success: boolean;
  data?: string;
  error?: string;
}

interface SearchResult {
  content: string;
  score: number;
  metadata: {
    file_path: string;
    language: string;
    file_type: string;
    start_line: string;
    end_line: string;
  };
}

class RAGClient {
  constructor(private baseUrl: string = 'http://localhost:8000') {}

  async ask(question: string): Promise<RAGResponse> {
    const response = await fetch(`${this.baseUrl}/ask`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ query: question }),
    });
    
    return response.json();
  }

  async search(query: string, limit: number = 5): Promise<{ success: boolean; data: SearchResult[] }> {
    const response = await fetch(`${this.baseUrl}/rag/search`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ 
        query, 
        limit,
        score_threshold: 0.7 
      }),
    });
    
    return response.json();
  }
}

// Usage in a React component
const useRAGAssistant = () => {
  const [client] = useState(() => new RAGClient());
  
  const askQuestion = async (question: string) => {
    try {
      const result = await client.ask(question);
      if (result.success) {
        return result.data;
      } else {
        throw new Error(result.error || 'Failed to get response');
      }
    } catch (error) {
      console.error('Error asking question:', error);
      throw error;
    }
  };
  
  return { askQuestion };
};
```

## 🔧 Advanced Usage Patterns

### Batch Processing

```bash
# Process multiple questions in sequence
questions=(
  "How does the flipper contract work?"
  "What is an ERC20 token implementation?"
  "How do I write tests for ink contracts?"
  "What are cross-contract calls?"
)

for question in "${questions[@]}"; do
  echo "Question: $question"
  curl -s -X POST "http://localhost:8000/ask" \
    -H "Content-Type: application/json" \
    -d "{\"query\": \"$question\"}" | \
    jq -r '.data // .error'
  echo "---"
done
```

### Error Handling Examples

```bash
# Test with empty query
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": ""}'

# Expected error response
{
  "success": false,
  "data": null,
  "error": "Query cannot be empty"
}

# Test with malformed JSON
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "test"'

# Expected HTTP 400 Bad Request
```

### Performance Testing

```bash
# Measure response time
time curl -s -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "How does ink storage work?"}' > /dev/null

# Load testing with multiple concurrent requests
for i in {1..10}; do
  curl -s -X POST "http://localhost:8000/ask" \
    -H "Content-Type: application/json" \
    -d '{"query": "What is the flipper contract?"}' &
done
wait
```

## 📝 Common Query Patterns

### Code Understanding Queries
- "How does the [contract_name] contract work?"
- "What does the [function_name] function do?"
- "Explain the storage structure in [contract_name]"
- "Show me examples of [pattern] in ink contracts"

### Implementation Guidance Queries  
- "How do I implement [feature] in ink?"
- "What is the best way to [task] in ink smart contracts?"
- "Show me how to write [type] tests for ink contracts"
- "What are the security considerations for [feature]?"

### Troubleshooting Queries
- "Why am I getting [error] in my ink contract?"
- "How do I fix [issue] in ink development?"
- "What causes [problem] in ink smart contracts?"
- "How do I debug [issue] in ink?"

### Best Practices Queries
- "What are the best practices for [topic] in ink?"
- "How should I structure [type] in ink contracts?"
- "What are common mistakes in [area] development?"
- "How do I optimize [aspect] in ink contracts?"