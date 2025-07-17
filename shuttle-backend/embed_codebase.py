#!/usr/bin/env python3
"""
Python script to embed codebase and store vectors in Qdrant.
This script recursively reads code/doc files, splits them into chunks,
embeds them using sentence-transformers, and stores in Qdrant.
"""

import os
import sys
import json
import hashlib
from pathlib import Path
from typing import List, Dict, Any, Optional
import argparse
from dataclasses import dataclass

# Third-party imports
try:
    from sentence_transformers import SentenceTransformer
    from qdrant_client import QdrantClient
    from qdrant_client.models import VectorParams, Distance, PointStruct
    from qdrant_client.http import models
    import tiktoken
except ImportError as e:
    print(f"Error importing required packages: {e}")
    print("Install with: pip install sentence-transformers qdrant-client tiktoken")
    sys.exit(1)

@dataclass
class CodeChunk:
    """Represents a chunk of code with metadata."""
    content: str
    file_path: str
    chunk_index: int
    start_line: int
    end_line: int
    file_type: str
    language: str

class CodebaseEmbedder:
    """Embeds a codebase into Qdrant vector database."""
    
    def __init__(self, 
                 qdrant_url: str = "http://localhost:6333",
                 qdrant_api_key: Optional[str] = None,
                 collection_name: str = "code_knowledge",
                 model_name: str = "all-MiniLM-L6-v2",
                 chunk_size: int = 1000,
                 chunk_overlap: int = 200):
        """
        Initialize the codebase embedder.
        
        Args:
            qdrant_url: URL of Qdrant instance
            qdrant_api_key: API key for Qdrant Cloud (optional)
            collection_name: Name of the collection to store embeddings
            model_name: Sentence transformer model name
            chunk_size: Maximum tokens per chunk
            chunk_overlap: Overlap between chunks
        """
        self.qdrant_url = qdrant_url
        self.qdrant_api_key = qdrant_api_key
        self.collection_name = collection_name
        self.chunk_size = chunk_size
        self.chunk_overlap = chunk_overlap
        
        # Initialize Qdrant client
        if qdrant_api_key:
            self.qdrant_client = QdrantClient(url=qdrant_url, api_key=qdrant_api_key)
        else:
            self.qdrant_client = QdrantClient(url=qdrant_url)
        
        # Initialize sentence transformer
        print(f"Loading sentence transformer model: {model_name}")
        self.model = SentenceTransformer(model_name)
        
        # Initialize tokenizer for chunk splitting
        self.tokenizer = tiktoken.get_encoding("cl100k_base")
        
        # Supported file extensions
        self.supported_extensions = {
            '.py': 'python',
            '.rs': 'rust',
            '.js': 'javascript',
            '.ts': 'typescript',
            '.java': 'java',
            '.cpp': 'cpp',
            '.c': 'c',
            '.cs': 'csharp',
            '.go': 'go',
            '.rb': 'ruby',
            '.php': 'php',
            '.swift': 'swift',
            '.kt': 'kotlin',
            '.md': 'markdown',
            '.txt': 'text',
            '.yml': 'yaml',
            '.yaml': 'yaml',
            '.json': 'json',
            '.toml': 'toml',
            '.xml': 'xml',
            '.html': 'html',
            '.css': 'css',
            '.sql': 'sql',
            '.sh': 'bash',
            '.dockerfile': 'dockerfile',
            '.gitignore': 'gitignore',
            '.env': 'env',
        }
        
        # Directories to ignore
        self.ignore_dirs = {
            'node_modules', '.git', '__pycache__', '.venv', 'venv', 
            'target', 'build', 'dist', '.next', '.nuxt', 'coverage',
            '.pytest_cache', '.mypy_cache', '.tox', 'htmlcov'
        }
        
        # Files to ignore
        self.ignore_files = {
            '.DS_Store', 'Thumbs.db', '.gitkeep', 'package-lock.json',
            'yarn.lock', 'Cargo.lock', '.env.local', '.env.production'
        }
    
    def should_process_file(self, file_path: Path) -> bool:
        """Check if file should be processed for embedding."""
        # Check if file extension is supported
        if file_path.suffix.lower() not in self.supported_extensions:
            return False
        
        # Check if file is in ignore list
        if file_path.name in self.ignore_files:
            return False
        
        # Check if any parent directory is in ignore list
        for parent in file_path.parents:
            if parent.name in self.ignore_dirs:
                return False
        
        # Check file size (skip very large files)
        try:
            if file_path.stat().st_size > 1024 * 1024:  # 1MB limit
                print(f"Skipping large file: {file_path}")
                return False
        except OSError:
            return False
        
        return True
    
    def read_file_content(self, file_path: Path) -> Optional[str]:
        """Read file content with error handling."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                return f.read()
        except (UnicodeDecodeError, OSError) as e:
            print(f"Error reading {file_path}: {e}")
            return None
    
    def split_text_into_chunks(self, text: str, file_path: str) -> List[CodeChunk]:
        """Split text into overlapping chunks."""
        chunks = []
        tokens = self.tokenizer.encode(text)
        
        if len(tokens) <= self.chunk_size:
            # If text is small enough, return as single chunk
            chunks.append(CodeChunk(
                content=text,
                file_path=file_path,
                chunk_index=0,
                start_line=1,
                end_line=len(text.split('\n')),
                file_type=Path(file_path).suffix.lower(),
                language=self.supported_extensions.get(Path(file_path).suffix.lower(), 'unknown')
            ))
        else:
            # Split into overlapping chunks
            step = self.chunk_size - self.chunk_overlap
            chunk_index = 0
            
            for i in range(0, len(tokens), step):
                chunk_tokens = tokens[i:i + self.chunk_size]
                chunk_text = self.tokenizer.decode(chunk_tokens)
                
                # Estimate line numbers (rough approximation)
                lines = text.split('\n')
                chars_per_line = len(text) / len(lines) if lines else 1
                start_char = len(self.tokenizer.decode(tokens[:i]))
                end_char = len(self.tokenizer.decode(tokens[:i + len(chunk_tokens)]))
                
                start_line = max(1, int(start_char / chars_per_line))
                end_line = min(len(lines), int(end_char / chars_per_line))
                
                chunks.append(CodeChunk(
                    content=chunk_text,
                    file_path=file_path,
                    chunk_index=chunk_index,
                    start_line=start_line,
                    end_line=end_line,
                    file_type=Path(file_path).suffix.lower(),
                    language=self.supported_extensions.get(Path(file_path).suffix.lower(), 'unknown')
                ))
                
                chunk_index += 1
        
        return chunks
    
    def create_collection(self):
        """Create Qdrant collection if it doesn't exist."""
        try:
            # Check if collection exists
            collections = self.qdrant_client.get_collections()
            collection_names = [c.name for c in collections.collections]
            
            if self.collection_name not in collection_names:
                print(f"Creating collection: {self.collection_name}")
                self.qdrant_client.create_collection(
                    collection_name=self.collection_name,
                    vectors_config=VectorParams(
                        size=self.model.get_sentence_embedding_dimension(),
                        distance=Distance.COSINE
                    )
                )
                print(f"Collection '{self.collection_name}' created successfully")
            else:
                print(f"Collection '{self.collection_name}' already exists")
        except Exception as e:
            print(f"Error creating collection: {e}")
            raise
    
    def embed_chunks(self, chunks: List[CodeChunk]) -> List[PointStruct]:
        """Embed chunks and create Qdrant points."""
        points = []
        
        print(f"Embedding {len(chunks)} chunks...")
        
        # Extract text content for embedding
        texts = [chunk.content for chunk in chunks]
        
        # Generate embeddings in batches
        batch_size = 32
        embeddings = []
        
        for i in range(0, len(texts), batch_size):
            batch_texts = texts[i:i + batch_size]
            batch_embeddings = self.model.encode(batch_texts, show_progress_bar=True)
            embeddings.extend(batch_embeddings)
        
        # Create Qdrant points
        for i, (chunk, embedding) in enumerate(zip(chunks, embeddings)):
            # Create unique ID for the chunk
            chunk_id = hashlib.md5(f"{chunk.file_path}_{chunk.chunk_index}".encode()).hexdigest()
            
            # Create metadata payload
            payload = {
                "content": chunk.content,
                "file_path": chunk.file_path,
                "chunk_index": chunk.chunk_index,
                "start_line": chunk.start_line,
                "end_line": chunk.end_line,
                "file_type": chunk.file_type,
                "language": chunk.language,
                "source": "codebase_embedding"
            }
            
            point = PointStruct(
                id=chunk_id,
                vector=embedding.tolist(),
                payload=payload
            )
            points.append(point)
        
        return points
    
    def process_directory(self, directory_path: str) -> List[CodeChunk]:
        """Process all files in a directory recursively."""
        directory = Path(directory_path)
        if not directory.exists():
            raise ValueError(f"Directory does not exist: {directory_path}")
        
        chunks = []
        processed_files = 0
        
        print(f"Processing directory: {directory_path}")
        
        for file_path in directory.rglob("*"):
            if file_path.is_file() and self.should_process_file(file_path):
                relative_path = str(file_path.relative_to(directory))
                print(f"Processing: {relative_path}")
                
                content = self.read_file_content(file_path)
                if content:
                    file_chunks = self.split_text_into_chunks(content, relative_path)
                    chunks.extend(file_chunks)
                    processed_files += 1
        
        print(f"Processed {processed_files} files, created {len(chunks)} chunks")
        return chunks
    
    def embed_codebase(self, directory_path: str):
        """Main method to embed entire codebase."""
        print("Starting codebase embedding process...")
        
        # Create collection
        self.create_collection()
        
        # Process directory and get chunks
        chunks = self.process_directory(directory_path)
        
        if not chunks:
            print("No chunks to embed. Check directory path and file extensions.")
            return
        
        # Embed chunks
        points = self.embed_chunks(chunks)
        
        # Upload to Qdrant
        print(f"Uploading {len(points)} points to Qdrant...")
        
        # Upload in batches to avoid timeouts
        batch_size = 100
        for i in range(0, len(points), batch_size):
            batch_points = points[i:i + batch_size]
            self.qdrant_client.upsert(
                collection_name=self.collection_name,
                points=batch_points
            )
            print(f"Uploaded batch {i//batch_size + 1}/{(len(points) + batch_size - 1)//batch_size}")
        
        print("Codebase embedding completed successfully!")
        
        # Print collection info
        collection_info = self.qdrant_client.get_collection(self.collection_name)
        print(f"Collection '{self.collection_name}' now contains {collection_info.points_count} points")
    
    def test_search(self, query: str, limit: int = 5):
        """Test search functionality."""
        print(f"Testing search with query: '{query}'")
        
        # Embed the query
        query_embedding = self.model.encode([query])[0]
        
        # Search in Qdrant
        results = self.qdrant_client.search(
            collection_name=self.collection_name,
            query_vector=query_embedding.tolist(),
            limit=limit,
            with_payload=True
        )
        
        print(f"Found {len(results)} results:")
        for i, result in enumerate(results):
            print(f"\n{i+1}. Score: {result.score:.4f}")
            print(f"   File: {result.payload['file_path']}")
            print(f"   Lines: {result.payload['start_line']}-{result.payload['end_line']}")
            print(f"   Content preview: {result.payload['content'][:200]}...")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Embed codebase into Qdrant vector database")
    parser.add_argument("directory", help="Directory containing the codebase")
    parser.add_argument("--qdrant-url", default="http://localhost:6334", help="Qdrant URL")
    parser.add_argument("--qdrant-api-key", help="Qdrant API key (for cloud)")
    parser.add_argument("--collection", default="code_knowledge", help="Collection name")
    parser.add_argument("--model", default="all-MiniLM-L6-v2", help="Sentence transformer model")
    parser.add_argument("--chunk-size", type=int, default=1000, help="Chunk size in tokens")
    parser.add_argument("--chunk-overlap", type=int, default=200, help="Chunk overlap in tokens")
    parser.add_argument("--test-query", help="Test search with a query after embedding")
    
    args = parser.parse_args()
    
    # Load API key from environment if not provided
    api_key = args.qdrant_api_key or os.getenv("QDRANT_API_KEY")
    
    try:
        embedder = CodebaseEmbedder(
            qdrant_url=args.qdrant_url,
            qdrant_api_key=api_key,
            collection_name=args.collection,
            model_name=args.model,
            chunk_size=args.chunk_size,
            chunk_overlap=args.chunk_overlap
        )
        
        # Embed the codebase
        embedder.embed_codebase(args.directory)
        
        # Test search if query provided
        if args.test_query:
            embedder.test_search(args.test_query)
            
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()