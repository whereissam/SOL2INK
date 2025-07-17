#!/usr/bin/env python3
"""
Specialized script to embed Solidity to ink! migration guides for AI training.
This script processes migration guides and creates structured embeddings for RAG system.
"""

import os
import sys
import json
import hashlib
import re
from pathlib import Path
from typing import List, Dict, Any, Optional
import argparse
from dataclasses import dataclass
from datetime import datetime

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
class MigrationChunk:
    """Represents a chunk of migration guide content with metadata."""
    content: str
    guide_name: str
    section: str
    subsection: str
    chunk_index: int
    chunk_type: str  # 'overview', 'solidity', 'ink', 'migration', 'example', 'pattern'
    language: str    # 'solidity', 'rust', 'markdown', 'mixed'
    difficulty: str  # 'beginner', 'intermediate', 'advanced'
    concepts: List[str]  # Key concepts covered
    patterns: List[str]  # Design patterns demonstrated

class MigrationGuideEmbedder:
    """Embeds migration guides into Qdrant vector database for RAG system."""
    
    def __init__(self, 
                 qdrant_url: str = "http://localhost:6334",
                 qdrant_api_key: Optional[str] = None,
                 collection_name: str = "migration_guides",
                 model_name: str = "all-MiniLM-L6-v2",
                 chunk_size: int = 800,
                 chunk_overlap: int = 100):
        """
        Initialize the migration guide embedder.
        
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
            # Use gRPC connection for local Qdrant
            if "localhost" in qdrant_url or "127.0.0.1" in qdrant_url:
                self.qdrant_client = QdrantClient(host='localhost', port=6334, grpc_port=6334, prefer_grpc=True)
            else:
                self.qdrant_client = QdrantClient(url=qdrant_url)
        
        # Initialize sentence transformer
        print(f"Loading sentence transformer model: {model_name}")
        self.model = SentenceTransformer(model_name)
        
        # Initialize tokenizer for chunk splitting
        self.tokenizer = tiktoken.get_encoding("cl100k_base")
        
        # Migration guide metadata
        self.guide_metadata = {
            "counter": {"difficulty": "beginner", "concepts": ["state", "functions", "basics"]},
            "flipper": {"difficulty": "beginner", "concepts": ["state", "boolean", "toggle", "events"]},
            "simple_storage": {"difficulty": "intermediate", "concepts": ["mappings", "arrays", "storage"]},
            "event_emitter": {"difficulty": "intermediate", "concepts": ["events", "topics", "indexing"]},
            "erc721_nft": {"difficulty": "advanced", "concepts": ["nft", "tokens", "standards", "metadata"]},
            "multisig_wallet": {"difficulty": "advanced", "concepts": ["multisig", "security", "approvals"]},
            "erc20": {"difficulty": "intermediate", "concepts": ["tokens", "standards", "transfers", "allowances"]},
            "erc1155": {"difficulty": "advanced", "concepts": ["multi-token", "batch", "fungible", "non-fungible"]},
            "escrow_vesting": {"difficulty": "advanced", "concepts": ["escrow", "vesting", "time-locks", "payments"]}
        }
        
        # Common migration patterns
        self.migration_patterns = [
            "storage_conversion", "event_handling", "error_handling", "access_control",
            "state_machine", "token_standard", "batch_operations", "time_based",
            "multi_party", "fee_calculation", "withdrawal_pattern", "approval_pattern"
        ]
    
    def extract_guide_sections(self, content: str, guide_name: str) -> List[Dict[str, Any]]:
        """Extract structured sections from migration guide markdown."""
        sections = []
        
        # Split by main headings
        section_pattern = r'^## (.+?)$'
        section_matches = list(re.finditer(section_pattern, content, re.MULTILINE))
        
        for i, match in enumerate(section_matches):
            section_title = match.group(1).strip()
            start_pos = match.end()
            end_pos = section_matches[i + 1].start() if i + 1 < len(section_matches) else len(content)
            section_content = content[start_pos:end_pos].strip()
            
            # Determine section type
            section_type = self.classify_section(section_title, section_content)
            
            # Extract code blocks
            code_blocks = self.extract_code_blocks(section_content)
            
            sections.append({
                "title": section_title,
                "content": section_content,
                "type": section_type,
                "code_blocks": code_blocks,
                "guide_name": guide_name
            })
        
        return sections
    
    def classify_section(self, title: str, content: str) -> str:
        """Classify section type based on title and content."""
        title_lower = title.lower()
        
        if "overview" in title_lower:
            return "overview"
        elif "solidity" in title_lower:
            return "solidity"
        elif "ink" in title_lower:
            return "ink"
        elif "migration" in title_lower or "steps" in title_lower:
            return "migration"
        elif "pattern" in title_lower or "example" in title_lower:
            return "pattern"
        elif "key" in title_lower and "point" in title_lower:
            return "comparison"
        elif "best" in title_lower and "practice" in title_lower:
            return "best_practices"
        else:
            return "general"
    
    def extract_code_blocks(self, content: str) -> List[Dict[str, str]]:
        """Extract code blocks with language information."""
        code_blocks = []
        
        # Pattern to match code blocks with language specification
        code_pattern = r'```(\w+)?\n(.*?)```'
        matches = re.finditer(code_pattern, content, re.DOTALL)
        
        for match in matches:
            language = match.group(1) or 'unknown'
            code_content = match.group(2).strip()
            
            code_blocks.append({
                "language": language,
                "content": code_content
            })
        
        return code_blocks
    
    def create_migration_chunks(self, guide_content: str, guide_name: str) -> List[MigrationChunk]:
        """Create structured chunks from migration guide content."""
        chunks = []
        
        # Get guide metadata
        metadata = self.guide_metadata.get(guide_name, {
            "difficulty": "intermediate",
            "concepts": ["general"]
        })
        
        # Extract sections
        sections = self.extract_guide_sections(guide_content, guide_name)
        
        for section in sections:
            # Create chunks for each section
            section_chunks = self.chunk_section(section, guide_name, metadata)
            chunks.extend(section_chunks)
        
        return chunks
    
    def chunk_section(self, section: Dict[str, Any], guide_name: str, metadata: Dict[str, Any]) -> List[MigrationChunk]:
        """Create chunks from a single section."""
        chunks = []
        content = section["content"]
        section_title = section["title"]
        section_type = section["type"]
        
        # Token-based chunking
        tokens = self.tokenizer.encode(content)
        
        if len(tokens) <= self.chunk_size:
            # Single chunk
            chunk = MigrationChunk(
                content=content,
                guide_name=guide_name,
                section=section_title,
                subsection="",
                chunk_index=0,
                chunk_type=section_type,
                language=self.detect_primary_language(content),
                difficulty=metadata["difficulty"],
                concepts=metadata["concepts"],
                patterns=self.detect_patterns(content)
            )
            chunks.append(chunk)
        else:
            # Multiple chunks
            step = self.chunk_size - self.chunk_overlap
            chunk_index = 0
            
            for i in range(0, len(tokens), step):
                chunk_tokens = tokens[i:i + self.chunk_size]
                chunk_content = self.tokenizer.decode(chunk_tokens)
                
                chunk = MigrationChunk(
                    content=chunk_content,
                    guide_name=guide_name,
                    section=section_title,
                    subsection=f"part_{chunk_index + 1}",
                    chunk_index=chunk_index,
                    chunk_type=section_type,
                    language=self.detect_primary_language(chunk_content),
                    difficulty=metadata["difficulty"],
                    concepts=metadata["concepts"],
                    patterns=self.detect_patterns(chunk_content)
                )
                chunks.append(chunk)
                chunk_index += 1
        
        return chunks
    
    def detect_primary_language(self, content: str) -> str:
        """Detect the primary language of content."""
        solidity_keywords = ["pragma", "contract", "function", "modifier", "msg.sender", "require"]
        rust_keywords = ["#[ink", "impl", "pub fn", "Result<", "AccountId", "Balance"]
        
        solidity_count = sum(1 for keyword in solidity_keywords if keyword in content)
        rust_count = sum(1 for keyword in rust_keywords if keyword in content)
        
        if solidity_count > rust_count:
            return "solidity"
        elif rust_count > solidity_count:
            return "rust"
        else:
            return "mixed"
    
    def detect_patterns(self, content: str) -> List[str]:
        """Detect migration patterns in content."""
        patterns = []
        content_lower = content.lower()
        
        pattern_keywords = {
            "storage_conversion": ["mapping", "storage", "struct"],
            "event_handling": ["event", "emit", "topic"],
            "error_handling": ["error", "result", "require"],
            "access_control": ["modifier", "only", "authorized"],
            "state_machine": ["enum", "state", "transition"],
            "token_standard": ["erc", "token", "transfer"],
            "batch_operations": ["batch", "array", "loop"],
            "time_based": ["timestamp", "deadline", "duration"],
            "multi_party": ["multisig", "approval", "consensus"],
            "fee_calculation": ["fee", "percentage", "basis"],
            "withdrawal_pattern": ["withdraw", "pending", "balance"],
            "approval_pattern": ["approve", "allowance", "operator"]
        }
        
        for pattern, keywords in pattern_keywords.items():
            if any(keyword in content_lower for keyword in keywords):
                patterns.append(pattern)
        
        return patterns
    
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
    
    def embed_chunks(self, chunks: List[MigrationChunk]) -> List[PointStruct]:
        """Embed migration chunks and create Qdrant points."""
        points = []
        
        print(f"Embedding {len(chunks)} migration guide chunks...")
        
        # Prepare texts for embedding
        texts = []
        for chunk in chunks:
            # Create enriched text for better embedding
            enriched_text = self.create_enriched_text(chunk)
            texts.append(enriched_text)
        
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
            chunk_id = hashlib.md5(
                f"{chunk.guide_name}_{chunk.section}_{chunk.chunk_index}".encode()
            ).hexdigest()
            
            # Create comprehensive metadata payload
            payload = {
                "content": chunk.content,
                "guide_name": chunk.guide_name,
                "section": chunk.section,
                "subsection": chunk.subsection,
                "chunk_index": chunk.chunk_index,
                "chunk_type": chunk.chunk_type,
                "language": chunk.language,
                "difficulty": chunk.difficulty,
                "concepts": chunk.concepts,
                "patterns": chunk.patterns,
                "source": "migration_guide",
                "timestamp": datetime.now().isoformat()
            }
            
            point = PointStruct(
                id=chunk_id,
                vector=embedding.tolist(),
                payload=payload
            )
            points.append(point)
        
        return points
    
    def create_enriched_text(self, chunk: MigrationChunk) -> str:
        """Create enriched text for better embedding quality."""
        # Add context and metadata to improve embedding
        context_parts = [
            f"Migration Guide: {chunk.guide_name}",
            f"Section: {chunk.section}",
            f"Difficulty: {chunk.difficulty}",
            f"Concepts: {', '.join(chunk.concepts)}",
            f"Language: {chunk.language}",
            f"Type: {chunk.chunk_type}"
        ]
        
        if chunk.patterns:
            context_parts.append(f"Patterns: {', '.join(chunk.patterns)}")
        
        context = " | ".join(context_parts)
        
        # Combine context with content
        enriched_text = f"{context}\n\n{chunk.content}"
        
        return enriched_text
    
    def process_solidity_examples(self, solidity_dir: str) -> List[MigrationChunk]:
        """Process Solidity example contracts."""
        solidity_path = Path(solidity_dir)
        chunks = []
        
        if not solidity_path.exists():
            print(f"Solidity examples directory not found: {solidity_dir}")
            return chunks
            
        print(f"Processing Solidity examples from: {solidity_dir}")
        
        src_path = solidity_path / "src"
        if src_path.exists():
            for sol_file in src_path.glob("*.sol"):
                try:
                    with open(sol_file, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    # Extract contract name from filename
                    contract_name = sol_file.stem.lower()
                    
                    # Create chunk for Solidity example
                    chunk = MigrationChunk(
                        content=f"Solidity Implementation of {contract_name}:\n\n```solidity\n{content}\n```",
                        guide_name=contract_name,
                        section="Solidity Implementation",
                        subsection="",
                        chunk_index=0,
                        chunk_type="solidity_example",
                        language="solidity",
                        difficulty=self.guide_metadata.get(contract_name, {"difficulty": "intermediate"}).get("difficulty", "intermediate"),
                        concepts=self.guide_metadata.get(contract_name, {"concepts": ["smart_contracts"]}).get("concepts", ["smart_contracts"]),
                        patterns=self.detect_patterns(content)
                    )
                    chunks.append(chunk)
                    print(f"  Processed Solidity: {contract_name}")
                    
                except Exception as e:
                    print(f"Error processing {sol_file}: {e}")
        
        return chunks
    
    def process_ink_examples(self, ink_dir: str) -> List[MigrationChunk]:
        """Process ink! example contracts."""
        ink_path = Path(ink_dir)
        chunks = []
        
        if not ink_path.exists():
            print(f"ink! examples directory not found: {ink_dir}")
            return chunks
            
        print(f"Processing ink! examples from: {ink_dir}")
        
        # Process individual contract directories
        for contract_dir in ink_path.iterdir():
            if contract_dir.is_dir() and (contract_dir / "lib.rs").exists():
                try:
                    lib_file = contract_dir / "lib.rs"
                    with open(lib_file, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    contract_name = contract_dir.name.replace("-", "_")
                    
                    # Create chunk for ink! example
                    chunk = MigrationChunk(
                        content=f"ink! Implementation of {contract_name}:\n\n```rust\n{content}\n```",
                        guide_name=contract_name,
                        section="ink! Implementation",
                        subsection="",
                        chunk_index=0,
                        chunk_type="ink_example",
                        language="rust",
                        difficulty=self.guide_metadata.get(contract_name, {"difficulty": "intermediate"}).get("difficulty", "intermediate"),
                        concepts=self.guide_metadata.get(contract_name, {"concepts": ["smart_contracts"]}).get("concepts", ["smart_contracts"]),
                        patterns=self.detect_patterns(content)
                    )
                    chunks.append(chunk)
                    print(f"  Processed ink!: {contract_name}")
                    
                except Exception as e:
                    print(f"Error processing {contract_dir}: {e}")
        
        return chunks
    
    def process_migration_guides(self, guides_directory: str) -> List[MigrationChunk]:
        """Process all migration guides in the directory."""
        guides_path = Path(guides_directory)
        if not guides_path.exists():
            raise ValueError(f"Directory does not exist: {guides_directory}")
        
        chunks = []
        processed_guides = 0
        
        print(f"Processing migration guides from: {guides_directory}")
        
        # Process each .md file
        for guide_file in guides_path.glob("migration_guide_*.md"):
            guide_name = guide_file.stem.replace("migration_guide_", "")
            print(f"Processing guide: {guide_name}")
            
            try:
                with open(guide_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                guide_chunks = self.create_migration_chunks(content, guide_name)
                chunks.extend(guide_chunks)
                processed_guides += 1
                
                print(f"  Created {len(guide_chunks)} chunks for {guide_name}")
                
            except Exception as e:
                print(f"Error processing {guide_file}: {e}")
        
        # Also process the main tutorial
        tutorial_file = guides_path / "SOLIDITY_TO_INK_TUTORIAL.md"
        if tutorial_file.exists():
            print("Processing main tutorial...")
            try:
                with open(tutorial_file, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                guide_chunks = self.create_migration_chunks(content, "main_tutorial")
                chunks.extend(guide_chunks)
                processed_guides += 1
                
                print(f"  Created {len(guide_chunks)} chunks for main tutorial")
                
            except Exception as e:
                print(f"Error processing tutorial: {e}")
        
        print(f"Processed {processed_guides} guides, created {len(chunks)} total chunks")
        return chunks
    
    def embed_migration_guides(self, guides_directory: str, solidity_dir: str = None, ink_dir: str = None):
        """Main method to embed migration guides and examples."""
        print("Starting migration guide and examples embedding process...")
        
        # Create collection
        self.create_collection()
        
        # Process guides and get chunks
        chunks = self.process_migration_guides(guides_directory)
        
        # Process Solidity examples if provided
        if solidity_dir:
            solidity_chunks = self.process_solidity_examples(solidity_dir)
            chunks.extend(solidity_chunks)
            print(f"Added {len(solidity_chunks)} Solidity example chunks")
        
        # Process ink! examples if provided
        if ink_dir:
            ink_chunks = self.process_ink_examples(ink_dir)
            chunks.extend(ink_chunks)
            print(f"Added {len(ink_chunks)} ink! example chunks")
        
        if not chunks:
            print("No chunks to embed. Check directory paths.")
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
        
        print("Migration guide and examples embedding completed successfully!")
        
        # Print collection info
        collection_info = self.qdrant_client.get_collection(self.collection_name)
        print(f"Collection '{self.collection_name}' now contains {collection_info.points_count} points")
        
        # Print summary statistics
        self.print_embedding_stats(chunks)
    
    def print_embedding_stats(self, chunks: List[MigrationChunk]):
        """Print statistics about the embedded chunks."""
        print("\n=== Embedding Statistics ===")
        
        # Guide distribution
        guide_counts = {}
        for chunk in chunks:
            guide_counts[chunk.guide_name] = guide_counts.get(chunk.guide_name, 0) + 1
        
        print(f"Chunks per guide:")
        for guide, count in sorted(guide_counts.items()):
            print(f"  {guide}: {count} chunks")
        
        # Difficulty distribution
        difficulty_counts = {}
        for chunk in chunks:
            difficulty_counts[chunk.difficulty] = difficulty_counts.get(chunk.difficulty, 0) + 1
        
        print(f"\nDifficulty distribution:")
        for difficulty, count in sorted(difficulty_counts.items()):
            print(f"  {difficulty}: {count} chunks")
        
        # Language distribution
        language_counts = {}
        for chunk in chunks:
            language_counts[chunk.language] = language_counts.get(chunk.language, 0) + 1
        
        print(f"\nLanguage distribution:")
        for language, count in sorted(language_counts.items()):
            print(f"  {language}: {count} chunks")
        
        # Top concepts
        concept_counts = {}
        for chunk in chunks:
            for concept in chunk.concepts:
                concept_counts[concept] = concept_counts.get(concept, 0) + 1
        
        print(f"\nTop concepts:")
        for concept, count in sorted(concept_counts.items(), key=lambda x: x[1], reverse=True)[:10]:
            print(f"  {concept}: {count} chunks")
    
    def test_search(self, query: str, limit: int = 5):
        """Test search functionality with migration-specific features."""
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
            payload = result.payload
            print(f"\n{i+1}. Score: {result.score:.4f}")
            print(f"   Guide: {payload['guide_name']}")
            print(f"   Section: {payload['section']}")
            print(f"   Type: {payload['chunk_type']}")
            print(f"   Language: {payload['language']}")
            print(f"   Difficulty: {payload['difficulty']}")
            print(f"   Concepts: {', '.join(payload['concepts'])}")
            if payload['patterns']:
                print(f"   Patterns: {', '.join(payload['patterns'])}")
            print(f"   Content preview: {payload['content'][:200]}...")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Embed migration guides and examples into Qdrant vector database")
    parser.add_argument("directory", help="Directory containing migration guides")
    parser.add_argument("--solidity-examples", help="Directory containing Solidity examples")
    parser.add_argument("--ink-examples", help="Directory containing ink! examples")
    parser.add_argument("--qdrant-url", default="http://localhost:6334", help="Qdrant URL")
    parser.add_argument("--qdrant-api-key", help="Qdrant API key (for cloud)")
    parser.add_argument("--collection", default="migration_guides", help="Collection name")
    parser.add_argument("--model", default="all-MiniLM-L6-v2", help="Sentence transformer model")
    parser.add_argument("--chunk-size", type=int, default=800, help="Chunk size in tokens")
    parser.add_argument("--chunk-overlap", type=int, default=100, help="Chunk overlap in tokens")
    parser.add_argument("--test-query", help="Test search with a query after embedding")
    
    args = parser.parse_args()
    
    # Load API key from environment if not provided
    api_key = args.qdrant_api_key or os.getenv("QDRANT_API_KEY")
    
    try:
        embedder = MigrationGuideEmbedder(
            qdrant_url=args.qdrant_url,
            qdrant_api_key=api_key,
            collection_name=args.collection,
            model_name=args.model,
            chunk_size=args.chunk_size,
            chunk_overlap=args.chunk_overlap
        )
        
        # Embed the migration guides and examples
        embedder.embed_migration_guides(
            args.directory, 
            solidity_dir=args.solidity_examples,
            ink_dir=args.ink_examples
        )
        
        # Test search if query provided
        if args.test_query:
            embedder.test_search(args.test_query)
            
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()