use std::env;

#[tokio::main]
async fn main() {
    // Test environment variables
    println!("🔍 Testing API Key Configuration...");
    
    // Check Qdrant configuration
    match env::var("QDRANT_URL") {
        Ok(url) => {
            println!("✅ Qdrant URL configured: {}", url);
            
            match env::var("QDRANT_API_KEY") {
                Ok(key) => {
                    if key.len() > 20 {
                        println!("✅ Qdrant API key is properly configured");
                    } else {
                        println!("❌ Qdrant API key seems too short");
                    }
                }
                Err(_) => println!("❌ Qdrant API key not found"),
            }
        }
        Err(_) => println!("❌ Qdrant URL not found"),
    }
    
    // Test Qdrant connection
    println!("\n🌐 Testing Qdrant Connection...");
    match env::var("QDRANT_URL") {
        Ok(url) => {
            match env::var("QDRANT_API_KEY") {
                Ok(api_key) => {
                    match qdrant_client::Qdrant::from_url(&url).api_key(api_key).build() {
                        Ok(client) => {
                            println!("✅ Qdrant client created successfully");
                            
                            // Test listing collections
                            match client.list_collections().await {
                                Ok(collections) => {
                                    println!("✅ Successfully connected to Qdrant!");
                                    println!("📊 Found {} collections", collections.collections.len());
                                    for collection in collections.collections {
                                        println!("   - {}", collection.name);
                                    }
                                }
                                Err(e) => {
                                    println!("❌ Failed to list collections: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("❌ Failed to create Qdrant client: {}", e);
                        }
                    }
                }
                Err(_) => println!("❌ Qdrant API key not available"),
            }
        }
        Err(_) => println!("❌ Qdrant URL not available"),
    }
    
    println!("\n✅ Configuration test completed!");
}