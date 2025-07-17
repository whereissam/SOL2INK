# ✅ RAG Developer Assistant Checklist (Rust + Qdrant + Gemini)

## 📁 Project Setup

- [ ] Create project repo (e.g., `rag-dev-assistant`)
- [ ] Set up Rust project (`cargo init`)
- [ ] Add dependencies:
  - [ ] `qdrant-client`
  - [ ] `reqwest`
  - [ ] `serde`, `serde_json`
  - [ ] `tokio`
  - [ ] Optional: `actix-web` or `axum` (REST API)
- [ ] Create `.env` for API key & config

---

## 🧠 Embedding & Vector Store Setup (Python)

> External step (can be reused across many apps)

- [ ] Write a Python script to:
  - [ ] Recursively read code/doc files from repo
  - [ ] Split files into chunks (`RecursiveCharacterTextSplitter`)
  - [ ] Embed chunks using `sentence-transformers` (`all-MiniLM-L6-v2` or similar)
  - [ ] Store vectors in Qdrant with text payload
- [ ] Test Qdrant setup with Python client
- [ ] Verify vector search returns relevant code chunks

---

## 🔧 Rust Backend: Qdrant Integration

- [ ] Initialize Qdrant client in Rust
- [ ] Connect to local or cloud Qdrant instance
- [ ] Implement function to:
  - [ ] Accept a search vector (or query string)
  - [ ] Query Qdrant (`SearchPoints`)
  - [ ] Return top-k matching documents
- [ ] Parse Qdrant payloads to get text chunks

---

## 🤖 Gemini API Integration (Rust)

- [ ] Create Gemini request function using `reqwest`
- [ ] Format prompt as:

  """
  You are a helpful assistant.

  Context:
  <retrieved docs>

  Question:
  <user query>

  Answer:
  """

- [ ] Parse and return Gemini’s response
- [ ] Handle API errors and edge cases

---

## 🌐 REST API (Optional but Recommended)

- [ ] Add `actix-web` or `axum`
- [ ] Create endpoint: `GET /ask?query=...`
- [ ] In handler:
  - [ ] Embed query (or simulate with placeholder)
  - [ ] Run Qdrant search
  - [ ] Call Gemini API
  - [ ] Return JSON response with the answer

---

## 🧪 Testing & Validation

- [ ] Write unit tests for:
  - [ ] Qdrant search
  - [ ] Gemini call
  - [ ] API response formatting
- [ ] Add test inputs (e.g., common developer questions)
- [ ] Validate real answers match expected project logic

---

## 🚀 Deployment (Shuttle or Docker)

- [ ] Add `shuttle.toml` or `Dockerfile`
- [ ] Ensure `.env` is loaded securely
- [ ] Deploy to Shuttle (`cargo shuttle deploy`)
- [ ] Test endpoints live

---

## 📚 Documentation

- [ ] Write `README.md` with:
  - [ ] How to embed code
  - [ ] How to run the Rust server
  - [ ] API usage examples
- [ ] Optional: Swagger/OpenAPI spec for REST API

---

## 📦 Optional Enhancements

- [ ] Use Rust-based tokenizer for improved chunking
- [ ] Support multiple repos/collections in Qdrant
- [ ] Add CLI wrapper for local query
- [ ] Add rate limiting or logging middleware
- [ ] Add Web UI (e.g., Svelte, React, or Shuttle + Tailwind)
