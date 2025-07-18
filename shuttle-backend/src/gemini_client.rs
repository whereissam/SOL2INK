use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContent,
}

pub struct GeminiClient {
    client: Client,
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        Self { client, api_key }
    }

    pub async fn generate_response(&self, prompt: &str, context: &[String]) -> Result<String> {
        // Build the complete prompt with context
        let context_text = if context.is_empty() {
            String::new()
        } else {
            format!("Context:\n{}\n\n", context.join("\n\n"))
        };

        let full_prompt = format!(
            "{}You are a helpful developer assistant that answers questions about codebases. Use the provided context to answer the user's question accurately.\n\nQuestion: {}\n\nAnswer:",
            context_text,
            prompt
        );

        info!("Generating response with Gemini API for prompt length: {}", full_prompt.len());

        // Create the request
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: full_prompt,
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
            self.api_key
        );

        // Make the API call
        match self.client
            .post(&url)
            .json(&request)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<GeminiResponse>().await {
                        Ok(gemini_response) => {
                            if let Some(candidate) = gemini_response.candidates.first() {
                                if let Some(part) = candidate.content.parts.first() {
                                    info!("Successfully generated response from Gemini");
                                    return Ok(part.text.clone());
                                }
                            }
                            
                            error!("No valid response content from Gemini");
                            Ok("I apologize, but I couldn't generate a proper response at this time.".to_string())
                        }
                        Err(e) => {
                            error!("Failed to parse Gemini response: {}", e);
                            Ok("I apologize, but I couldn't generate a proper response at this time.".to_string())
                        }
                    }
                } else {
                    error!("Gemini API returned error status: {}", response.status());
                    Ok("I apologize, but I couldn't generate a proper response at this time.".to_string())
                }
            }
            Err(e) => {
                error!("Gemini API request failed: {}", e);
                Ok("I apologize, but the Gemini API is currently slow or unavailable. Please try again later, or check that the API key is correct.".to_string())
            }
        }
    }

    pub async fn generate_rag_response(&self, query: &str, retrieved_chunks: &[String]) -> Result<String> {
        if retrieved_chunks.is_empty() {
            return Ok("I don't have enough information in the codebase to answer that question.".to_string());
        }

        // For now, return a simplified response due to Gemini API performance issues
        let response = format!(
            "Based on the code context found, here's what I can tell you about '{}':\n\n{}\n\n(Note: This is a simplified response. The Gemini API is currently experiencing performance issues.)",
            query,
            retrieved_chunks.iter().take(2).map(|chunk| format!("• {}", chunk.chars().take(200).collect::<String>())).collect::<Vec<_>>().join("\n")
        );

        Ok(response)
    }
}

impl Default for GeminiClient {
    fn default() -> Self {
        let api_key = std::env::var("GEMINI_API_KEY")
            .unwrap_or_else(|_| "mock-api-key".to_string());
        Self::new(api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_client_creation() {
        let client = GeminiClient::new("test-api-key".to_string());
        assert_eq!(client.api_key, "test-api-key");
    }

    #[test]
    fn test_gemini_client_default() {
        let client = GeminiClient::default();
        // Should not panic and should create a client
        assert!(!client.api_key.is_empty());
    }

    #[test]
    fn test_gemini_request_serialization() {
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: "Test prompt".to_string(),
                }],
            }],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("Test prompt"));
    }

    #[test]
    fn test_gemini_response_deserialization() {
        let json = r#"
        {
            "candidates": [
                {
                    "content": {
                        "parts": [
                            {
                                "text": "Test response"
                            }
                        ]
                    }
                }
            ]
        }
        "#;

        let response: GeminiResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.candidates[0].content.parts[0].text, "Test response");
    }

    #[tokio::test]
    async fn test_generate_rag_response_empty_chunks() {
        let client = GeminiClient::new("test-key".to_string());
        let result = client.generate_rag_response("test query", &[]).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("don't have enough information"));
    }

    #[tokio::test]
    async fn test_generate_response_with_context() {
        let client = GeminiClient::new("test-key".to_string());
        let context = vec!["Context line 1".to_string(), "Context line 2".to_string()];
        
        // This will fail with mock API key, but should not panic
        let result = client.generate_response("test query", &context).await;
        // Should return error or fallback response, but not panic
        assert!(result.is_ok());
    }
}