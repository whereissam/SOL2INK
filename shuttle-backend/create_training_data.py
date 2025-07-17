#!/usr/bin/env python3
"""
Create structured training data from migration guides for AI fine-tuning.
This script generates question-answer pairs and training examples from migration guides.
"""

import os
import json
import re
from pathlib import Path
from typing import List, Dict, Any, Tuple
import argparse
from dataclasses import dataclass
from datetime import datetime

@dataclass
class TrainingExample:
    """Represents a training example for AI fine-tuning."""
    question: str
    answer: str
    context: str
    guide_name: str
    section: str
    difficulty: str
    concepts: List[str]
    patterns: List[str]
    example_type: str  # 'comparison', 'explanation', 'code_example', 'migration_step'

class MigrationTrainingDataGenerator:
    """Generates training data from migration guides."""
    
    def __init__(self):
        self.training_examples = []
        
        # Guide metadata
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
        
        # Question templates for different types of queries
        self.question_templates = {
            "comparison": [
                "What are the differences between Solidity and ink! for {concept}?",
                "How does {concept} implementation differ between Solidity and ink!?",
                "Compare {concept} in Solidity vs ink!",
                "What changes when migrating {concept} from Solidity to ink!?"
            ],
            "explanation": [
                "How do you implement {concept} in ink!?",
                "Explain {concept} in ink! smart contracts",
                "What is the ink! approach to {concept}?",
                "How does {concept} work in ink!?"
            ],
            "code_example": [
                "Show me how to implement {concept} in ink!",
                "Provide an example of {concept} in ink!",
                "What does {concept} look like in ink! code?",
                "Give me a code example for {concept} in ink!"
            ],
            "migration_step": [
                "How do I migrate {concept} from Solidity to ink!?",
                "What are the steps to convert {concept} from Solidity to ink!?",
                "Guide me through migrating {concept} to ink!",
                "What's the migration process for {concept}?"
            ],
            "pattern": [
                "What is the {pattern} pattern in ink!?",
                "How do you implement {pattern} in ink!?",
                "Show me the {pattern} pattern for ink! contracts",
                "Explain the {pattern} pattern in ink!"
            ],
            "best_practices": [
                "What are the best practices for {concept} in ink!?",
                "How should I handle {concept} in ink! contracts?",
                "What are common pitfalls with {concept} in ink!?",
                "What are the recommendations for {concept} in ink!?"
            ]
        }
    
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
    
    def extract_sections(self, content: str) -> Dict[str, str]:
        """Extract sections from markdown content."""
        sections = {}
        
        # Split by main headings
        section_pattern = r'^## (.+?)$'
        section_matches = list(re.finditer(section_pattern, content, re.MULTILINE))
        
        for i, match in enumerate(section_matches):
            section_title = match.group(1).strip()
            start_pos = match.end()
            end_pos = section_matches[i + 1].start() if i + 1 < len(section_matches) else len(content)
            section_content = content[start_pos:end_pos].strip()
            
            sections[section_title] = section_content
        
        return sections
    
    def generate_comparison_examples(self, guide_name: str, sections: Dict[str, str]) -> List[TrainingExample]:
        """Generate comparison-type training examples."""
        examples = []
        metadata = self.guide_metadata.get(guide_name, {"difficulty": "intermediate", "concepts": ["general"]})
        
        # Look for sections with code comparisons
        for section_title, section_content in sections.items():
            if "implementation" in section_title.lower() or "solidity" in section_title.lower():
                code_blocks = self.extract_code_blocks(section_content)
                
                # Find Solidity and ink! code pairs
                solidity_code = None
                ink_code = None
                
                for block in code_blocks:
                    if block["language"] == "solidity":
                        solidity_code = block["content"]
                    elif block["language"] == "rust":
                        ink_code = block["content"]
                
                if solidity_code and ink_code:
                    # Generate comparison questions
                    for concept in metadata["concepts"]:
                        for template in self.question_templates["comparison"]:
                            question = template.format(concept=concept)
                            
                            answer = f"When migrating {concept} from Solidity to ink!, here are the key differences:\n\n"
                            answer += f"**Solidity Implementation:**\n```solidity\n{solidity_code[:500]}...\n```\n\n"
                            answer += f"**ink! Implementation:**\n```rust\n{ink_code[:500]}...\n```\n\n"
                            answer += f"The main differences include storage structure, error handling, and type safety."
                            
                            examples.append(TrainingExample(
                                question=question,
                                answer=answer,
                                context=section_content,
                                guide_name=guide_name,
                                section=section_title,
                                difficulty=metadata["difficulty"],
                                concepts=metadata["concepts"],
                                patterns=[],
                                example_type="comparison"
                            ))
                            
                            # Limit examples per concept
                            if len(examples) >= 2:
                                break
                    break
        
        return examples
    
    def generate_explanation_examples(self, guide_name: str, sections: Dict[str, str]) -> List[TrainingExample]:
        """Generate explanation-type training examples."""
        examples = []
        metadata = self.guide_metadata.get(guide_name, {"difficulty": "intermediate", "concepts": ["general"]})
        
        # Look for overview and explanation sections
        for section_title, section_content in sections.items():
            if "overview" in section_title.lower() or "explanation" in section_title.lower():
                
                for concept in metadata["concepts"]:
                    for template in self.question_templates["explanation"]:
                        question = template.format(concept=concept)
                        
                        # Create comprehensive answer
                        answer = f"{section_content[:800]}...\n\n"
                        answer += f"This demonstrates the ink! approach to {concept} implementation."
                        
                        examples.append(TrainingExample(
                            question=question,
                            answer=answer,
                            context=section_content,
                            guide_name=guide_name,
                            section=section_title,
                            difficulty=metadata["difficulty"],
                            concepts=metadata["concepts"],
                            patterns=[],
                            example_type="explanation"
                        ))
                        
                        # Limit examples per concept
                        if len([e for e in examples if e.guide_name == guide_name]) >= 3:
                            break
                break
        
        return examples
    
    def generate_code_examples(self, guide_name: str, sections: Dict[str, str]) -> List[TrainingExample]:
        """Generate code example training examples."""
        examples = []
        metadata = self.guide_metadata.get(guide_name, {"difficulty": "intermediate", "concepts": ["general"]})
        
        # Look for ink! implementation sections
        for section_title, section_content in sections.items():
            if "ink!" in section_title and "implementation" in section_title.lower():
                code_blocks = self.extract_code_blocks(section_content)
                
                for block in code_blocks:
                    if block["language"] == "rust":
                        for concept in metadata["concepts"]:
                            for template in self.question_templates["code_example"]:
                                question = template.format(concept=concept)
                                
                                answer = f"Here's how to implement {concept} in ink!:\n\n"
                                answer += f"```rust\n{block['content'][:1000]}...\n```\n\n"
                                answer += f"This example shows the ink! approach with proper error handling and type safety."
                                
                                examples.append(TrainingExample(
                                    question=question,
                                    answer=answer,
                                    context=section_content,
                                    guide_name=guide_name,
                                    section=section_title,
                                    difficulty=metadata["difficulty"],
                                    concepts=metadata["concepts"],
                                    patterns=[],
                                    example_type="code_example"
                                ))
                                
                                # Limit examples per concept
                                if len([e for e in examples if e.guide_name == guide_name and e.example_type == "code_example"]) >= 2:
                                    break
                        break
                break
        
        return examples
    
    def generate_migration_step_examples(self, guide_name: str, sections: Dict[str, str]) -> List[TrainingExample]:
        """Generate migration step training examples."""
        examples = []
        metadata = self.guide_metadata.get(guide_name, {"difficulty": "intermediate", "concepts": ["general"]})
        
        # Look for migration steps sections
        for section_title, section_content in sections.items():
            if "migration" in section_title.lower() and "step" in section_title.lower():
                
                for concept in metadata["concepts"]:
                    for template in self.question_templates["migration_step"]:
                        question = template.format(concept=concept)
                        
                        answer = f"To migrate {concept} from Solidity to ink!, follow these steps:\n\n"
                        answer += f"{section_content[:1000]}...\n\n"
                        answer += f"These steps ensure a smooth transition while maintaining functionality."
                        
                        examples.append(TrainingExample(
                            question=question,
                            answer=answer,
                            context=section_content,
                            guide_name=guide_name,
                            section=section_title,
                            difficulty=metadata["difficulty"],
                            concepts=metadata["concepts"],
                            patterns=[],
                            example_type="migration_step"
                        ))
                        
                        # Limit examples per concept
                        if len([e for e in examples if e.guide_name == guide_name and e.example_type == "migration_step"]) >= 2:
                            break
                break
        
        return examples
    
    def generate_pattern_examples(self, guide_name: str, sections: Dict[str, str]) -> List[TrainingExample]:
        """Generate pattern-based training examples."""
        examples = []
        metadata = self.guide_metadata.get(guide_name, {"difficulty": "intermediate", "concepts": ["general"]})
        
        # Common patterns in migration guides
        patterns = {
            "storage_pattern": "storage structure and mapping conversion",
            "event_pattern": "event definition and emission",
            "error_pattern": "error handling and Result types",
            "access_pattern": "access control and modifiers",
            "payment_pattern": "payment handling and transfers"
        }
        
        for section_title, section_content in sections.items():
            if "pattern" in section_title.lower() or "common" in section_title.lower():
                
                for pattern_name, pattern_desc in patterns.items():
                    if any(word in section_content.lower() for word in pattern_desc.split()):
                        for template in self.question_templates["pattern"]:
                            question = template.format(pattern=pattern_desc)
                            
                            answer = f"The {pattern_desc} pattern in ink! works as follows:\n\n"
                            answer += f"{section_content[:800]}...\n\n"
                            answer += f"This pattern ensures clean and efficient ink! implementation."
                            
                            examples.append(TrainingExample(
                                question=question,
                                answer=answer,
                                context=section_content,
                                guide_name=guide_name,
                                section=section_title,
                                difficulty=metadata["difficulty"],
                                concepts=metadata["concepts"],
                                patterns=[pattern_name],
                                example_type="pattern"
                            ))
                            break
                break
        
        return examples
    
    def generate_best_practices_examples(self, guide_name: str, sections: Dict[str, str]) -> List[TrainingExample]:
        """Generate best practices training examples."""
        examples = []
        metadata = self.guide_metadata.get(guide_name, {"difficulty": "intermediate", "concepts": ["general"]})
        
        # Look for best practices sections
        for section_title, section_content in sections.items():
            if "best" in section_title.lower() and "practice" in section_title.lower():
                
                for concept in metadata["concepts"]:
                    for template in self.question_templates["best_practices"]:
                        question = template.format(concept=concept)
                        
                        answer = f"Best practices for {concept} in ink!:\n\n"
                        answer += f"{section_content[:800]}...\n\n"
                        answer += f"Following these practices ensures robust and maintainable ink! contracts."
                        
                        examples.append(TrainingExample(
                            question=question,
                            answer=answer,
                            context=section_content,
                            guide_name=guide_name,
                            section=section_title,
                            difficulty=metadata["difficulty"],
                            concepts=metadata["concepts"],
                            patterns=[],
                            example_type="best_practices"
                        ))
                        
                        # Limit examples per concept
                        if len([e for e in examples if e.guide_name == guide_name and e.example_type == "best_practices"]) >= 2:
                            break
                break
        
        return examples
    
    def process_guide(self, guide_path: Path) -> List[TrainingExample]:
        """Process a single migration guide and generate training examples."""
        guide_name = guide_path.stem
        if guide_name.startswith("migration_guide_"):
            guide_name = guide_name.replace("migration_guide_", "")
        elif guide_name == "SOLIDITY_TO_INK_TUTORIAL":
            guide_name = "main_tutorial"
        
        print(f"Processing guide: {guide_name}")
        
        # Read guide content
        with open(guide_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract sections
        sections = self.extract_sections(content)
        
        # Generate different types of training examples
        examples = []
        
        # Generate comparison examples
        examples.extend(self.generate_comparison_examples(guide_name, sections))
        
        # Generate explanation examples
        examples.extend(self.generate_explanation_examples(guide_name, sections))
        
        # Generate code examples
        examples.extend(self.generate_code_examples(guide_name, sections))
        
        # Generate migration step examples
        examples.extend(self.generate_migration_step_examples(guide_name, sections))
        
        # Generate pattern examples
        examples.extend(self.generate_pattern_examples(guide_name, sections))
        
        # Generate best practices examples
        examples.extend(self.generate_best_practices_examples(guide_name, sections))
        
        print(f"Generated {len(examples)} training examples for {guide_name}")
        return examples
    
    def process_all_guides(self, guides_directory: str) -> List[TrainingExample]:
        """Process all migration guides and generate training examples."""
        guides_path = Path(guides_directory)
        if not guides_path.exists():
            raise ValueError(f"Directory does not exist: {guides_directory}")
        
        all_examples = []
        
        print(f"Processing migration guides from: {guides_directory}")
        
        # Process all .md files
        for guide_file in guides_path.glob("*.md"):
            try:
                examples = self.process_guide(guide_file)
                all_examples.extend(examples)
            except Exception as e:
                print(f"Error processing {guide_file}: {e}")
        
        print(f"Generated {len(all_examples)} total training examples")
        return all_examples
    
    def save_training_data(self, examples: List[TrainingExample], output_dir: str):
        """Save training examples in multiple formats."""
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)
        
        # Save as JSONL for fine-tuning
        jsonl_path = output_path / "migration_training_data.jsonl"
        with open(jsonl_path, 'w', encoding='utf-8') as f:
            for example in examples:
                training_item = {
                    "messages": [
                        {"role": "system", "content": "You are an expert in migrating smart contracts from Solidity to ink!. Provide detailed, accurate guidance on contract migration."},
                        {"role": "user", "content": example.question},
                        {"role": "assistant", "content": example.answer}
                    ],
                    "metadata": {
                        "guide_name": example.guide_name,
                        "section": example.section,
                        "difficulty": example.difficulty,
                        "concepts": example.concepts,
                        "patterns": example.patterns,
                        "example_type": example.example_type
                    }
                }
                f.write(json.dumps(training_item) + '\n')
        
        # Save as structured JSON
        json_path = output_path / "migration_training_data.json"
        with open(json_path, 'w', encoding='utf-8') as f:
            training_data = {
                "metadata": {
                    "created_at": datetime.now().isoformat(),
                    "total_examples": len(examples),
                    "guides_processed": len(set(e.guide_name for e in examples))
                },
                "examples": [
                    {
                        "question": e.question,
                        "answer": e.answer,
                        "context": e.context,
                        "guide_name": e.guide_name,
                        "section": e.section,
                        "difficulty": e.difficulty,
                        "concepts": e.concepts,
                        "patterns": e.patterns,
                        "example_type": e.example_type
                    }
                    for e in examples
                ]
            }
            json.dump(training_data, f, indent=2, ensure_ascii=False)
        
        # Save statistics
        stats_path = output_path / "training_data_stats.json"
        with open(stats_path, 'w', encoding='utf-8') as f:
            stats = self.generate_statistics(examples)
            json.dump(stats, f, indent=2, ensure_ascii=False)
        
        print(f"Training data saved to {output_path}")
        print(f"  - JSONL format: {jsonl_path}")
        print(f"  - JSON format: {json_path}")
        print(f"  - Statistics: {stats_path}")
    
    def generate_statistics(self, examples: List[TrainingExample]) -> Dict[str, Any]:
        """Generate statistics about the training data."""
        stats = {
            "total_examples": len(examples),
            "guides_processed": len(set(e.guide_name for e in examples)),
            "created_at": datetime.now().isoformat()
        }
        
        # Guide distribution
        guide_counts = {}
        for example in examples:
            guide_counts[example.guide_name] = guide_counts.get(example.guide_name, 0) + 1
        stats["guide_distribution"] = guide_counts
        
        # Difficulty distribution
        difficulty_counts = {}
        for example in examples:
            difficulty_counts[example.difficulty] = difficulty_counts.get(example.difficulty, 0) + 1
        stats["difficulty_distribution"] = difficulty_counts
        
        # Example type distribution
        type_counts = {}
        for example in examples:
            type_counts[example.example_type] = type_counts.get(example.example_type, 0) + 1
        stats["example_type_distribution"] = type_counts
        
        # Concept distribution
        concept_counts = {}
        for example in examples:
            for concept in example.concepts:
                concept_counts[concept] = concept_counts.get(concept, 0) + 1
        stats["concept_distribution"] = concept_counts
        
        return stats

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Generate training data from migration guides")
    parser.add_argument("guides_directory", help="Directory containing migration guides")
    parser.add_argument("--output-dir", default="training_data", help="Output directory for training data")
    
    args = parser.parse_args()
    
    try:
        generator = MigrationTrainingDataGenerator()
        
        # Process all guides
        examples = generator.process_all_guides(args.guides_directory)
        
        # Save training data
        generator.save_training_data(examples, args.output_dir)
        
        print(f"\nTraining data generation completed!")
        print(f"Generated {len(examples)} training examples")
        
    except Exception as e:
        print(f"Error: {e}")
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main())