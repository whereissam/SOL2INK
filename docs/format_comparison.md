# API Response Format Improvements

## Current Issues

Your current API response format is hard to read in raw JSON:

```json
{
  "success": true,
  "data": "# Answer to: Show me ERC20 token implementation in ink\n\n## Code Example 1 (Score: 0.06)\n\n```rust\n\n// ERC20 Token Implementation in ink!\n#[ink::contract]\nmod erc20 {\n    use ink::storage::Mapping;\n\n    /// A simple ERC-20 contract.\n    #[ink(storage)]\n    #[derive(Default)]\n    pub struct Erc20 {\n        /// Total token supply.\n        total_supply: Balance,\n        /// Mapping from owner to number of owned token.\n        balances: Mapping<AccountId, Balance>,\n        /// Mapping of the token amount which an account is allowed to withdraw\n        /// from another account.\n        allowances: Mapping<(AccountId, AccountId), Balance>,\n    }\n\n    impl Erc20 {\n        /// Creates a new ERC-20 contract with the specified initial supply.\n        #[ink(constructor)]\n        pub fn new(total_supply: Balance) -> Self {\n            let mut balances = Mapping::default();\n            let caller = Self::env().caller();\n            balances.insert(caller, &total_supply);\n            Self::env().emit_event(Transfer {\n                from: None,\n                to: Some(caller),\n                value: total_supply,\n            });\n            Self {\n                total_supply,\n                balances,\n                allowances: Default::default(),\n            }\n        }\n\n        /// Returns the total token supply.\n        #[ink(message)]\n        pub fn total_supply(&self) -> Balance {\n            self.total_supply\n        }\n\n        /// Returns the account balance for the specified `owner`.\n        #[ink(message)]\n        pub fn balance_of(&self, owner: AccountId) -> Balance {\n            self.balance_of_impl(&owner)\n        }\n\n        /// Transfers `value` amount of tokens from the caller's account to account `to`.\n        #[ink(message)]\n        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {\n            let from = self.env().caller();\n            self.transfer_from_to(&from, &to, value)\n        }\n\n        /// Allows `spender` to withdraw from the caller's account multiple times, up to\n        /// the `value` amount.\n        #[ink(message)]\n        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {\n            let owner = self.env().caller();\n            self.allowances.insert((&owner, &spender), &value);\n            self.env().emit_event(Approval {\n                owner,\n                spender,\n                value,\n            });\n            Ok(())\n        }\n    }\n}\n\n```\n\n*Source: ink-examples/erc20/lib.rs*\n\n## Code Example 2 (Score: 0.05)\n\n```rust\n\n// Cross-contract calls in ink!\n#[ink::contract]\nmod cross_contract_calls {\n    use other_contract::OtherContractRef;\n\n    #[ink(storage)]\n    pub struct CrossContractCalls {\n        /// Address of the other contract\n        other_contract: OtherContractRef,\n    }\n\n    impl CrossContractCalls {\n        #[ink(constructor)]\n        pub fn new(other_contract_code_hash: Hash) -> Self {\n            let other_contract = OtherContractRef::new()\n                .code_hash(other_contract_code_hash)\n                .endowment(0)\n                .salt_bytes([0; 4])\n                .instantiate();\n            Self { other_contract }\n        }\n\n        /// Call another contract's method\n        #[ink(message)]\n        pub fn call_other_contract(&self) -> u32 {\n            self.other_contract.get_value()\n        }\n\n        /// Call and forward any errors\n        #[ink(message)]\n        pub fn call_other_contract_and_forward_errors(&self) -> Result<u32, OtherError> {\n            self.other_contract.get_value_or_error()\n        }\n    }\n}\n\n```\n\n*Source: ink-examples/cross-contract-calls/lib.rs*\n\n## Code Example 3 (Score: 0.03)\n\n```rust\n\n// Basic contract with storage mapping in ink!\n#[ink::contract]\nmod mapping {\n    use ink::storage::Mapping;\n\n    #[ink(storage)]\n    pub struct MappingContract {\n        /// A simple mapping from AccountId to Balance\n        balances: Mapping<AccountId, Balance>,\n        /// Total supply of tokens\n        total_supply: Balance,\n    }\n\n    impl MappingContract {\n        #[ink(constructor)]\n        pub fn new() -> Self {\n            Self {\n                balances: Mapping::default(),\n                total_supply: 0,\n            }\n        }\n\n        /// Set balance for an account\n        #[ink(message)]\n        pub fn set_balance(&mut self, account: AccountId, balance: Balance) {\n            // Remove old balance from total supply\n            let old_balance = self.balances.get(&account).unwrap_or(0);\n            self.total_supply = self.total_supply - old_balance + balance;\n            \n            // Set new balance\n            self.balances.insert(&account, &balance);\n        }\n\n        /// Get balance for an account\n        #[ink(message)]\n        pub fn get_balance(&self, account: AccountId) -> Balance {\n            self.balances.get(&account).unwrap_or(0)\n        }\n\n        /// Get total supply\n        #[ink(message)]\n        pub fn get_total_supply(&self) -> Balance {\n            self.total_supply\n        }\n    }\n}\n\n```\n\n*Source: ink-examples/mapping/lib.rs*\n\n\n---\n\n**Note:** This response is generated from the embedded ink! smart contract examples. The code snippets shown are the most relevant matches found in the codebase.\n",
  "error": null
}
```

## Solutions Implemented

### 1. Improved Markdown Format (`/ask` endpoint)

The response is now cleaner and more readable:

```markdown
🔍 **Query:** Show me ERC20 token implementation in ink

