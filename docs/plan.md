# TDD Plan: Solidity to ink! Migration Training System

## Overview
Build a training system that helps Solidity developers migrate their contracts to Polkadot's ink! framework using the existing ink-examples and solidity-examples codebases.

## Test-Driven Development Approach

### Phase 1: Core Contract Analysis Engine

#### Test 1: [ ] Parse Solidity ERC20 Contract Structure
- **Test**: `should_parse_solidity_erc20_contract_and_extract_functions`
- **Input**: `solidity-examples/src/SimpleERC20.sol`
- **Expected Output**: List of functions with signatures, state variables, events
- **Implementation**: Create `SolidityParser` struct with `parse_contract()` method

#### Test 2: [ ] Parse ink! ERC20 Contract Structure  
- **Test**: `should_parse_ink_erc20_contract_and_extract_functions`
- **Input**: `ink-examples-main/erc20/lib.rs`
- **Expected Output**: List of functions with signatures, storage items, events
- **Implementation**: Create `InkParser` struct with `parse_contract()` method

#### Test 3: [ ] Create Contract Mapping System
- **Test**: `should_map_solidity_functions_to_ink_equivalents`
- **Input**: Parsed Solidity and ink! contracts
- **Expected Output**: Mapping of equivalent functions and their differences
- **Implementation**: Create `ContractMapper` with `create_mapping()` method

### Phase 2: Migration Pattern Recognition

#### Test 4: [ ] Identify Storage Pattern Differences
- **Test**: `should_identify_solidity_mapping_to_ink_mapping_patterns`
- **Input**: Solidity `mapping(address => uint256) balances` 
- **Expected Output**: ink! `Mapping<AccountId, Balance>` with usage differences
- **Implementation**: Create `StoragePatternAnalyzer` 

#### Test 5: [ ] Identify Function Modifier Patterns
- **Test**: `should_convert_solidity_modifiers_to_ink_patterns`
- **Input**: Solidity `onlyOwner` modifier
- **Expected Output**: ink! ownership check pattern with `ensure!` macro
- **Implementation**: Create `ModifierConverter`

#### Test 6: [ ] Identify Event Pattern Differences
- **Test**: `should_convert_solidity_events_to_ink_events`
- **Input**: Solidity `event Transfer(address indexed from, address indexed to, uint256 value)`
- **Expected Output**: ink! event struct with `#[ink(event)]` and `#[ink(topic)]`
- **Implementation**: Create `EventConverter`

### Phase 3: Code Generation Engine

#### Test 7: [ ] Generate Basic ink! Contract Structure
- **Test**: `should_generate_basic_ink_contract_from_solidity`
- **Input**: Solidity contract structure
- **Expected Output**: Basic ink! contract with storage, constructor, and function stubs
- **Implementation**: Create `InkCodeGenerator`

#### Test 8: [ ] Generate Function Implementations
- **Test**: `should_generate_ink_function_implementations`
- **Input**: Solidity function signature and body
- **Expected Output**: Equivalent ink! function with proper error handling
- **Implementation**: Extend `InkCodeGenerator` with `generate_function()`

#### Test 9: [ ] Generate Error Handling Patterns
- **Test**: `should_convert_solidity_require_to_ink_result`
- **Input**: Solidity `require(condition, "message")`
- **Expected Output**: ink! `ensure!(condition, Error::Message)`
- **Implementation**: Create `ErrorPatternConverter`

### Phase 4: Training Content Generation

#### Test 10: [ ] Generate Step-by-Step Migration Guide
- **Test**: `should_generate_migration_guide_for_erc20`
- **Input**: Solidity ERC20 contract
- **Expected Output**: Markdown guide with step-by-step migration instructions
- **Implementation**: Create `MigrationGuideGenerator`

#### Test 11: [ ] Generate Code Comparison Examples
- **Test**: `should_generate_side_by_side_code_comparison`
- **Input**: Solidity and equivalent ink! code
- **Expected Output**: HTML/Markdown with side-by-side comparison
- **Implementation**: Create `CodeComparisonGenerator`

#### Test 12: [ ] Generate Practice Exercises
- **Test**: `should_generate_practice_exercises_for_migration`
- **Input**: Contract type (ERC20, ERC721, ERC1155)
- **Expected Output**: Series of incremental migration exercises
- **Implementation**: Create `ExerciseGenerator`

### Phase 5: Interactive Training API

#### Test 13: [ ] Create Contract Analysis Endpoint
- **Test**: `should_analyze_uploaded_solidity_contract`
- **Input**: POST request with Solidity contract code
- **Expected Output**: JSON with contract analysis and migration suggestions
- **Implementation**: Create `POST /api/analyze` endpoint

