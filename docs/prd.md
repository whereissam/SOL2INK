📄 PRD: RAG-Based Developer Assistant (Rust + Qdrant + Gemini)
1. Overview
This system provides a developer assistant capable of answering questions about a codebase. It uses:

Qdrant as a vector database for semantic search

Rust for the backend service that performs retrieval and LLM interaction

Gemini API for generating natural language responses from retrieved context

2. Goals
🎯 Primary Goals
Allow developers to ask natural language questions about their codebase

Return accurate answers using Retrieval-Augmented Generation (RAG)

Use Rust as the main application runtime

Support Gemini as the LLM backend

Use Qdrant to store and retrieve embedded context

🛠️ Non-Goals
Embedding of code within Rust (handled externally via Python or embedding service)

Full IDE plugin integration (could be future work)

3. System Architecture
text
Copy
Edit
[ Source Code ]
     |
     | (embedding script - Python)
     ↓
[ Vector DB: Qdrant ]
     ↑                       ↓
[ Rust Backend API ] ---> [ Gemini API ]
         ↑
      [ Developer ]
4. Features
✅ Core Features
Feature	Description
Upload repo context	Use an external script to embed and store code/documentation in Qdrant
Semantic Search	Rust backend queries Qdrant with embedded user question
Gemini Integration	Retrieved chunks are sent with the question to Gemini for final response
REST API	Provide endpoints like /ask?query=...
Configurable API Key	Gemini API key via .env or config file

⚙️ Planned Endpoints
Method	Endpoint	Description
GET	/ask?query=	Ask a question and get a response
POST	/query_vector	Optional endpoint to simulate vector lookup

5. Tech Stack
Component	Technology
Backend	Rust
Web Framework	actix-web or axum
Vector Search	Qdrant
LLM	Gemini API
Embedding Tool	Python script using sentence-transformers
Deployment	Shuttle or Docker

6. Functional Requirements
 Load pre-computed embeddings into Qdrant (external step)

 Accept query via REST endpoint

 Embed the query (via Python script or expose embedding vector input)

 Search Qdrant and retrieve top-k most relevant chunks

 Assemble prompt and call Gemini API

 Return the generated answer to the user

7. Non-Functional Requirements
⚡ Fast response time (<2s round trip)

🔐 Secure handling of Gemini API key

🧪 Unit and integration tests for endpoints and Qdrant lookup

🌐 Stateless (Shuttle-deployable)

8. Milestones
Milestone	Deliverable
Day 1–2	Python embedding script to load code into Qdrant
Day 3–5	Rust backend with Qdrant client and Gemini call
Day 6–7	REST API integration, prompt design, error handling
Day 8	Deployment to Shuttle (with .env)
Day 9–10	Testing, logging, optional Web UI

9. Out of Scope / Future Work
IDE integrations (e.g. VSCode)

Authenticated user sessions

Real-time embedding (for file changes)

Gemini Pro Vision support (code + diagram)