📋 **Summary:** Here are the most relevant code examples from the ink! smart contract library:

## 📄 Example 1: erc20
**Relevance Score:** 6.0%

**Description:** A simple ERC-20 contract.

```rust
#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from owner to number of owned token.
        balances: Mapping<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }
    
    // ... more code ...
}
```

📁 **Source:** `ink-examples/erc20/lib.rs`

---

💡 **Need help?** These examples show real ink! smart contract implementations. You can use them as templates for your own contracts.
```

### 2. Structured JSON Response (`/ask/structured` endpoint)

A much more developer-friendly JSON structure:

```json
{
  "success": true,
  "data": {
    "query": "Show me ERC20 token implementation in ink",
    "summary": "Found 3 relevant ink! smart contract examples matching your query. These examples demonstrate best practices and common patterns in ink! development.",
    "examples": [
      {
        "title": "erc20",
        "description": "A simple ERC-20 contract.",
        "code": "formatted rust code here...",
        "source_file": "ink-examples/erc20/lib.rs",
        "relevance_score": 6.0
      },
      {
        "title": "cross_contract_calls",
        "description": "Cross-contract calls in ink!",
        "code": "formatted rust code here...",
        "source_file": "ink-examples/cross-contract-calls/lib.rs",
        "relevance_score": 5.0
      }
    ],
    "help_text": "These examples are from the official ink! examples repository. You can use them as templates for building your own smart contracts on Polkadot."
  },
  "error": null
}
```

### 3. Frontend Demo (`demo.html`)

I've created a beautiful HTML frontend that demonstrates both formats:

- **Structured view**: Clean cards with titles, descriptions, and relevance scores
- **Markdown view**: Rendered markdown with syntax highlighting
- **Quick examples**: Clickable example queries
- **Responsive design**: Works on mobile and desktop

## Benefits

1. **Better UX**: Clean, readable responses instead of raw markdown in JSON
2. **Easier Integration**: Structured data makes it simple to build frontends
3. **Flexibility**: Choose between markdown (for simple display) or structured (for rich UIs)
4. **Developer Friendly**: Clear separation of data elements (title, description, code, source)
5. **Enhanced Metadata**: Relevance scores, source files, and descriptions

## Usage

### For Simple Display (Markdown)
```bash
curl -X POST "http://localhost:8000/ask" \
  -H "Content-Type: application/json" \
  -d '{"query": "Show me ERC20 token implementation in ink"}'
```

### For Rich UI (Structured)
```bash
curl -X POST "http://localhost:8000/ask/structured" \
  -H "Content-Type: application/json" \
  -d '{"query": "Show me ERC20 token implementation in ink"}'
```

### Frontend Demo
Open `demo.html` in your browser to see both formats in action with a beautiful UI.

## Next Steps

1. **Deploy the updated backend** with the new endpoints
2. **Use the structured endpoint** for building rich user interfaces
3. **Customize the frontend** to match your brand and requirements
4. **Add more features** like code copying, syntax highlighting, and examples filtering