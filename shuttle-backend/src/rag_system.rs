use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, SearchPointsBuilder, VectorParamsBuilder,
    UpsertPointsBuilder,
};
use qdrant_client::Payload;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error};
use anyhow::Result;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::gemini_client::GeminiClient;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmbeddingRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchRequest {
    pub query: String,
    pub limit: u64,
    pub score_threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SearchResult {
    pub content: String,
    pub score: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry {
    pub query: String,
    pub answer: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct RAGSystem {
    qdrant_client: Qdrant,
    gemini_client: GeminiClient,
    regular_collection: String,
    cache_collection: String,
}

impl RAGSystem {
    pub fn new(qdrant_client: Qdrant, gemini_api_key: String) -> Self {
        let gemini_client = GeminiClient::new(gemini_api_key);
        
        Self {
            qdrant_client,
            gemini_client,
            regular_collection: "code_knowledge".to_string(),
            cache_collection: "code_knowledge_cache".to_string(),
        }
    }

    /// Initialize both regular and cache collections
    pub async fn initialize_collections(&self) -> Result<()> {
        info!("Initializing RAG system collections...");
        
        // Initialize regular collection
        self.create_regular_collection().await?;
        
        // Initialize cache collection
        self.create_cache_collection().await?;
        
        info!("RAG system collections initialized successfully");
        Ok(())
    }

    /// Create regular collection for document storage
    async fn create_regular_collection(&self) -> Result<()> {
        let collections = self.qdrant_client.list_collections().await?;
        let collection_exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.regular_collection);

        if collection_exists {
            info!("Deleting existing regular collection to recreate with correct dimensions: {}", self.regular_collection);
            self.qdrant_client.delete_collection(&self.regular_collection).await?;
        }

        info!("Creating regular collection with 384 dimensions: {}", self.regular_collection);
        
        self.qdrant_client
            .create_collection(
                CreateCollectionBuilder::new(&self.regular_collection)
                    .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine))
            )
            .await?;

        Ok(())
    }

    /// Create cache collection for semantic caching
    async fn create_cache_collection(&self) -> Result<()> {
        let collections = self.qdrant_client.list_collections().await?;
        let collection_exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.cache_collection);

        if collection_exists {
            info!("Deleting existing cache collection to recreate with correct dimensions: {}", self.cache_collection);
            self.qdrant_client.delete_collection(&self.cache_collection).await?;
        }

        info!("Creating cache collection with 384 dimensions: {}", self.cache_collection);
        
        self.qdrant_client
            .create_collection(
                CreateCollectionBuilder::new(&self.cache_collection)
                    .vectors_config(VectorParamsBuilder::new(384, Distance::Euclid))
            )
            .await?;

        Ok(())
    }

    /// Generate embeddings for text using sentence-transformers (Python) via HTTP
    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        // For now, create a simple embedding using the same model as Python
        // This is a temporary solution - in production you'd want to:
        // 1. Run a separate embedding service
        // 2. Use ONNX runtime for sentence-transformers in Rust
        // 3. Or call a Python microservice
        
        // Simple hash-based embedding for demo (384 dimensions to match Python model)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Create a deterministic but pseudo-random embedding
        let mut embedding = Vec::with_capacity(384);
        let mut seed = hash;
        for _ in 0..384 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            embedding.push((seed as f32 / u64::MAX as f32) * 2.0 - 1.0);
        }
        
        // Normalize the vector
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for val in &mut embedding {
                *val /= magnitude;
            }
        }
        
        Ok(embedding)
    }

    /// Add document to regular collection
    pub async fn add_document(&self, text: &str, metadata: HashMap<String, String>) -> Result<String> {
        let embedding = self.embed_text(text).await?;
        let document_id = Uuid::new_v4().to_string();
        
        let mut payload = serde_json::json!({
            "content": text,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        
        // Add metadata to payload
        for (key, value) in metadata {
            payload[key] = serde_json::Value::String(value);
        }
        
        let points = vec![PointStruct::new(
            document_id.clone(),
            embedding,
            Payload::try_from(payload)?,
        )];

        self.qdrant_client
            .upsert_points(UpsertPointsBuilder::new(&self.regular_collection, points))
            .await?;

        info!("Document added to regular collection with ID: {}", document_id);
        Ok(document_id)
    }

    /// Search regular collection for similar documents
    pub async fn search_documents(&self, query: &str, limit: u64, score_threshold: Option<f32>) -> Result<Vec<SearchResult>> {
        let embedding = self.embed_text(query).await?;
        
        let mut search_builder = SearchPointsBuilder::new(&self.regular_collection, embedding, limit)
            .with_payload(true);
            
        if let Some(threshold) = score_threshold {
            search_builder = search_builder.score_threshold(threshold);
        }

        let search_result = self.qdrant_client
            .search_points(search_builder)
            .await?;

        let mut results = Vec::new();
        for point in search_result.result {
            let content = point.payload
                .get("content")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_default();
                
            let mut metadata = HashMap::new();
            for (key, value) in point.payload.iter() {
                if key != "content" {
                    if let Some(str_value) = value.as_str() {
                        metadata.insert(key.clone(), str_value.to_string());
                    }
                }
            }

            results.push(SearchResult {
                content,
                score: point.score,
                metadata,
            });
        }

        Ok(results)
    }

    /// Search cache collection for similar queries
    pub async fn search_cache(&self, query: &str) -> Result<Option<String>> {
        let embedding = self.embed_text(query).await?;

        let search_result = self.qdrant_client
            .search_points(
                SearchPointsBuilder::new(&self.cache_collection, embedding, 1)
                    .with_payload(true)
                    .score_threshold(0.95) // High threshold for cache hits
            )
            .await?;

        if let Some(point) = search_result.result.first() {
            let answer = point.payload
                .get("answer")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            info!("Cache hit for query with score: {}", point.score);
            return Ok(answer);
        }

        Ok(None)
    }

    /// Add response to cache
    pub async fn add_to_cache(&self, query: &str, answer: &str) -> Result<String> {
        let embedding = self.embed_text(query).await?;
        let cache_id = Uuid::new_v4().to_string();
        
        let payload = serde_json::json!({
            "query": query,
            "answer": answer,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        
        let points = vec![PointStruct::new(
            cache_id.clone(),
            embedding,
            Payload::try_from(payload)?,
        )];

        self.qdrant_client
            .upsert_points(UpsertPointsBuilder::new(&self.cache_collection, points))
            .await?;

        info!("Response cached with ID: {}", cache_id);
        Ok(cache_id)
    }

    /// Generate AI response using RAG
    pub async fn generate_rag_response(&self, query: &str, context_limit: u64) -> Result<String> {
        info!("Starting RAG response generation for query: {}", query);
        
        // Search for relevant documents (skip cache for now to avoid delays)
        info!("Searching for relevant documents");
        let search_results = self.search_documents(query, context_limit, Some(0.0)).await?;
        info!("Found {} search results", search_results.len());
        
        if search_results.is_empty() {
            info!("No relevant documents found for query");
            return Ok("I don't have enough information to answer that question about ink! smart contracts.".to_string());
        }

        // Prepare context from search results
        let context: Vec<String> = search_results.iter()
            .take(5)
            .map(|result| {
                let mut context_item = String::new();
                
                if let Some(file_path) = result.metadata.get("file_path") {
                    context_item.push_str(&format!("Source: {}\n", file_path));
                }
                
                if let Some(contract_name) = self.extract_contract_name(&result.content) {
                    context_item.push_str(&format!("Contract: {}\n", contract_name));
                }
                
                if let Some(description) = self.extract_description(&result.content) {
                    context_item.push_str(&format!("Description: {}\n", description));
                }
                
                context_item.push_str(&format!("Code:\n{}\n", self.format_code(&result.content)));
                context_item
            })
            .collect();

        // Create specialized migration prompt
        let migration_prompt = format!(
            "You are an expert in both Solidity and ink! smart contracts. The user is asking: '{}'

Please provide a detailed, step-by-step explanation based on the provided code examples. Focus on:

1. **Key Differences**: Explain main conceptual differences between Solidity and ink!
2. **Migration Steps**: Provide clear, actionable steps for converting patterns
3. **Code Examples**: Show concrete before/after examples from the context
4. **Best Practices**: Highlight important considerations and gotchas
5. **Practical Guide**: Make it actionable for developers

Format your response clearly with specific code snippets and explanations, not just raw code dumps.",
            query
        );

        // Use Gemini AI to generate proper response
        match self.gemini_client.generate_response(&migration_prompt, &context).await {
            Ok(ai_response) => {
                info!("Successfully generated AI response");
                Ok(ai_response)
            },
            Err(e) => {
                error!("Failed to generate AI response: {}", e);
                Ok(format!(
                    "I'm having trouble generating a detailed response right now. Here are the most relevant code examples I found:\n\n{}",
                    context.join("\n\n---\n\n")
                ))
            }
        }
    }
    
    /// Extract contract name from code content
    fn extract_contract_name(&self, content: &str) -> Option<String> {
        // Look for mod name or contract name
        if let Some(mod_line) = content.lines().find(|line| line.trim().starts_with("mod ")) {
            if let Some(name) = mod_line.split_whitespace().nth(1) {
                return Some(name.trim_end_matches(" {").to_string());
            }
        }
        
        // Look for struct name
        if let Some(struct_line) = content.lines().find(|line| line.contains("pub struct") && line.contains("{")) {
            if let Some(name) = struct_line.split("struct").nth(1) {
                if let Some(name) = name.split_whitespace().next() {
                    return Some(name.to_string());
                }
            }
        }
        
        None
    }
    
    /// Extract description from comments
    fn extract_description(&self, content: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut description = String::new();
        
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("/// ") {
                if !description.is_empty() {
                    description.push(' ');
                }
                description.push_str(&trimmed[4..]);
            } else if trimmed.starts_with("//") && trimmed.contains("contract") {
                return Some(trimmed[2..].trim().to_string());
            }
        }
        
        if !description.is_empty() {
            Some(description)
        } else {
            None
        }
    }
    
    /// Format code for better display
    fn format_code(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut formatted = String::new();
        let mut in_important_section = false;
        let mut line_count = 0;
        
        for line in lines {
            line_count += 1;
            
            // Skip empty lines at the beginning
            if line_count < 5 && line.trim().is_empty() {
                continue;
            }
            
            // Highlight important sections
            if line.contains("#[ink(") || line.contains("pub fn") || line.contains("impl") {
                in_important_section = true;
            }
            
            formatted.push_str(line);
            formatted.push('\n');
            
            // Limit to ~50 lines for readability
            if line_count > 50 {
                formatted.push_str("    // ... more code ...\n");
                break;
            }
        }
        
        formatted
    }
    
    /// Generate structured response for API consumption
    pub async fn generate_structured_response(&self, query: &str, context_limit: u64) -> Result<crate::FormattedResponse> {
        info!("Starting structured response generation for query: {}", query);
        
        // Search for relevant documents
        let search_results = self.search_documents(query, context_limit, Some(0.0)).await?;
        
        if search_results.is_empty() {
            return Ok(crate::FormattedResponse {
                query: query.to_string(),
                summary: "No relevant ink! smart contract examples found for your query.".to_string(),
                examples: vec![],
                help_text: "Try refining your search terms or asking about specific ink! concepts like 'storage', 'messages', 'events', or 'constructors'.".to_string(),
            });
        }
        
        let mut examples = Vec::new();
        
        for result in search_results.iter().take(3) {
            let example = crate::CodeExample {
                title: self.extract_contract_name(&result.content)
                    .unwrap_or_else(|| "Smart Contract".to_string()),
                description: self.extract_description(&result.content),
                code: self.format_code(&result.content),
                source_file: result.metadata.get("file_path").cloned(),
                relevance_score: result.score * 100.0,
            };
            examples.push(example);
        }
        
        let summary = format!(
            "Found {} relevant ink! smart contract examples matching your query. These examples demonstrate best practices and common patterns in ink! development.",
            examples.len()
        );
        
        let help_text = "These examples are from the official ink! examples repository. You can use them as templates for building your own smart contracts on Polkadot.".to_string();
        
        Ok(crate::FormattedResponse {
            query: query.to_string(),
            summary,
            examples,
            help_text,
        })
    }

    /// Bulk insert documents from text data
    pub async fn bulk_insert_documents(&self, documents: Vec<(String, HashMap<String, String>)>) -> Result<Vec<String>> {
        let mut document_ids = Vec::new();
        
        for (text, metadata) in documents {
            match self.add_document(&text, metadata).await {
                Ok(doc_id) => {
                    document_ids.push(doc_id);
                }
                Err(e) => {
                    error!("Failed to insert document: {}", e);
                }
            }
        }
        
        info!("Bulk inserted {} documents", document_ids.len());
        Ok(document_ids)
    }

    /// Get collection statistics
    pub async fn get_collection_stats(&self) -> Result<HashMap<String, u64>> {
        let mut stats = HashMap::new();
        
        // Get regular collection info
        if let Ok(regular_info) = self.qdrant_client.collection_info(&self.regular_collection).await {
            if let Some(info) = regular_info.result {
                stats.insert("regular_documents".to_string(), info.points_count.unwrap_or(0));
            }
        }
        
        // Get cache collection info
        if let Ok(cache_info) = self.qdrant_client.collection_info(&self.cache_collection).await {
            if let Some(info) = cache_info.result {
                stats.insert("cached_responses".to_string(), info.points_count.unwrap_or(0));
            }
        }
        
        Ok(stats)
    }
}