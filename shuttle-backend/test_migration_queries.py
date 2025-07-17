#!/usr/bin/env python3
"""
Test script for migration guide queries and validation.
This script tests the embedded migration guides with various queries.
"""

import os
import sys
import json
from pathlib import Path
from typing import List, Dict, Any, Optional
import argparse

# Third-party imports
try:
    from sentence_transformers import SentenceTransformer
    from qdrant_client import QdrantClient
    from qdrant_client.models import VectorParams, Distance, PointStruct
    from qdrant_client.http import models
except ImportError as e:
    print(f"Error importing required packages: {e}")
    print("Install with: pip install sentence-transformers qdrant-client")
    sys.exit(1)

class MigrationQueryTester:
    """Test migration guide embeddings with various queries."""
    
    def __init__(self, 
                 qdrant_url: str = "http://localhost:6334",
                 qdrant_api_key: Optional[str] = None,
                 collection_name: str = "migration_guides",
                 model_name: str = "all-MiniLM-L6-v2"):
        """Initialize the query tester."""
        self.qdrant_url = qdrant_url
        self.qdrant_api_key = qdrant_api_key
        self.collection_name = collection_name
        
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
        
        # Test queries organized by category
        self.test_queries = {
            "basic_concepts": [
                "How do I create a simple contract in ink!?",
                "What are the main differences between Solidity and ink!?",
                "How do I define storage in ink! contracts?",
                "What is the ink! equivalent of Solidity modifiers?",
                "How do I handle errors in ink! contracts?"
            ],
            "token_standards": [
                "How do I implement an ERC20 token in ink!?",
                "What are the differences between ERC20 in Solidity and ink!?",
                "How do I create an NFT contract in ink!?",
                "How do I implement ERC1155 multi-token standard in ink!?",
                "What are the allowance patterns in ink! tokens?"
            ],
            "events_and_topics": [
                "How do I emit events in ink! contracts?",
                "What is the difference between Solidity and ink! event handling?",
                "How do I use topics in ink! events?",
                "How do I index events in ink!?"
            ],
            "storage_patterns": [
                "How do I use mappings in ink! contracts?",
                "What is the difference between Solidity and ink! storage?",
                "How do I handle nested mappings in ink!?",
                "What are the best practices for storage in ink!?"
            ],
            "access_control": [
                "How do I implement access control in ink!?",
                "What is the ink! equivalent of Solidity modifiers?",
                "How do I restrict function access in ink!?",
                "How do I implement multi-signature wallets in ink!?"
            ],
            "advanced_patterns": [
                "How do I implement escrow contracts in ink!?",
                "What are vesting contracts in ink!?",
                "How do I handle time-based operations in ink!?",
                "How do I implement batch operations in ink!?"
            ],
            "migration_specific": [
                "How do I migrate a Solidity contract to ink!?",
                "What are the steps to convert Solidity to ink!?",
                "What are common pitfalls when migrating to ink!?",
                "How do I test migrated ink! contracts?"
            ],
            "best_practices": [
                "What are the best practices for ink! contract development?",
                "How do I optimize gas costs in ink!?",
                "What are security considerations in ink!?",
                "How do I structure large ink! contracts?"
            ]
        }
    
    def search_query(self, query: str, limit: int = 5, score_threshold: float = 0.0) -> List[Dict[str, Any]]:
        """Search for a query in the migration guides."""
        # Embed the query
        query_embedding = self.model.encode([query])[0]
        
        # Search in Qdrant
        try:
            results = self.qdrant_client.search(
                collection_name=self.collection_name,
                query_vector=query_embedding.tolist(),
                limit=limit,
                score_threshold=score_threshold,
                with_payload=True
            )
            
            # Format results
            formatted_results = []
            for result in results:
                formatted_results.append({
                    "score": result.score,
                    "guide_name": result.payload.get("guide_name", "unknown"),
                    "section": result.payload.get("section", "unknown"),
                    "chunk_type": result.payload.get("chunk_type", "unknown"),
                    "language": result.payload.get("language", "unknown"),
                    "difficulty": result.payload.get("difficulty", "unknown"),
                    "concepts": result.payload.get("concepts", []),
                    "patterns": result.payload.get("patterns", []),
                    "content": result.payload.get("content", "")
                })
            
            return formatted_results
            
        except Exception as e:
            print(f"Error searching query '{query}': {e}")
            return []
    
    def test_single_query(self, query: str, expected_guides: List[str] = None) -> Dict[str, Any]:
        """Test a single query and return results."""
        print(f"\n🔍 Testing query: '{query}'")
        
        results = self.search_query(query, limit=5)
        
        if not results:
            print("❌ No results found")
            return {
                "query": query,
                "success": False,
                "results": [],
                "expected_guides": expected_guides or []
            }
        
        print(f"✅ Found {len(results)} results")
        
        # Display top results
        for i, result in enumerate(results[:3]):
            print(f"\n{i+1}. Score: {result['score']:.4f}")
            print(f"   Guide: {result['guide_name']}")
            print(f"   Section: {result['section']}")
            print(f"   Type: {result['chunk_type']}")
            print(f"   Language: {result['language']}")
            print(f"   Difficulty: {result['difficulty']}")
            print(f"   Concepts: {', '.join(result['concepts'])}")
            if result['patterns']:
                print(f"   Patterns: {', '.join(result['patterns'])}")
            print(f"   Content preview: {result['content'][:200]}...")
        
        # Check if expected guides are found
        found_guides = [r['guide_name'] for r in results]
        expected_found = []
        if expected_guides:
            expected_found = [guide for guide in expected_guides if guide in found_guides]
            if expected_found:
                print(f"\n✅ Expected guides found: {', '.join(expected_found)}")
            else:
                print(f"\n⚠️  Expected guides not found: {', '.join(expected_guides)}")
        
        return {
            "query": query,
            "success": True,
            "results": results,
            "expected_guides": expected_guides or [],
            "expected_found": expected_found
        }
    
    def test_category(self, category: str, queries: List[str]) -> Dict[str, Any]:
        """Test all queries in a category."""
        print(f"\n{'='*60}")
        print(f"Testing category: {category.upper()}")
        print(f"{'='*60}")
        
        results = []
        successful_queries = 0
        
        for query in queries:
            result = self.test_single_query(query)
            results.append(result)
            if result["success"]:
                successful_queries += 1
        
        success_rate = (successful_queries / len(queries)) * 100 if queries else 0
        
        print(f"\n📊 Category '{category}' Results:")
        print(f"   Successful queries: {successful_queries}/{len(queries)} ({success_rate:.1f}%)")
        
        return {
            "category": category,
            "queries_tested": len(queries),
            "successful_queries": successful_queries,
            "success_rate": success_rate,
            "results": results
        }
    
    def run_comprehensive_test(self) -> Dict[str, Any]:
        """Run comprehensive test on all query categories."""
        print("🚀 Starting comprehensive migration guide query test...")
        
        # Check collection exists
        try:
            collection_info = self.qdrant_client.get_collection(self.collection_name)
            print(f"✅ Collection '{self.collection_name}' found with {collection_info.points_count} points")
        except Exception as e:
            print(f"❌ Error accessing collection: {e}")
            return {"error": str(e)}
        
        all_results = []
        total_queries = 0
        total_successful = 0
        
        # Test each category
        for category, queries in self.test_queries.items():
            category_result = self.test_category(category, queries)
            all_results.append(category_result)
            total_queries += category_result["queries_tested"]
            total_successful += category_result["successful_queries"]
        
        # Calculate overall statistics
        overall_success_rate = (total_successful / total_queries) * 100 if total_queries else 0
        
        print(f"\n{'='*60}")
        print(f"COMPREHENSIVE TEST RESULTS")
        print(f"{'='*60}")
        print(f"Total queries tested: {total_queries}")
        print(f"Successful queries: {total_successful}")
        print(f"Overall success rate: {overall_success_rate:.1f}%")
        
        # Category performance summary
        print(f"\n📊 Category Performance:")
        for result in all_results:
            print(f"   {result['category']}: {result['success_rate']:.1f}% ({result['successful_queries']}/{result['queries_tested']})")
        
        return {
            "total_queries": total_queries,
            "total_successful": total_successful,
            "overall_success_rate": overall_success_rate,
            "category_results": all_results
        }
    
    def test_guide_coverage(self) -> Dict[str, Any]:
        """Test coverage of all migration guides."""
        print(f"\n{'='*60}")
        print(f"TESTING GUIDE COVERAGE")
        print(f"{'='*60}")
        
        expected_guides = [
            "counter", "flipper", "simple_storage", "event_emitter",
            "erc721_nft", "multisig_wallet", "erc20", "erc1155",
            "escrow_vesting", "main_tutorial"
        ]
        
        coverage_results = {}
        
        for guide in expected_guides:
            # Test if guide is accessible
            test_query = f"How do I implement {guide} in ink!?"
            results = self.search_query(test_query, limit=10)
            
            # Count results from this guide
            guide_results = [r for r in results if r['guide_name'] == guide]
            
            coverage_results[guide] = {
                "total_results": len(results),
                "guide_specific_results": len(guide_results),
                "accessible": len(guide_results) > 0
            }
            
            status = "✅" if len(guide_results) > 0 else "❌"
            print(f"{status} {guide}: {len(guide_results)} specific results out of {len(results)} total")
        
        accessible_guides = sum(1 for r in coverage_results.values() if r["accessible"])
        coverage_percentage = (accessible_guides / len(expected_guides)) * 100
        
        print(f"\n📊 Guide Coverage Summary:")
        print(f"   Accessible guides: {accessible_guides}/{len(expected_guides)} ({coverage_percentage:.1f}%)")
        
        return {
            "expected_guides": expected_guides,
            "accessible_guides": accessible_guides,
            "coverage_percentage": coverage_percentage,
            "guide_results": coverage_results
        }
    
    def test_specific_scenarios(self) -> Dict[str, Any]:
        """Test specific migration scenarios."""
        print(f"\n{'='*60}")
        print(f"TESTING SPECIFIC SCENARIOS")
        print(f"{'='*60}")
        
        scenarios = [
            {
                "name": "Beginner Migration",
                "query": "I'm new to ink!, how do I start migrating from Solidity?",
                "expected_guides": ["counter", "flipper", "main_tutorial"],
                "expected_difficulty": "beginner"
            },
            {
                "name": "Token Standard Migration",
                "query": "How do I migrate my ERC20 token to ink!?",
                "expected_guides": ["erc20"],
                "expected_concepts": ["tokens", "transfers", "allowances"]
            },
            {
                "name": "Complex Contract Migration",
                "query": "How do I migrate a complex multisig wallet to ink!?",
                "expected_guides": ["multisig_wallet"],
                "expected_difficulty": "advanced"
            },
            {
                "name": "Event Handling Migration",
                "query": "How do events work differently in ink! compared to Solidity?",
                "expected_guides": ["event_emitter"],
                "expected_concepts": ["events", "topics"]
            },
            {
                "name": "Storage Pattern Migration",
                "query": "How do I convert Solidity mappings to ink! storage?",
                "expected_guides": ["simple_storage"],
                "expected_concepts": ["mappings", "storage"]
            }
        ]
        
        scenario_results = []
        
        for scenario in scenarios:
            print(f"\n🎯 Testing scenario: {scenario['name']}")
            print(f"   Query: {scenario['query']}")
            
            results = self.search_query(scenario['query'], limit=5)
            
            # Check expectations
            found_guides = [r['guide_name'] for r in results]
            expected_guides_found = [g for g in scenario.get('expected_guides', []) if g in found_guides]
            
            # Check difficulty if specified
            difficulty_match = False
            if 'expected_difficulty' in scenario:
                difficulty_match = any(r['difficulty'] == scenario['expected_difficulty'] for r in results)
            
            # Check concepts if specified
            concepts_match = False
            if 'expected_concepts' in scenario:
                for result in results:
                    if any(concept in result['concepts'] for concept in scenario['expected_concepts']):
                        concepts_match = True
                        break
            
            scenario_result = {
                "name": scenario['name'],
                "query": scenario['query'],
                "results_found": len(results),
                "expected_guides": scenario.get('expected_guides', []),
                "expected_guides_found": expected_guides_found,
                "difficulty_match": difficulty_match,
                "concepts_match": concepts_match,
                "success": len(results) > 0 and len(expected_guides_found) > 0
            }
            
            status = "✅" if scenario_result["success"] else "❌"
            print(f"   {status} Found {len(results)} results, expected guides: {expected_guides_found}")
            
            scenario_results.append(scenario_result)
        
        successful_scenarios = sum(1 for r in scenario_results if r["success"])
        success_rate = (successful_scenarios / len(scenarios)) * 100
        
        print(f"\n📊 Scenario Test Summary:")
        print(f"   Successful scenarios: {successful_scenarios}/{len(scenarios)} ({success_rate:.1f}%)")
        
        return {
            "scenarios_tested": len(scenarios),
            "successful_scenarios": successful_scenarios,
            "success_rate": success_rate,
            "scenario_results": scenario_results
        }
    
    def save_test_results(self, results: Dict[str, Any], output_file: str):
        """Save test results to a JSON file."""
        output_path = Path(output_file)
        output_path.parent.mkdir(exist_ok=True)
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(results, f, indent=2, ensure_ascii=False)
        
        print(f"\n💾 Test results saved to: {output_file}")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Test migration guide queries")
    parser.add_argument("--qdrant-url", default="http://localhost:6334", help="Qdrant URL")
    parser.add_argument("--qdrant-api-key", help="Qdrant API key (for cloud)")
    parser.add_argument("--collection", default="migration_guides", help="Collection name")
    parser.add_argument("--model", default="all-MiniLM-L6-v2", help="Sentence transformer model")
    parser.add_argument("--output", default="test_results.json", help="Output file for results")
    parser.add_argument("--query", help="Test a single query")
    parser.add_argument("--category", help="Test queries in a specific category")
    
    args = parser.parse_args()
    
    # Load API key from environment if not provided
    api_key = args.qdrant_api_key or os.getenv("QDRANT_API_KEY")
    
    try:
        tester = MigrationQueryTester(
            qdrant_url=args.qdrant_url,
            qdrant_api_key=api_key,
            collection_name=args.collection,
            model_name=args.model
        )
        
        if args.query:
            # Test single query
            result = tester.test_single_query(args.query)
            print(f"\nQuery test {'✅ passed' if result['success'] else '❌ failed'}")
        
        elif args.category:
            # Test specific category
            if args.category in tester.test_queries:
                result = tester.test_category(args.category, tester.test_queries[args.category])
                print(f"\nCategory test completed with {result['success_rate']:.1f}% success rate")
            else:
                print(f"Category '{args.category}' not found. Available categories:")
                for category in tester.test_queries.keys():
                    print(f"  - {category}")
        
        else:
            # Run comprehensive test
            comprehensive_results = tester.run_comprehensive_test()
            coverage_results = tester.test_guide_coverage()
            scenario_results = tester.test_specific_scenarios()
            
            # Combine all results
            all_results = {
                "comprehensive_test": comprehensive_results,
                "coverage_test": coverage_results,
                "scenario_test": scenario_results,
                "test_timestamp": str(pd.Timestamp.now()) if 'pd' in globals() else "unknown"
            }
            
            # Save results
            tester.save_test_results(all_results, args.output)
            
            print(f"\n🎉 All tests completed!")
            print(f"Overall success rate: {comprehensive_results.get('overall_success_rate', 0):.1f}%")
            print(f"Guide coverage: {coverage_results.get('coverage_percentage', 0):.1f}%")
            print(f"Scenario success: {scenario_results.get('success_rate', 0):.1f}%")
            
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()