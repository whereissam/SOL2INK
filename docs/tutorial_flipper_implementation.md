# How to Implement Flipper in ink! - Developer Tutorial

## 📋 Overview

This tutorial will guide you through implementing a Flipper contract in ink!, step by step. The Flipper contract is a simple boolean toggle that demonstrates fundamental ink! concepts like storage, constructors, and message functions.

## 🎯 What You'll Learn

- How to set up ink! storage with `#[ink(storage)]`
- How to create constructors with `#[ink(constructor)]`
- How to implement message functions with `#[ink(message)]`
- How to handle mutable and immutable references
- How to write unit tests for ink! contracts
- How to write E2E tests for ink! contracts

## 🚀 Prerequisites

- Basic Rust knowledge
- `cargo-contract` installed
- `substrate-contracts-node` running (for E2E tests)

## 📁 Project Setup

1. **Create a new ink! project:**
   ```bash
   cargo contract new flipper
   cd flipper
   ```

2. **Update Cargo.toml:**
   ```toml
   [package]
   name = "flipper"
   version = "0.1.0"
   edition = "2021"
   
   [dependencies]
   ink = { version = "5.0.0", default-features = false }
   
   [dev-dependencies]
   ink_e2e = "5.0.0"
   
   [lib]
   path = "lib.rs"
   crate-type = ["cdylib"]
   
   [features]
   default = ["std"]
   std = [
       "ink/std",
   ]
   ink-as-dependency = []
   e2e-tests = []
   ```

## 🏗️ Step-by-Step Implementation

### Step 1: Define the Contract Module

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod flipper {
    // Contract implementation will go here
}
```

**Key Points:**
- `#![cfg_attr(not(feature = "std"), no_std, no_main)]` - Ensures no_std compilation for WebAssembly
- `#[ink::contract]` - Marks this module as an ink! contract
- `pub mod flipper` - Makes the contract module public

### Step 2: Define Storage Structure

```rust
#[ink::contract]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }
}
```

**Key Points:**
- `#[ink(storage)]` - Marks this struct as contract storage
- `value: bool` - Simple boolean field to store the flip state
- Storage is automatically persistent across contract calls

### Step 3: Implement Constructors

```rust
impl Flipper {
    /// Creates a new flipper smart contract initialized with the given value.
    #[ink(constructor)]
    pub fn new(init_value: bool) -> Self {
        Self { value: init_value }
    }

    /// Creates a new flipper smart contract initialized to `false`.
    #[ink(constructor)]
    pub fn new_default() -> Self {
        Self::new(Default::default())
    }
}
```

**Key Points:**
- `#[ink(constructor)]` - Marks functions as contract constructors
- Multiple constructors are allowed
- Constructors must return `Self`
- `Default::default()` for `bool` is `false`

### Step 4: Implement Message Functions

```rust
impl Flipper {
    // ... constructors ...

    /// Flips the current value of the Flipper's boolean.
    #[ink(message)]
    pub fn flip(&mut self) {
        self.value = !self.value;
    }

    /// Returns the current value of the Flipper's boolean.
    #[ink(message)]
    pub fn get(&self) -> bool {
        self.value
    }
}
```

**Key Points:**
- `#[ink(message)]` - Marks functions as public contract messages
- `&mut self` - For functions that modify state
- `&self` - For read-only functions
- Return types are automatically handled by ink!

### Step 5: Add Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn default_works() {
        let flipper = Flipper::new_default();
        assert!(!flipper.get());
    }

    #[ink::test]
    fn it_works() {
        let mut flipper = Flipper::new(false);
        assert!(!flipper.get());
        flipper.flip();
        assert!(flipper.get());
    }
}
```

**Key Points:**
- `#[cfg(test)]` - Conditional compilation for tests
- `#[ink::test]` - ink! unit test attribute
- Tests run in simulated environment
- Use `assert!` macros for testing

### Step 6: Add E2E Tests

```rust
#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::ContractsBackend;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        // given
        let mut constructor = FlipperRef::new(false);
        let contract = client
            .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<Flipper>();

        let get = call_builder.get();
        let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_res.return_value(), false));

        // when
        let flip = call_builder.flip();
        let _flip_res = client
            .call(&ink_e2e::bob(), &flip)
            .submit()
            .await
            .expect("flip failed");

        // then
        let get = call_builder.get();
        let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
        assert!(matches!(get_res.return_value(), true));

        Ok(())
    }
}
```

**Key Points:**
- `#[cfg(all(test, feature = "e2e-tests"))]` - E2E tests only compile with feature flag
- `#[ink_e2e::test]` - E2E test attribute
- Tests run against real blockchain node
- Use `dry_run()` for read-only calls, `submit()` for transactions

## 🧪 Testing Your Contract

### Run Unit Tests
```bash
cargo test
```

### Run E2E Tests
```bash
# Start substrate-contracts-node first
substrate-contracts-node --dev

# In another terminal
cargo test --features e2e-tests
```

## 🔨 Building and Deploying

### Build the Contract
```bash
cargo contract build
```

### Deploy to Local Node
```bash
cargo contract instantiate --constructor new --args false --suri //Alice --salt $(date +%s)
```

### Interact with Contract
```bash
# Call flip function
cargo contract call --contract YOUR_CONTRACT_ADDRESS --message flip --suri //Alice

# Query get function
cargo contract call --contract YOUR_CONTRACT_ADDRESS --message get --suri //Alice --dry-run
```

## 🎨 Complete Implementation

Here's the complete `lib.rs` file:

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod flipper {
    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::new_default();
            assert!(!flipper.get());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert!(!flipper.get());
            flipper.flip();
            assert!(flipper.get());
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn it_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new(false);
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), false));

            // when
            let flip = call_builder.flip();
            let _flip_res = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // then
            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), true));

            Ok(())
        }

        #[ink_e2e::test]
        async fn default_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
            // given
            let mut constructor = FlipperRef::new_default();

            // when
            let contract = client
                .instantiate("flipper", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<Flipper>();

            // then
            let get = call_builder.get();
            let get_res = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_res.return_value(), false));

            Ok(())
        }
    }
}
```

## 🚀 Next Steps

1. **Add Events**: Learn how to emit events with `#[ink(event)]`
2. **Add Error Handling**: Implement custom error types with `Result<T, E>`
3. **Add Access Control**: Implement ownership patterns
4. **Add More Storage**: Use `Mapping` for complex data structures
5. **Upgrade to Advanced Features**: Chain extensions, cross-contract calls

## 🔧 Common Issues and Solutions

### Issue: "cannot find attribute `ink` in this scope"
**Solution**: Make sure you have the correct ink! dependency in `Cargo.toml`

### Issue: E2E tests failing
**Solution**: Ensure `substrate-contracts-node` is running and accessible

### Issue: Contract compilation errors
**Solution**: Check that all ink! attributes are correctly applied and types are compatible

## 📚 Additional Resources

- [ink! Documentation](https://use.ink/)
- [ink! Examples](https://github.com/paritytech/ink-examples)
- [Substrate Contracts Node](https://github.com/paritytech/substrate-contracts-node)
- [cargo-contract](https://github.com/paritytech/cargo-contract)

## 🎯 Key Takeaways

1. **ink! uses attributes** to mark different parts of your contract
2. **Storage is persistent** and automatically managed
3. **Multiple constructors** are allowed and useful
4. **Testing is built-in** with both unit and E2E test support
5. **Rust's type system** provides safety and efficiency benefits

Happy coding with ink! 🦀