#### Test 14: [ ] Create Migration Generation Endpoint
- **Test**: `should_generate_ink_migration_for_contract`
- **Input**: POST request with Solidity contract and migration preferences
- **Expected Output**: Generated ink! contract with explanations
- **Implementation**: Create `POST /api/migrate` endpoint

#### Test 15: [ ] Create Training Progress Endpoint
- **Test**: `should_track_user_training_progress`
- **Input**: User ID and completed exercises
- **Expected Output**: Progress tracking and next recommended exercises
- **Implementation**: Create progress tracking system

### Phase 6: Advanced Features

#### Test 16: [ ] Handle Complex Contract Patterns
- **Test**: `should_handle_inheritance_patterns`
- **Input**: Solidity contract with inheritance
- **Expected Output**: ink! trait-based equivalent
- **Implementation**: Create `InheritanceConverter`

#### Test 17: [ ] Handle DeFi-Specific Patterns
- **Test**: `should_convert_defi_patterns_to_ink`
- **Input**: Solidity DeFi contract (DEX, lending, etc.)
- **Expected Output**: ink! equivalent with Polkadot ecosystem integration
- **Implementation**: Create `DeFiPatternConverter`

#### Test 18: [ ] Generate Test Migration
- **Test**: `should_convert_solidity_tests_to_ink_tests`
- **Input**: Solidity test files
- **Expected Output**: ink! test equivalents with proper mocking
- **Implementation**: Create `TestMigrationGenerator`

### Phase 7: Integration and Deployment

#### Test 19: [ ] Create Web Interface
- **Test**: `should_serve_training_web_interface`
- **Input**: Web request to training interface
- **Expected Output**: Interactive web UI for migration training
- **Implementation**: Create web frontend with upload and generation features

#### Test 20: [ ] Integrate with Existing RAG System
- **Test**: `should_integrate_with_existing_rag_backend`
- **Input**: Training queries about migration patterns
- **Expected Output**: Contextual help from existing documentation
- **Implementation**: Integrate with shuttle-backend RAG system

## Implementation Strategy

### Project Structure
```
src/
├── parsers/
│   ├── solidity_parser.rs    # Test 1
│   └── ink_parser.rs         # Test 2
├── mappers/
│   ├── contract_mapper.rs    # Test 3
│   ├── storage_analyzer.rs   # Test 4
│   ├── modifier_converter.rs # Test 5
│   └── event_converter.rs    # Test 6
├── generators/
│   ├── ink_generator.rs      # Test 7, 8
│   ├── error_converter.rs    # Test 9
│   ├── guide_generator.rs    # Test 10
│   ├── comparison_generator.rs # Test 11
│   └── exercise_generator.rs # Test 12
├── api/
│   └── endpoints.rs          # Test 13, 14, 15
├── advanced/
│   ├── inheritance_converter.rs # Test 16
│   ├── defi_converter.rs     # Test 17
│   └── test_migrator.rs      # Test 18
└── web/
    └── interface.rs          # Test 19, 20
```

### Dependencies
```toml
[dependencies]
# Existing dependencies from shuttle-backend
shuttle-runtime = "0.47.0"
shuttle-axum = "0.47.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"

# New dependencies for parsing and generation
syn = "2.0"           # Rust syntax parsing
quote = "1.0"         # Code generation
proc-macro2 = "1.0"   # Token manipulation
regex = "1.10"        # Pattern matching
askama = "0.12"       # Template engine
tree-sitter = "0.20"  # Solidity parsing
walkdir = "2.4"       # File system traversal
```

## Testing Strategy

### Unit Tests
- Each test should be independent and focus on a single function
- Use mock inputs from actual contract files
- Test both success and failure cases
- Validate output format and content accuracy

### Integration Tests
- Test complete migration flow from Solidity to ink!
- Use real contract examples from both codebases
- Validate generated code compiles and functions correctly
- Test API endpoints with realistic payloads

### E2E Tests
- Test complete training workflow
- Validate web interface functionality
- Test integration with existing RAG system
- Performance testing with large contracts

## Success Criteria

1. **Functional**: System can parse and migrate basic ERC20, ERC721, ERC1155 contracts
2. **Educational**: Generated migration guides are clear and comprehensive
3. **Accurate**: Generated ink! code compiles and maintains equivalent functionality
4. **Scalable**: System can handle complex contracts with multiple inheritance
5. **Integrated**: Seamlessly works with existing RAG backend for contextual help

## Next Steps

1. Start with Test 1: Parse Solidity ERC20 contract structure
2. Follow TDD cycle: Write failing test → Implement minimum code → Refactor
3. Complete each test before moving to the next
4. Maintain clean separation between structural and behavioral changes
5. Commit after each successful test implementation