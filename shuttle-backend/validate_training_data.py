#!/usr/bin/env python3
"""
Validate training data quality and completeness for migration guides.
This script checks the quality of generated training data and identifies issues.
"""

import os
import json
import re
from pathlib import Path
from typing import List, Dict, Any, Set, Tuple
import argparse
from collections import defaultdict
from datetime import datetime

class TrainingDataValidator:
    """Validates training data quality and completeness."""
    
    def __init__(self):
        self.validation_results = {}
        self.issues = []
        
        # Quality thresholds
        self.min_question_length = 10
        self.min_answer_length = 50
        self.max_question_length = 200
        self.max_answer_length = 2000
        
        # Expected patterns
        self.question_patterns = [
            r"how\s+do\s+i",
            r"what\s+is",
            r"how\s+does",
            r"what\s+are",
            r"show\s+me",
            r"explain",
            r"compare",
            r"difference",
            r"migrate",
            r"convert"
        ]
        
        # Required concepts for migration guides
        self.required_concepts = {
            "counter", "flipper", "simple_storage", "event_emitter",
            "erc721_nft", "multisig_wallet", "erc20", "erc1155",
            "escrow_vesting", "main_tutorial"
        }
        
        # Expected difficulty levels
        self.expected_difficulties = {"beginner", "intermediate", "advanced"}
        
        # Expected example types
        self.expected_example_types = {
            "comparison", "explanation", "code_example", 
            "migration_step", "pattern", "best_practices"
        }
    
    def load_training_data(self, file_path: str) -> Dict[str, Any]:
        """Load training data from JSON file."""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
            return data
        except Exception as e:
            self.issues.append(f"Error loading training data: {e}")
            return {}
    
    def validate_structure(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Validate the basic structure of training data."""
        results = {
            "has_metadata": False,
            "has_examples": False,
            "metadata_complete": False,
            "structure_issues": []
        }
        
        # Check for required top-level keys
        if "metadata" in data:
            results["has_metadata"] = True
            metadata = data["metadata"]
            
            required_metadata = ["created_at", "total_examples", "guides_processed"]
            missing_metadata = [key for key in required_metadata if key not in metadata]
            
            if not missing_metadata:
                results["metadata_complete"] = True
            else:
                results["structure_issues"].append(f"Missing metadata fields: {missing_metadata}")
        else:
            results["structure_issues"].append("Missing metadata section")
        
        if "examples" in data:
            results["has_examples"] = True
            if not isinstance(data["examples"], list):
                results["structure_issues"].append("Examples should be a list")
        else:
            results["structure_issues"].append("Missing examples section")
        
        return results
    
    def validate_examples(self, examples: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Validate individual training examples."""
        results = {
            "total_examples": len(examples),
            "valid_examples": 0,
            "invalid_examples": 0,
            "quality_issues": [],
            "length_issues": [],
            "content_issues": []
        }
        
        for i, example in enumerate(examples):
            example_issues = []
            
            # Check required fields
            required_fields = ["question", "answer", "guide_name", "difficulty", "example_type"]
            missing_fields = [field for field in required_fields if field not in example]
            
            if missing_fields:
                example_issues.append(f"Missing fields: {missing_fields}")
            
            # Validate question quality
            if "question" in example:
                question = example["question"]
                
                # Length validation
                if len(question) < self.min_question_length:
                    example_issues.append(f"Question too short ({len(question)} chars)")
                elif len(question) > self.max_question_length:
                    example_issues.append(f"Question too long ({len(question)} chars)")
                
                # Pattern validation
                question_lower = question.lower()
                if not any(re.search(pattern, question_lower) for pattern in self.question_patterns):
                    example_issues.append("Question doesn't match expected patterns")
                
                # Check for question mark
                if not question.rstrip().endswith('?'):
                    example_issues.append("Question should end with '?'")
            
            # Validate answer quality
            if "answer" in example:
                answer = example["answer"]
                
                # Length validation
                if len(answer) < self.min_answer_length:
                    example_issues.append(f"Answer too short ({len(answer)} chars)")
                elif len(answer) > self.max_answer_length:
                    example_issues.append(f"Answer too long ({len(answer)} chars)")
                
                # Check for code blocks
                if "code_example" in example.get("example_type", ""):
                    if "```" not in answer:
                        example_issues.append("Code example should contain code blocks")
                
                # Check for ink! and Solidity mentions
                if "ink!" not in answer and "ink" not in answer.lower():
                    example_issues.append("Answer should mention ink!")
            
            # Validate metadata
            if "difficulty" in example:
                if example["difficulty"] not in self.expected_difficulties:
                    example_issues.append(f"Invalid difficulty: {example['difficulty']}")
            
            if "example_type" in example:
                if example["example_type"] not in self.expected_example_types:
                    example_issues.append(f"Invalid example type: {example['example_type']}")
            
            # Record issues
            if example_issues:
                results["invalid_examples"] += 1
                results["quality_issues"].append({
                    "example_index": i,
                    "issues": example_issues
                })
            else:
                results["valid_examples"] += 1
        
        return results
    
    def validate_coverage(self, examples: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Validate coverage of guides, concepts, and patterns."""
        results = {
            "guide_coverage": {},
            "difficulty_distribution": {},
            "example_type_distribution": {},
            "concept_coverage": {},
            "coverage_issues": []
        }
        
        # Analyze guide coverage
        guide_counts = defaultdict(int)
        for example in examples:
            guide_name = example.get("guide_name", "unknown")
            guide_counts[guide_name] += 1
        
        results["guide_coverage"] = dict(guide_counts)
        
        # Check for missing guides
        covered_guides = set(guide_counts.keys())
        missing_guides = self.required_concepts - covered_guides
        if missing_guides:
            results["coverage_issues"].append(f"Missing guides: {missing_guides}")
        
        # Check for uneven distribution
        if guide_counts:
            min_count = min(guide_counts.values())
            max_count = max(guide_counts.values())
            if max_count > min_count * 3:  # More than 3x difference
                results["coverage_issues"].append("Uneven guide distribution")
        
        # Analyze difficulty distribution
        difficulty_counts = defaultdict(int)
        for example in examples:
            difficulty = example.get("difficulty", "unknown")
            difficulty_counts[difficulty] += 1
        
        results["difficulty_distribution"] = dict(difficulty_counts)
        
        # Check difficulty balance
        if difficulty_counts:
            total = sum(difficulty_counts.values())
            for difficulty, count in difficulty_counts.items():
                percentage = (count / total) * 100
                if percentage < 10:  # Less than 10% representation
                    results["coverage_issues"].append(f"Low {difficulty} representation: {percentage:.1f}%")
        
        # Analyze example type distribution
        type_counts = defaultdict(int)
        for example in examples:
            example_type = example.get("example_type", "unknown")
            type_counts[example_type] += 1
        
        results["example_type_distribution"] = dict(type_counts)
        
        # Analyze concept coverage
        concept_counts = defaultdict(int)
        for example in examples:
            concepts = example.get("concepts", [])
            for concept in concepts:
                concept_counts[concept] += 1
        
        results["concept_coverage"] = dict(concept_counts)
        
        return results
    
    def validate_duplicates(self, examples: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Check for duplicate questions and answers."""
        results = {
            "duplicate_questions": [],
            "duplicate_answers": [],
            "similar_questions": [],
            "duplicate_count": 0
        }
        
        # Check for exact duplicates
        question_map = {}
        answer_map = {}
        
        for i, example in enumerate(examples):
            question = example.get("question", "")
            answer = example.get("answer", "")
            
            # Check duplicate questions
            if question in question_map:
                results["duplicate_questions"].append({
                    "question": question,
                    "indices": [question_map[question], i]
                })
            else:
                question_map[question] = i
            
            # Check duplicate answers
            if answer in answer_map:
                results["duplicate_answers"].append({
                    "answer": answer[:100] + "...",
                    "indices": [answer_map[answer], i]
                })
            else:
                answer_map[answer] = i
        
        # Check for similar questions (basic similarity)
        questions = [(i, example.get("question", "")) for i, example in enumerate(examples)]
        
        for i, (idx1, q1) in enumerate(questions):
            for idx2, q2 in questions[i+1:]:
                similarity = self.calculate_similarity(q1, q2)
                if similarity > 0.8:  # High similarity threshold
                    results["similar_questions"].append({
                        "question1": q1,
                        "question2": q2,
                        "indices": [idx1, idx2],
                        "similarity": similarity
                    })
        
        results["duplicate_count"] = len(results["duplicate_questions"]) + len(results["duplicate_answers"])
        
        return results
    
    def calculate_similarity(self, text1: str, text2: str) -> float:
        """Calculate basic similarity between two texts."""
        # Simple word-based similarity
        words1 = set(text1.lower().split())
        words2 = set(text2.lower().split())
        
        if not words1 or not words2:
            return 0.0
        
        intersection = words1.intersection(words2)
        union = words1.union(words2)
        
        return len(intersection) / len(union) if union else 0.0
    
    def validate_language_quality(self, examples: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Validate language quality of questions and answers."""
        results = {
            "language_issues": [],
            "grammar_issues": [],
            "terminology_issues": []
        }
        
        # Common issues to check
        common_issues = [
            (r"\b(solidity|Solidity)\b", "Should use 'Solidity' (capitalized)"),
            (r"\bink\b(?!\!)", "Should use 'ink!' (with exclamation)"),
            (r"\bpolkadot\b", "Should use 'Polkadot' (capitalized)"),
            (r"\bsubstrate\b", "Should use 'Substrate' (capitalized)"),
        ]
        
        for i, example in enumerate(examples):
            question = example.get("question", "")
            answer = example.get("answer", "")
            
            # Check common terminology issues
            for pattern, issue in common_issues:
                if re.search(pattern, question, re.IGNORECASE):
                    results["terminology_issues"].append({
                        "example_index": i,
                        "field": "question",
                        "issue": issue,
                        "text": question
                    })
                
                if re.search(pattern, answer, re.IGNORECASE):
                    results["terminology_issues"].append({
                        "example_index": i,
                        "field": "answer",
                        "issue": issue,
                        "text": answer[:100] + "..."
                    })
        
        return results
    
    def run_validation(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Run comprehensive validation on training data."""
        print("🔍 Starting training data validation...")
        
        # Basic structure validation
        structure_results = self.validate_structure(data)
        print(f"✅ Structure validation: {'PASSED' if structure_results['has_metadata'] and structure_results['has_examples'] else 'FAILED'}")
        
        if not structure_results["has_examples"]:
            return {"error": "No examples found in training data"}
        
        examples = data.get("examples", [])
        
        # Example validation
        example_results = self.validate_examples(examples)
        print(f"✅ Example validation: {example_results['valid_examples']}/{example_results['total_examples']} valid")
        
        # Coverage validation
        coverage_results = self.validate_coverage(examples)
        print(f"✅ Coverage validation: {len(coverage_results['guide_coverage'])} guides covered")
        
        # Duplicate validation
        duplicate_results = self.validate_duplicates(examples)
        print(f"✅ Duplicate validation: {duplicate_results['duplicate_count']} duplicates found")
        
        # Language quality validation
        language_results = self.validate_language_quality(examples)
        print(f"✅ Language validation: {len(language_results['terminology_issues'])} terminology issues")
        
        # Compile overall results
        overall_results = {
            "validation_timestamp": datetime.now().isoformat(),
            "structure_validation": structure_results,
            "example_validation": example_results,
            "coverage_validation": coverage_results,
            "duplicate_validation": duplicate_results,
            "language_validation": language_results,
            "overall_quality_score": self.calculate_quality_score(
                structure_results, example_results, coverage_results, duplicate_results, language_results
            )
        }
        
        return overall_results
    
    def calculate_quality_score(self, structure: Dict, examples: Dict, coverage: Dict, 
                              duplicates: Dict, language: Dict) -> float:
        """Calculate overall quality score (0-100)."""
        score = 100.0
        
        # Structure penalties
        if not structure["has_metadata"]:
            score -= 10
        if not structure["has_examples"]:
            score -= 20
        
        # Example quality penalties
        if examples["total_examples"] > 0:
            invalid_ratio = examples["invalid_examples"] / examples["total_examples"]
            score -= invalid_ratio * 30
        
        # Coverage penalties
        coverage_issues = len(coverage["coverage_issues"])
        score -= coverage_issues * 5
        
        # Duplicate penalties
        if examples["total_examples"] > 0:
            duplicate_ratio = duplicates["duplicate_count"] / examples["total_examples"]
            score -= duplicate_ratio * 20
        
        # Language quality penalties
        terminology_issues = len(language["terminology_issues"])
        score -= min(terminology_issues * 2, 20)  # Cap at 20 points
        
        return max(0.0, score)
    
    def generate_report(self, results: Dict[str, Any]) -> str:
        """Generate a detailed validation report."""
        report = []
        report.append("=" * 60)
        report.append("TRAINING DATA VALIDATION REPORT")
        report.append("=" * 60)
        
        # Overall score
        score = results.get("overall_quality_score", 0)
        report.append(f"\n🎯 Overall Quality Score: {score:.1f}/100")
        
        if score >= 90:
            report.append("🟢 Excellent quality - ready for training")
        elif score >= 75:
            report.append("🟡 Good quality - minor issues to address")
        elif score >= 60:
            report.append("🟠 Moderate quality - several issues to fix")
        else:
            report.append("🔴 Poor quality - major issues require attention")
        
        # Structure validation
        structure = results.get("structure_validation", {})
        report.append(f"\n📋 Structure Validation:")
        report.append(f"   Has metadata: {'✅' if structure.get('has_metadata') else '❌'}")
        report.append(f"   Has examples: {'✅' if structure.get('has_examples') else '❌'}")
        report.append(f"   Metadata complete: {'✅' if structure.get('metadata_complete') else '❌'}")
        
        if structure.get("structure_issues"):
            report.append(f"   Issues: {', '.join(structure['structure_issues'])}")
        
        # Example validation
        examples = results.get("example_validation", {})
        report.append(f"\n📝 Example Validation:")
        report.append(f"   Total examples: {examples.get('total_examples', 0)}")
        report.append(f"   Valid examples: {examples.get('valid_examples', 0)}")
        report.append(f"   Invalid examples: {examples.get('invalid_examples', 0)}")
        
        if examples.get("quality_issues"):
            report.append(f"   Top issues:")
            for issue in examples["quality_issues"][:5]:  # Show first 5 issues
                report.append(f"     - Example {issue['example_index']}: {', '.join(issue['issues'])}")
        
        # Coverage validation
        coverage = results.get("coverage_validation", {})
        report.append(f"\n📊 Coverage Validation:")
        report.append(f"   Guides covered: {len(coverage.get('guide_coverage', {}))}")
        report.append(f"   Guide distribution:")
        
        for guide, count in coverage.get("guide_coverage", {}).items():
            report.append(f"     {guide}: {count} examples")
        
        report.append(f"   Difficulty distribution:")
        for difficulty, count in coverage.get("difficulty_distribution", {}).items():
            report.append(f"     {difficulty}: {count} examples")
        
        if coverage.get("coverage_issues"):
            report.append(f"   Coverage issues: {', '.join(coverage['coverage_issues'])}")
        
        # Duplicate validation
        duplicates = results.get("duplicate_validation", {})
        report.append(f"\n🔍 Duplicate Validation:")
        report.append(f"   Duplicate questions: {len(duplicates.get('duplicate_questions', []))}")
        report.append(f"   Duplicate answers: {len(duplicates.get('duplicate_answers', []))}")
        report.append(f"   Similar questions: {len(duplicates.get('similar_questions', []))}")
        
        # Language validation
        language = results.get("language_validation", {})
        report.append(f"\n🗣️ Language Validation:")
        report.append(f"   Terminology issues: {len(language.get('terminology_issues', []))}")
        
        if language.get("terminology_issues"):
            report.append(f"   Common issues:")
            issue_counts = {}
            for issue in language["terminology_issues"]:
                issue_type = issue["issue"]
                issue_counts[issue_type] = issue_counts.get(issue_type, 0) + 1
            
            for issue_type, count in issue_counts.items():
                report.append(f"     {issue_type}: {count} occurrences")
        
        # Recommendations
        report.append(f"\n💡 Recommendations:")
        
        if score < 75:
            report.append("   - Review and fix invalid examples")
        if coverage.get("coverage_issues"):
            report.append("   - Address coverage gaps")
        if duplicates.get("duplicate_count", 0) > 0:
            report.append("   - Remove or rephrase duplicate content")
        if language.get("terminology_issues"):
            report.append("   - Fix terminology inconsistencies")
        
        report.append(f"\n📅 Validation completed at: {results.get('validation_timestamp', 'unknown')}")
        
        return "\n".join(report)
    
    def save_results(self, results: Dict[str, Any], output_file: str):
        """Save validation results to a file."""
        output_path = Path(output_file)
        output_path.parent.mkdir(exist_ok=True)
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(results, f, indent=2, ensure_ascii=False)
        
        print(f"💾 Validation results saved to: {output_file}")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Validate training data quality")
    parser.add_argument("training_data", help="Path to training data JSON file")
    parser.add_argument("--output", default="validation_results.json", help="Output file for validation results")
    parser.add_argument("--report", default="validation_report.txt", help="Output file for validation report")
    
    args = parser.parse_args()
    
    try:
        validator = TrainingDataValidator()
        
        # Load training data
        data = validator.load_training_data(args.training_data)
        
        if not data:
            print("❌ Failed to load training data")
            return 1
        
        # Run validation
        results = validator.run_validation(data)
        
        # Generate and save report
        report = validator.generate_report(results)
        
        # Save results
        validator.save_results(results, args.output)
        
        # Save report
        with open(args.report, 'w', encoding='utf-8') as f:
            f.write(report)
        
        print(f"📄 Validation report saved to: {args.report}")
        
        # Print summary
        print("\n" + report)
        
        # Return appropriate exit code
        score = results.get("overall_quality_score", 0)
        return 0 if score >= 75 else 1
        
    except Exception as e:
        print(f"Error: {e}")
        return 1

if __name__ == "__main__":
    exit(main())