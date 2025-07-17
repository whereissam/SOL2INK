# Migration Guide AI Embedding & Training Setup

This guide walks you through setting up AI embedding and training for your 10 Solidity to ink! migration guides.

## 🎯 Overview

You now have a complete pipeline to:
1. **Embed migration guides** into Qdrant vector database for RAG system
2. **Generate training data** for AI fine-tuning 
3. **Validate training quality** and completeness
4. **Test migration knowledge** with comprehensive queries

## 📁 Files Created

### Core Scripts
- `embed_migration_guides.py` - Specialized embedding for migration guides
- `create_training_data.py` - Generate training examples from guides
- `validate_training_data.py` - Quality validation of training data
- `test_migration_queries.py` - Test embedded knowledge with queries

### Existing Files (Your Setup)
- `embed_codebase.py` - General codebase embedding
- `requirements.txt` - Python dependencies

## 🚀 Quick Start

### 1. Install Dependencies

```bash
cd shuttle-backend
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

### 2. Start Qdrant Database

```bash
# Terminal 1: Start Qdrant (keep running)
docker run -p 6334:6334 qdrant/qdrant
```

### 3. Embed Migration Guides

```bash
# Embed all 10 migration guides into specialized collection
python3 embed_migration_guides.py ../docs/migrations --collection migration_guides

# Test the embeddings
python3 embed_migration_guides.py ../docs/migrations --test-query "How do I migrate ERC20 from Solidity to ink?"
```

### 4. Generate Training Data

```bash
# Create training examples from migration guides
python3 create_training_data.py ../docs/migrations --output-dir training_data

# This creates:
# - training_data/migration_training_data.jsonl (for fine-tuning)
# - training_data/migration_training_data.json (structured format)
# - training_data/training_data_stats.json (statistics)
```

### 5. Validate Training Data

```bash
# Validate quality of training data
python3 validate_training_data.py training_data/migration_training_data.json

# Generates:
# - validation_results.json (detailed results)
# - validation_report.txt (human-readable report)
```

### 6. Test Migration Knowledge

```bash
# Run comprehensive test of embedded knowledge
python3 test_migration_queries.py

# Test specific categories
python3 test_migration_queries.py --category "token_standards"

# Test single query
python3 test_migration_queries.py --query "How do I implement ERC20 in ink?"
```

## 📊 Expected Results

### Embedding Results
- **~200+ chunks** from 10 migration guides
- **Structured metadata** (difficulty, concepts, patterns)
- **Semantic search** capability through Qdrant
- **Guide coverage** for all contract types

### Training Data Results
- **~300+ training examples** across all guides
- **6 example types**: comparison, explanation, code_example, migration_step, pattern, best_practices
- **3 difficulty levels**: beginner, intermediate, advanced
- **JSONL format** ready for OpenAI fine-tuning

### Validation Results
- **Quality score** 0-100 (target: >75)
- **Coverage analysis** across all guides
- **Duplicate detection** and language validation
- **Issue reporting** with recommendations

### Test Results
- **8 query categories** testing different aspects
- **40+ test queries** covering common migration scenarios
- **Guide coverage** verification
- **Performance metrics** and success rates

## 🔧 Advanced Usage

### Custom Embedding Configuration

```bash
# Use different model or chunk size
python3 embed_migration_guides.py ../docs/migrations \
  --model "all-mpnet-base-v2" \
  --chunk-size 600 \
  --chunk-overlap 50

# Use cloud Qdrant
python3 embed_migration_guides.py ../docs/migrations \
  --qdrant-url "https://your-cluster.qdrant.io:6334" \
  --qdrant-api-key "your-api-key"
```

### Training Data Customization

```bash
# Generate training data with custom output
python3 create_training_data.py ../docs/migrations \
  --output-dir custom_training \
```

### Targeted Testing

```bash
# Test specific difficulty level
python3 test_migration_queries.py --category "advanced_patterns"

# Save detailed test results
python3 test_migration_queries.py --output detailed_test_results.json
```

## 📈 Integration with Your RAG System

### 1. Update Rust Backend

Add migration guide queries to your existing RAG endpoints in `main.rs`:

```rust
// Add to your existing RAG search
async fn search_migration_guides(
    query: String,
    collection: String = "migration_guides".to_string()
) -> Result<Vec<SearchResult>, Error> {
    // Use existing qdrant search logic but with migration_guides collection
}
```

### 2. Enhanced Query Processing

```bash
# Search both code_knowledge and migration_guides
curl -X POST "http://localhost:8000/rag/search" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "How do I migrate ERC20 token to ink?",
    "collections": ["code_knowledge", "migration_guides"],
    "limit": 5
  }'
