use crate::rag_system::RAGSystem;
use std::collections::HashMap;
use tracing::info;

pub async fn populate_sample_data(rag_system: &RAGSystem) -> Result<(), anyhow::Error> {
    info!("Populating RAG system with ink! smart contract examples...");
    
    let sample_documents = vec![
        (
            r#"
// ERC20 Token Implementation in ink!
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

    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }

        /// Transfers `value` amount of tokens from the caller's account to account `to`.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Allows `spender` to withdraw from the caller's account multiple times, up to
        /// the `value` amount.
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "erc20".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/erc20/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "token".to_string()),
            ])
        ),
        (
            r#"
// Simple Flipper Contract in ink!
#[ink::contract]
mod flipper {
    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "flipper".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/flipper/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "basic".to_string()),
            ])
        ),
        (
            r#"
// Incrementer Contract in ink!
#[ink::contract]
mod incrementer {
    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores the current value.
        value: i32,
        /// Stores who can increment.
        allowed: ink::storage::Mapping<AccountId, bool>,
    }

    impl Incrementer {
        /// Constructor that initializes the `i32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let mut allowed = ink::storage::Mapping::default();
            let caller = Self::env().caller();
            allowed.insert(caller, &true);
            Self {
                value: init_value,
                allowed,
            }
        }

        /// Increments the stored value by 1.
        #[ink(message)]
        pub fn inc(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if !self.allowed.get(caller).unwrap_or(false) {
                return Err(Error::NotAllowed);
            }
            self.value = self.value.checked_add(1).ok_or(Error::Overflow)?;
            Ok(())
        }

        /// Returns the current value.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "incrementer".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/incrementer/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "counter".to_string()),
            ])
        ),
        (
            r#"
// ERC721 NFT Implementation in ink!
#[ink::contract]
mod erc721 {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Erc721 {
        /// Mapping from token ID to owner address
        token_owner: Mapping<TokenId, AccountId>,
        /// Mapping owner address to token count
        owned_tokens_count: Mapping<AccountId, u32>,
        /// Mapping from token ID to approved address
        token_approvals: Mapping<TokenId, AccountId>,
        /// Mapping from owner to operator approvals
        operator_approvals: Mapping<(AccountId, AccountId), ()>,
        /// Next available token ID
        next_token_id: TokenId,
    }

    impl Erc721 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                token_owner: Default::default(),
                owned_tokens_count: Default::default(),
                token_approvals: Default::default(),
                operator_approvals: Default::default(),
                next_token_id: 1,
            }
        }

        /// Mint a new token to the specified address
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId) -> Result<TokenId, Error> {
            let token_id = self.next_token_id;
            self.token_owner.insert(&token_id, &to);
            
            let count = self.owned_tokens_count.get(&to).unwrap_or(0);
            self.owned_tokens_count.insert(&to, &(count + 1));
            
            self.next_token_id += 1;
            Ok(token_id)
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "erc721".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/erc721/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "nft".to_string()),
            ])
        ),
        (
            r#"
// Cross-contract calls in ink!
#[ink::contract]
mod cross_contract_calls {
    use other_contract::OtherContractRef;

    #[ink(storage)]
    pub struct CrossContractCalls {
        /// Address of the other contract
        other_contract: OtherContractRef,
    }

    impl CrossContractCalls {
        #[ink(constructor)]
        pub fn new(other_contract_code_hash: Hash) -> Self {
            let other_contract = OtherContractRef::new()
                .code_hash(other_contract_code_hash)
                .endowment(0)
                .salt_bytes([0; 4])
                .instantiate();
            Self { other_contract }
        }

        /// Call another contract's method
        #[ink(message)]
        pub fn call_other_contract(&self) -> u32 {
            self.other_contract.get_value()
        }

        /// Call and forward any errors
        #[ink(message)]
        pub fn call_other_contract_and_forward_errors(&self) -> Result<u32, OtherError> {
            self.other_contract.get_value_or_error()
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "cross_contract_calls".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/cross-contract-calls/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "advanced".to_string()),
            ])
        ),
        (
            r#"
// Contract events in ink!
#[ink::contract]
mod events {
    /// Event emitted when a value is stored
    #[ink(event)]
    pub struct ValueStored {
        #[ink(topic)]
        by: AccountId,
        #[ink(topic)]
        value: u32,
    }

    #[ink(storage)]
    pub struct Events {
        value: u32,
    }

    impl Events {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        /// Store a value and emit an event
        #[ink(message)]
        pub fn store_value(&mut self, value: u32) {
            self.value = value;
            let caller = self.env().caller();
            self.env().emit_event(ValueStored {
                by: caller,
                value,
            });
        }

        /// Get the stored value
        #[ink(message)]
        pub fn get_value(&self) -> u32 {
            self.value
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "events".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/events/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "events".to_string()),
            ])
        ),
        (
            r#"
// Multisig contract in ink!
#[ink::contract]
mod multisig {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Multisig {
        /// Required number of confirmations
        required: u32,
        /// List of owners
        owners: Vec<AccountId>,
        /// Mapping of owner addresses
        is_owner: Mapping<AccountId, bool>,
        /// Transaction proposals
        transactions: Vec<Transaction>,
        /// Confirmations for each transaction
        confirmations: Mapping<(u32, AccountId), bool>,
    }

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Transaction {
        pub to: AccountId,
        pub value: Balance,
        pub data: Vec<u8>,
        pub executed: bool,
    }

    impl Multisig {
        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>, required: u32) -> Self {
            let mut is_owner = Mapping::default();
            for owner in &owners {
                is_owner.insert(owner, &true);
            }
            Self {
                required,
                owners,
                is_owner,
                transactions: Vec::new(),
                confirmations: Mapping::default(),
            }
        }

        /// Submit a transaction proposal
        #[ink(message)]
        pub fn submit_transaction(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> u32 {
            let transaction = Transaction {
                to,
                value,
                data,
                executed: false,
            };
            self.transactions.push(transaction);
            self.transactions.len() as u32 - 1
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "multisig".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/multisig/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "security".to_string()),
            ])
        ),
        (
            r#"
// Basic contract with storage mapping in ink!
#[ink::contract]
mod mapping {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct MappingContract {
        /// A simple mapping from AccountId to Balance
        balances: Mapping<AccountId, Balance>,
        /// Total supply of tokens
        total_supply: Balance,
    }

    impl MappingContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
                total_supply: 0,
            }
        }

        /// Set balance for an account
        #[ink(message)]
        pub fn set_balance(&mut self, account: AccountId, balance: Balance) {
            // Remove old balance from total supply
            let old_balance = self.balances.get(&account).unwrap_or(0);
            self.total_supply = self.total_supply - old_balance + balance;
            
            // Set new balance
            self.balances.insert(&account, &balance);
        }

        /// Get balance for an account
        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> Balance {
            self.balances.get(&account).unwrap_or(0)
        }

        /// Get total supply
        #[ink(message)]
        pub fn get_total_supply(&self) -> Balance {
            self.total_supply
        }
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "mapping".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/mapping/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "storage".to_string()),
            ])
        ),
        (
            r#"
// Contract termination example in ink!
#[ink::contract]
mod contract_terminate {
    #[ink(storage)]
    pub struct ContractTerminate {
        /// Value stored in the contract
        value: u32,
        /// Contract owner
        owner: AccountId,
    }

    impl ContractTerminate {
        #[ink(constructor)]
        pub fn new(value: u32) -> Self {
            Self {
                value,
                owner: Self::env().caller(),
            }
        }

        /// Get the stored value
        #[ink(message)]
        pub fn get_value(&self) -> u32 {
            self.value
        }

        /// Terminate the contract (only owner)
        #[ink(message)]
        pub fn terminate(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            self.env().terminate_contract(caller)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        NotOwner,
    }
}
"#.to_string(),
            HashMap::from([
                ("category".to_string(), "contract_terminate".to_string()),
                ("topic".to_string(), "ink_smart_contracts".to_string()),
                ("file_path".to_string(), "ink-examples/contract-terminate/lib.rs".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "lifecycle".to_string()),
            ])
        ),
        (
            "ink! is a Rust-based embedded domain specific language (eDSL) for writing smart contracts for blockchains built on the Substrate framework. It provides a familiar Rust experience with additional contract-specific functionality. Key features include: storage handling with Mapping and Vec types, message and constructor annotations, event emission, cross-contract calls, and built-in testing frameworks.".to_string(),
            HashMap::from([
                ("category".to_string(), "ink_overview".to_string()),
                ("topic".to_string(), "ink_documentation".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "documentation".to_string()),
            ])
        ),
        (
            "ink! contracts are compiled to WebAssembly (WASM) bytecode and executed on Substrate-based blockchains. Key concepts include: #[ink::contract] macro for contract definition, #[ink(storage)] for state variables, #[ink(constructor)] for initialization, #[ink(message)] for public functions, #[ink(event)] for event definitions, and cross-contract calls using ContractRef traits.".to_string(),
            HashMap::from([
                ("category".to_string(), "ink_concepts".to_string()),
                ("topic".to_string(), "ink_documentation".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "documentation".to_string()),
            ])
        ),
        (
            "Testing ink! contracts can be done using unit tests with #[ink::test] annotation and end-to-end tests with #[ink_e2e::test]. Unit tests run in an off-chain environment simulating contract execution, while e2e tests deploy and interact with contracts on a real blockchain node. Use ink::env::test utilities for setting up test environments, accounts, and balances.".to_string(),
            HashMap::from([
                ("category".to_string(), "ink_testing".to_string()),
                ("topic".to_string(), "ink_documentation".to_string()),
                ("language".to_string(), "rust".to_string()),
                ("contract_type".to_string(), "testing".to_string()),
            ])
        ),
    ];

    let mut successful_inserts = 0;
    for (text, metadata) in sample_documents {
        match rag_system.add_document(&text, metadata).await {
            Ok(_) => {
                successful_inserts += 1;
            }
            Err(e) => {
                info!("Failed to insert sample document: {}", e);
            }
        }
    }

    info!("Successfully inserted {} ink! smart contract examples into RAG system", successful_inserts);
    Ok(())
}