```

### 3. Migration-Specific Endpoints

Add specialized endpoints for migration queries:

```rust
#[axum::debug_handler]
async fn migration_query(
    Json(request): Json<MigrationQueryRequest>
) -> impl IntoResponse {
    // Search migration_guides collection specifically
    // Return structured migration advice
}
```

## 🎓 Training Data Usage

### OpenAI Fine-tuning

```bash
# Upload training data to OpenAI
openai api fine-tuning.jobs.create \
  -t "training_data/migration_training_data.jsonl" \
  -m "gpt-3.5-turbo"
```

### Local Model Training

```python
# Use with Hugging Face transformers
from transformers import AutoTokenizer, AutoModelForCausalLM

# Load your training data
with open("training_data/migration_training_data.json") as f:
    training_data = json.load(f)

# Fine-tune local model
# (Implementation depends on your chosen model)
```

## 🔍 Query Examples

### Basic Migration Queries
```bash
"How do I migrate a Solidity contract to ink!?"
"What are the main differences between Solidity and ink!?"
"How do I convert Solidity mappings to ink! storage?"
```

### Token Standard Queries
```bash
"How do I implement ERC20 tokens in ink!?"
"What changes when migrating ERC721 to ink!?"
"How do I handle token transfers in ink!?"
```

### Advanced Pattern Queries
```bash
"How do I implement multi-signature wallets in ink!?"
"What are escrow patterns in ink! contracts?"
"How do I handle time-based operations in ink!?"
```

## 📊 Performance Optimization

### Embedding Performance
- **Batch size**: 32 for optimal embedding speed
- **Model choice**: `all-MiniLM-L6-v2` (fast) vs `all-mpnet-base-v2` (accurate)
- **Chunk size**: 800 tokens optimal for migration content

### Search Performance
- **Collection size**: ~200 chunks = <1s search time
- **Score threshold**: 0.7 for relevant results
- **Result limit**: 5-10 for optimal response time

### Training Performance
- **Example count**: 300+ examples sufficient for fine-tuning
- **Quality score**: >75 for production use
- **Validation**: Check before training deployment

## 🐛 Troubleshooting

### Common Issues

**1. "Collection not found"**
```bash
# Make sure Qdrant is running and embedding completed
docker ps | grep qdrant
python3 embed_migration_guides.py ../docs/migrations --collection migration_guides
```

**2. "No results found"**
```bash
# Check collection has data
curl -X GET "http://localhost:6334/collections/migration_guides"

# Test with broader query
python3 test_migration_queries.py --query "ink contract"
```

**3. "Low quality score"**
```bash
# Run validation to see specific issues
python3 validate_training_data.py training_data/migration_training_data.json

# Review validation report
cat validation_report.txt
```

**4. "Missing dependencies"**
```bash
# Reinstall requirements
pip install -r requirements.txt

# Check specific package
pip show sentence-transformers qdrant-client
```

## 🎯 Success Metrics

### Embedding Success
- ✅ All 10 guides embedded successfully
- ✅ 200+ chunks with proper metadata
- ✅ Search returns relevant results (score >0.7)
- ✅ All guides accessible via queries

### Training Data Success
- ✅ 300+ training examples generated
- ✅ Quality score >75
- ✅ All example types represented
- ✅ Balanced difficulty distribution

### System Integration Success
- ✅ RAG system returns migration guidance
- ✅ Sub-second response times
- ✅ Accurate technical information
- ✅ Comprehensive coverage of migration topics

## 🔄 Maintenance

### Regular Tasks
1. **Update embeddings** when guides are modified
2. **Regenerate training data** for new fine-tuning
3. **Run validation** before deploying training data
4. **Test queries** to ensure knowledge quality

### Monitoring
- Monitor search performance and relevance
- Track user queries for new training examples
- Update embeddings when guides are modified
- Validate training data quality regularly

---

## 🎉 Conclusion

You now have a complete AI pipeline for your migration guides:

1. **📚 10 comprehensive migration guides** covering all major patterns
2. **🔍 Vector database** with searchable embeddings
3. **🧠 Training data** for AI fine-tuning
4. **✅ Quality validation** and testing frameworks
5. **🚀 Ready for integration** with your existing RAG system

Your migration guides are now ready to power intelligent assistance for developers migrating from Solidity to ink!!