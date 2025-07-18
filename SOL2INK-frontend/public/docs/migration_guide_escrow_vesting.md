# Escrow & Vesting Implementation: Solidity vs ink!

## Overview
A comprehensive comparison between secure payment holding (escrow) and time-locked token release (vesting) mechanisms. This example demonstrates how to implement trustless payment systems with dispute resolution and linear token vesting with time-based releases in both blockchain platforms.

## Solidity Implementation (Escrow)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title SimpleEscrow
/// @notice A simple escrow contract for secure payments between parties
/// @dev Demonstrates payment holding, dispute resolution, and secure transfers
contract SimpleEscrow {
    // Escrow states
    enum EscrowState { Created, Funded, Completed, Disputed, Refunded }
    
    // Escrow struct
    struct Escrow {
        address payer;
        address payee;
        address arbiter;
        uint256 amount;
        EscrowState state;
        uint256 createdAt;
        uint256 deadline;
        string description;
    }
    
    // State variables
    address public owner;
    uint256 public nextEscrowId;
    uint256 public totalEscrows;
    uint256 public feePercentage; // Fee in basis points (1% = 100)
    mapping(uint256 => Escrow) public escrows;
    mapping(address => uint256[]) public userEscrows;
    mapping(address => uint256) public pendingWithdrawals;
    
    // Events
    event EscrowCreated(
        uint256 indexed escrowId,
        address indexed payer,
        address indexed payee,
        address arbiter,
        uint256 amount,
        uint256 deadline
    );
    event EscrowFunded(uint256 indexed escrowId, address indexed payer, uint256 amount);
    event EscrowCompleted(uint256 indexed escrowId, address indexed payee, uint256 amount);
    event EscrowDisputed(uint256 indexed escrowId, address indexed disputeInitiator);
    event EscrowRefunded(uint256 indexed escrowId, address indexed payer, uint256 amount);
    event DisputeResolved(uint256 indexed escrowId, address indexed winner, uint256 amount);
    event WithdrawalRequested(address indexed user, uint256 amount);
    event FeeChanged(uint256 oldFee, uint256 newFee);
    
    // Custom errors
    error EscrowNotFound(uint256 escrowId);
    error InvalidState(uint256 escrowId, EscrowState current, EscrowState required);
    error NotAuthorized(address caller);
    error InsufficientFunds(uint256 required, uint256 available);
    error DeadlineExpired(uint256 escrowId);
    error DeadlineNotExpired(uint256 escrowId);
    
    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }
    
    modifier escrowExists(uint256 escrowId) {
        if (escrows[escrowId].payer == address(0)) {
            revert EscrowNotFound(escrowId);
        }
        _;
    }
    
    modifier onlyParties(uint256 escrowId) {
        Escrow storage escrow = escrows[escrowId];
        if (msg.sender != escrow.payer && msg.sender != escrow.payee && msg.sender != escrow.arbiter) {
            revert NotAuthorized(msg.sender);
        }
        _;
    }
    
    modifier inState(uint256 escrowId, EscrowState requiredState) {
        EscrowState currentState = escrows[escrowId].state;
        if (currentState != requiredState) {
            revert InvalidState(escrowId, currentState, requiredState);
        }
        _;
    }
    
    /// @notice Constructor sets owner and fee
    /// @param _feePercentage Fee percentage in basis points (1% = 100)
    constructor(uint256 _feePercentage) {
        owner = msg.sender;
        feePercentage = _feePercentage;
        nextEscrowId = 1;
    }
    
    /// @notice Create a new escrow
    /// @param payee Address to receive payment
    /// @param arbiter Address to resolve disputes
    /// @param deadline Timestamp when escrow expires
    /// @param description Description of the escrow
    /// @return escrowId The ID of the created escrow
    function createEscrow(
        address payee,
        address arbiter,
        uint256 deadline,
        string memory description
    ) public payable returns (uint256) {
        require(payee != address(0), "Payee cannot be zero address");
        require(arbiter != address(0), "Arbiter cannot be zero address");
        require(deadline > block.timestamp, "Deadline must be in the future");
        require(msg.value > 0, "Escrow amount must be greater than 0");
        
        uint256 escrowId = nextEscrowId++;
        totalEscrows++;
        
        escrows[escrowId] = Escrow({
            payer: msg.sender,
            payee: payee,
            arbiter: arbiter,
            amount: msg.value,
            state: EscrowState.Funded,
            createdAt: block.timestamp,
            deadline: deadline,
            description: description
        });
        
        userEscrows[msg.sender].push(escrowId);
        userEscrows[payee].push(escrowId);
        
        emit EscrowCreated(escrowId, msg.sender, payee, arbiter, msg.value, deadline);
        emit EscrowFunded(escrowId, msg.sender, msg.value);
        
        return escrowId;
    }
    
    /// @notice Complete an escrow (releases payment to payee)
    /// @param escrowId The escrow ID
    function completeEscrow(uint256 escrowId) public 
        escrowExists(escrowId) 
        inState(escrowId, EscrowState.Funded) 
        onlyParties(escrowId) 
    {
        Escrow storage escrow = escrows[escrowId];
        
        // Only payer or arbiter can complete
        if (msg.sender != escrow.payer && msg.sender != escrow.arbiter) {
            revert NotAuthorized(msg.sender);
        }
        
        escrow.state = EscrowState.Completed;
        
        // Calculate fee
        uint256 fee = (escrow.amount * feePercentage) / 10000;
        uint256 payeeAmount = escrow.amount - fee;
        
        // Transfer to payee
        pendingWithdrawals[escrow.payee] += payeeAmount;
        
        // Transfer fee to owner
        if (fee > 0) {
            pendingWithdrawals[owner] += fee;
        }
        
        emit EscrowCompleted(escrowId, escrow.payee, payeeAmount);
    }
    
    /// @notice Dispute an escrow
    /// @param escrowId The escrow ID
    function disputeEscrow(uint256 escrowId) public 
        escrowExists(escrowId) 
        inState(escrowId, EscrowState.Funded) 
        onlyParties(escrowId) 
    {
        Escrow storage escrow = escrows[escrowId];
        
        // Only payer or payee can dispute
        if (msg.sender != escrow.payer && msg.sender != escrow.payee) {
            revert NotAuthorized(msg.sender);
        }
        
        escrow.state = EscrowState.Disputed;
        emit EscrowDisputed(escrowId, msg.sender);
    }
    
    /// @notice Resolve a dispute (arbiter only)
    /// @param escrowId The escrow ID
    /// @param winner Address to receive the escrowed amount
    function resolveDispute(uint256 escrowId, address winner) public 
        escrowExists(escrowId) 
        inState(escrowId, EscrowState.Disputed) 
    {
        Escrow storage escrow = escrows[escrowId];
        
        // Only arbiter can resolve
        if (msg.sender != escrow.arbiter) {
            revert NotAuthorized(msg.sender);
        }
        
        require(winner == escrow.payer || winner == escrow.payee, "Winner must be payer or payee");
        
        escrow.state = EscrowState.Completed;
        
        // No fee for disputed escrows
        pendingWithdrawals[winner] += escrow.amount;
        
        emit DisputeResolved(escrowId, winner, escrow.amount);
    }
    
    /// @notice Refund an expired escrow
    /// @param escrowId The escrow ID
    function refundExpiredEscrow(uint256 escrowId) public 
        escrowExists(escrowId) 
        inState(escrowId, EscrowState.Funded) 
    {
        Escrow storage escrow = escrows[escrowId];
        
        if (block.timestamp <= escrow.deadline) {
            revert DeadlineNotExpired(escrowId);
        }
        
        escrow.state = EscrowState.Refunded;
        pendingWithdrawals[escrow.payer] += escrow.amount;
        
        emit EscrowRefunded(escrowId, escrow.payer, escrow.amount);
    }
    
    /// @notice Withdraw pending funds
    function withdraw() public {
        uint256 amount = pendingWithdrawals[msg.sender];
        require(amount > 0, "No funds to withdraw");
        
        pendingWithdrawals[msg.sender] = 0;
        payable(msg.sender).transfer(amount);
        
        emit WithdrawalRequested(msg.sender, amount);
    }
    
    /// @notice Get escrow details
    /// @param escrowId The escrow ID
    /// @return The escrow details
    function getEscrow(uint256 escrowId) public view escrowExists(escrowId) returns (Escrow memory) {
        return escrows[escrowId];
    }
    
    /// @notice Get user's escrow IDs
    /// @param user The user address
    /// @return Array of escrow IDs
    function getUserEscrows(address user) public view returns (uint256[] memory) {
        return userEscrows[user];
    }
    
    /// @notice Get pending withdrawal amount for a user
    /// @param user The user address
    /// @return The pending withdrawal amount
    function getPendingWithdrawal(address user) public view returns (uint256) {
        return pendingWithdrawals[user];
    }
    
    /// @notice Set fee percentage (owner only)
    /// @param newFeePercentage New fee percentage in basis points
    function setFeePercentage(uint256 newFeePercentage) public onlyOwner {
        require(newFeePercentage <= 1000, "Fee cannot exceed 10%"); // Max 10%
        uint256 oldFee = feePercentage;
        feePercentage = newFeePercentage;
        emit FeeChanged(oldFee, newFeePercentage);
    }
    
    /// @notice Transfer ownership
    /// @param newOwner New owner address
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "New owner cannot be zero address");
        owner = newOwner;
    }
}
```

## ink! Implementation (Vesting + Escrow Pattern)

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod escrow_vesting {
    use ink::storage::Mapping;
    use ink::prelude::{vec::Vec, string::String};

    /// Escrow states
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum EscrowState {
        Created,
        Funded,
        Completed,
        Disputed,
        Refunded,
    }

    /// Escrow details
    #[derive(Debug, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Escrow {
        pub payer: AccountId,
        pub payee: AccountId,
        pub arbiter: AccountId,
        pub amount: Balance,
        pub state: EscrowState,
        pub created_at: Timestamp,
        pub deadline: Timestamp,
        pub description: String,
    }

    /// Vesting schedule details
    #[derive(Debug, Clone)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct VestingSchedule {
        pub beneficiary: AccountId,
        pub start_time: Timestamp,
        pub duration: Timestamp,
        pub total_amount: Balance,
        pub released_amount: Balance,
        pub revoked: bool,
    }

    /// The storage struct
    #[ink(storage)]
    pub struct EscrowVesting {
        /// Contract owner
        owner: AccountId,
        /// Escrow counter
        next_escrow_id: u64,
        /// Vesting schedule counter
        next_vesting_id: u64,
        /// Fee percentage in basis points (1% = 100)
        fee_percentage: u16,
        /// Escrow data
        escrows: Mapping<u64, Escrow>,
        /// User escrow lists
        user_escrows: Mapping<AccountId, Vec<u64>>,
        /// Pending withdrawals
        pending_withdrawals: Mapping<AccountId, Balance>,
        /// Vesting schedules
        vesting_schedules: Mapping<u64, VestingSchedule>,
        /// User vesting lists
        user_vestings: Mapping<AccountId, Vec<u64>>,
    }

    /// Events
    #[ink(event)]
    pub struct EscrowCreated {
        #[ink(topic)]
        escrow_id: u64,
        #[ink(topic)]
        payer: AccountId,
        #[ink(topic)]
        payee: AccountId,
        arbiter: AccountId,
        amount: Balance,
        deadline: Timestamp,
    }

    #[ink(event)]
    pub struct EscrowCompleted {
        #[ink(topic)]
        escrow_id: u64,
        #[ink(topic)]
        payee: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct EscrowDisputed {
        #[ink(topic)]
        escrow_id: u64,
        #[ink(topic)]
        dispute_initiator: AccountId,
    }

    #[ink(event)]
    pub struct EscrowRefunded {
        #[ink(topic)]
        escrow_id: u64,
        #[ink(topic)]
        payer: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct DisputeResolved {
        #[ink(topic)]
        escrow_id: u64,
        #[ink(topic)]
        winner: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct VestingCreated {
        #[ink(topic)]
        vesting_id: u64,
        #[ink(topic)]
        beneficiary: AccountId,
        start_time: Timestamp,
        duration: Timestamp,
        total_amount: Balance,
    }

    #[ink(event)]
    pub struct TokensReleased {
        #[ink(topic)]
        vesting_id: u64,
        #[ink(topic)]
        beneficiary: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct VestingRevoked {
        #[ink(topic)]
        vesting_id: u64,
        #[ink(topic)]
        beneficiary: AccountId,
        refund_amount: Balance,
    }

    #[ink(event)]
    pub struct WithdrawalRequested {
        #[ink(topic)]
        user: AccountId,
        amount: Balance,
    }

    /// Error types
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        EscrowNotFound,
        VestingNotFound,
        InvalidState,
        NotAuthorized,
        InsufficientFunds,
        DeadlineExpired,
        DeadlineNotExpired,
        InvalidBeneficiary,
        ZeroAmount,
        ZeroReleasableBalance,
        AlreadyRevoked,
        VestingNotStarted,
        ArithmeticOverflow,
    }

    /// Result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl EscrowVesting {
        /// Constructor
        #[ink(constructor)]
        pub fn new(fee_percentage: u16) -> Self {
            let owner = Self::env().caller();
            Self {
                owner,
                next_escrow_id: 1,
                next_vesting_id: 1,
                fee_percentage,
                escrows: Mapping::default(),
                user_escrows: Mapping::default(),
                pending_withdrawals: Mapping::default(),
                vesting_schedules: Mapping::default(),
                user_vestings: Mapping::default(),
            }
        }

        /// Create a new escrow
        #[ink(message, payable)]
        pub fn create_escrow(
            &mut self,
            payee: AccountId,
            arbiter: AccountId,
            deadline: Timestamp,
            description: String,
        ) -> Result<u64> {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            let now = self.env().block_timestamp();

            if payee == AccountId::from([0u8; 32]) || arbiter == AccountId::from([0u8; 32]) {
                return Err(Error::InvalidBeneficiary);
            }

            if deadline <= now {
                return Err(Error::DeadlineExpired);
            }

            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let escrow_id = self.next_escrow_id;
            self.next_escrow_id = self.next_escrow_id.checked_add(1).ok_or(Error::ArithmeticOverflow)?;

            let escrow = Escrow {
                payer: caller,
                payee,
                arbiter,
                amount,
                state: EscrowState::Funded,
                created_at: now,
                deadline,
                description,
            };

            self.escrows.insert(escrow_id, &escrow);

            // Add to user escrow lists
            let mut payer_escrows = self.user_escrows.get(caller).unwrap_or_default();
            payer_escrows.push(escrow_id);
            self.user_escrows.insert(caller, &payer_escrows);

            let mut payee_escrows = self.user_escrows.get(payee).unwrap_or_default();
            payee_escrows.push(escrow_id);
            self.user_escrows.insert(payee, &payee_escrows);

            self.env().emit_event(EscrowCreated {
                escrow_id,
                payer: caller,
                payee,
                arbiter,
                amount,
                deadline,
            });

            Ok(escrow_id)
        }

        /// Complete an escrow
        #[ink(message)]
        pub fn complete_escrow(&mut self, escrow_id: u64) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(escrow_id).ok_or(Error::EscrowNotFound)?;

            if escrow.state != EscrowState::Funded {
                return Err(Error::InvalidState);
            }

            if caller != escrow.payer && caller != escrow.arbiter {
                return Err(Error::NotAuthorized);
            }

            escrow.state = EscrowState::Completed;
            self.escrows.insert(escrow_id, &escrow);

            // Calculate fee
            let fee = (escrow.amount * self.fee_percentage as u128) / 10000;
            let payee_amount = escrow.amount - fee;

            // Add to pending withdrawals
            let current_pending = self.pending_withdrawals.get(escrow.payee).unwrap_or(0);
            self.pending_withdrawals.insert(escrow.payee, &(current_pending + payee_amount));

            if fee > 0 {
                let owner_pending = self.pending_withdrawals.get(self.owner).unwrap_or(0);
                self.pending_withdrawals.insert(self.owner, &(owner_pending + fee));
            }

            self.env().emit_event(EscrowCompleted {
                escrow_id,
                payee: escrow.payee,
                amount: payee_amount,
            });

            Ok(())
        }

        /// Dispute an escrow
        #[ink(message)]
        pub fn dispute_escrow(&mut self, escrow_id: u64) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(escrow_id).ok_or(Error::EscrowNotFound)?;

            if escrow.state != EscrowState::Funded {
                return Err(Error::InvalidState);
            }

            if caller != escrow.payer && caller != escrow.payee {
                return Err(Error::NotAuthorized);
            }

            escrow.state = EscrowState::Disputed;
            self.escrows.insert(escrow_id, &escrow);

            self.env().emit_event(EscrowDisputed {
                escrow_id,
                dispute_initiator: caller,
            });

            Ok(())
        }

        /// Resolve a dispute
        #[ink(message)]
        pub fn resolve_dispute(&mut self, escrow_id: u64, winner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            let mut escrow = self.escrows.get(escrow_id).ok_or(Error::EscrowNotFound)?;

            if escrow.state != EscrowState::Disputed {
                return Err(Error::InvalidState);
            }

            if caller != escrow.arbiter {
                return Err(Error::NotAuthorized);
            }

            if winner != escrow.payer && winner != escrow.payee {
                return Err(Error::InvalidBeneficiary);
            }

            escrow.state = EscrowState::Completed;
            self.escrows.insert(escrow_id, &escrow);

            // No fee for disputed escrows
            let current_pending = self.pending_withdrawals.get(winner).unwrap_or(0);
            self.pending_withdrawals.insert(winner, &(current_pending + escrow.amount));

            self.env().emit_event(DisputeResolved {
                escrow_id,
                winner,
                amount: escrow.amount,
            });

            Ok(())
        }

        /// Refund an expired escrow
        #[ink(message)]
        pub fn refund_expired_escrow(&mut self, escrow_id: u64) -> Result<()> {
            let now = self.env().block_timestamp();
            let mut escrow = self.escrows.get(escrow_id).ok_or(Error::EscrowNotFound)?;

            if escrow.state != EscrowState::Funded {
                return Err(Error::InvalidState);
            }

            if now <= escrow.deadline {
                return Err(Error::DeadlineNotExpired);
            }

            escrow.state = EscrowState::Refunded;
            self.escrows.insert(escrow_id, &escrow);

            let current_pending = self.pending_withdrawals.get(escrow.payer).unwrap_or(0);
            self.pending_withdrawals.insert(escrow.payer, &(current_pending + escrow.amount));

            self.env().emit_event(EscrowRefunded {
                escrow_id,
                payer: escrow.payer,
                amount: escrow.amount,
            });

            Ok(())
        }

        /// Create a vesting schedule
        #[ink(message, payable)]
        pub fn create_vesting(
            &mut self,
            beneficiary: AccountId,
            duration_seconds: u64,
        ) -> Result<u64> {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            let now = self.env().block_timestamp();

            if beneficiary == AccountId::from([0u8; 32]) {
                return Err(Error::InvalidBeneficiary);
            }

            if amount == 0 {
                return Err(Error::ZeroAmount);
            }

            let vesting_id = self.next_vesting_id;
            self.next_vesting_id = self.next_vesting_id.checked_add(1).ok_or(Error::ArithmeticOverflow)?;

            let duration = (duration_seconds as u64).checked_mul(1000).ok_or(Error::ArithmeticOverflow)?;

            let vesting = VestingSchedule {
                beneficiary,
                start_time: now,
                duration,
                total_amount: amount,
                released_amount: 0,
                revoked: false,
            };

            self.vesting_schedules.insert(vesting_id, &vesting);

            // Add to user vesting list
            let mut user_vestings = self.user_vestings.get(beneficiary).unwrap_or_default();
            user_vestings.push(vesting_id);
            self.user_vestings.insert(beneficiary, &user_vestings);

            self.env().emit_event(VestingCreated {
                vesting_id,
                beneficiary,
                start_time: now,
                duration,
                total_amount: amount,
            });

            Ok(vesting_id)
        }

        /// Release vested tokens
        #[ink(message)]
        pub fn release_vested_tokens(&mut self, vesting_id: u64) -> Result<()> {
            let now = self.env().block_timestamp();
            let mut vesting = self.vesting_schedules.get(vesting_id).ok_or(Error::VestingNotFound)?;

            if vesting.revoked {
                return Err(Error::AlreadyRevoked);
            }

            let releasable = self.calculate_releasable_amount(&vesting, now)?;
            if releasable == 0 {
                return Err(Error::ZeroReleasableBalance);
            }

            vesting.released_amount += releasable;
            self.vesting_schedules.insert(vesting_id, &vesting);

            // Transfer tokens to beneficiary
            let current_pending = self.pending_withdrawals.get(vesting.beneficiary).unwrap_or(0);
            self.pending_withdrawals.insert(vesting.beneficiary, &(current_pending + releasable));

            self.env().emit_event(TokensReleased {
                vesting_id,
                beneficiary: vesting.beneficiary,
                amount: releasable,
            });

            Ok(())
        }

        /// Revoke a vesting schedule (owner only)
        #[ink(message)]
        pub fn revoke_vesting(&mut self, vesting_id: u64) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotAuthorized);
            }

            let now = self.env().block_timestamp();
            let mut vesting = self.vesting_schedules.get(vesting_id).ok_or(Error::VestingNotFound)?;

            if vesting.revoked {
                return Err(Error::AlreadyRevoked);
            }

            // Release any vested tokens first
            let releasable = self.calculate_releasable_amount(&vesting, now)?;
            if releasable > 0 {
                vesting.released_amount += releasable;
                let current_pending = self.pending_withdrawals.get(vesting.beneficiary).unwrap_or(0);
                self.pending_withdrawals.insert(vesting.beneficiary, &(current_pending + releasable));
            }

            // Calculate refund amount
            let refund_amount = vesting.total_amount - vesting.released_amount;

            vesting.revoked = true;
            self.vesting_schedules.insert(vesting_id, &vesting);

            // Refund remaining tokens to owner
            if refund_amount > 0 {
                let owner_pending = self.pending_withdrawals.get(self.owner).unwrap_or(0);
                self.pending_withdrawals.insert(self.owner, &(owner_pending + refund_amount));
            }

            self.env().emit_event(VestingRevoked {
                vesting_id,
                beneficiary: vesting.beneficiary,
                refund_amount,
            });

            Ok(())
        }

        /// Withdraw pending funds
        #[ink(message)]
        pub fn withdraw(&mut self) -> Result<()> {
            let caller = self.env().caller();
            let amount = self.pending_withdrawals.get(caller).unwrap_or(0);

            if amount == 0 {
                return Err(Error::InsufficientFunds);
            }

            self.pending_withdrawals.remove(caller);

            self.env().transfer(caller, amount).map_err(|_| Error::InsufficientFunds)?;

            self.env().emit_event(WithdrawalRequested {
                user: caller,
                amount,
            });

            Ok(())
        }

        /// Calculate releasable amount for a vesting schedule
        fn calculate_releasable_amount(&self, vesting: &VestingSchedule, current_time: Timestamp) -> Result<Balance> {
            let vested_amount = self.calculate_vested_amount(vesting, current_time)?;
            Ok(vested_amount.saturating_sub(vesting.released_amount))
        }

        /// Calculate vested amount for a vesting schedule
        fn calculate_vested_amount(&self, vesting: &VestingSchedule, current_time: Timestamp) -> Result<Balance> {
            if current_time < vesting.start_time {
                return Ok(0);
            }

            let end_time = vesting.start_time.checked_add(vesting.duration).ok_or(Error::ArithmeticOverflow)?;
            
            if current_time >= end_time {
                return Ok(vesting.total_amount);
            }

            let elapsed = current_time.saturating_sub(vesting.start_time);
            let vested = vesting.total_amount
                .checked_mul(elapsed as u128)
                .ok_or(Error::ArithmeticOverflow)?
                .checked_div(vesting.duration as u128)
                .ok_or(Error::ArithmeticOverflow)?;

            Ok(vested)
        }

        /// Get escrow details
        #[ink(message)]
        pub fn get_escrow(&self, escrow_id: u64) -> Option<Escrow> {
            self.escrows.get(escrow_id)
        }

        /// Get vesting schedule details
        #[ink(message)]
        pub fn get_vesting_schedule(&self, vesting_id: u64) -> Option<VestingSchedule> {
            self.vesting_schedules.get(vesting_id)
        }

        /// Get pending withdrawal amount
        #[ink(message)]
        pub fn get_pending_withdrawal(&self, user: AccountId) -> Balance {
            self.pending_withdrawals.get(user).unwrap_or(0)
        }

        /// Get releasable amount for a vesting schedule
        #[ink(message)]
        pub fn get_releasable_amount(&self, vesting_id: u64) -> Result<Balance> {
            let now = self.env().block_timestamp();
            let vesting = self.vesting_schedules.get(vesting_id).ok_or(Error::VestingNotFound)?;
            self.calculate_releasable_amount(&vesting, now)
        }

        /// Get vested amount for a vesting schedule
        #[ink(message)]
        pub fn get_vested_amount(&self, vesting_id: u64) -> Result<Balance> {
            let now = self.env().block_timestamp();
            let vesting = self.vesting_schedules.get(vesting_id).ok_or(Error::VestingNotFound)?;
            self.calculate_vested_amount(&vesting, now)
        }

        /// Get user's escrows
        #[ink(message)]
        pub fn get_user_escrows(&self, user: AccountId) -> Vec<u64> {
            self.user_escrows.get(user).unwrap_or_default()
        }

        /// Get user's vesting schedules
        #[ink(message)]
        pub fn get_user_vestings(&self, user: AccountId) -> Vec<u64> {
            self.user_vestings.get(user).unwrap_or_default()
        }

        /// Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Get fee percentage
        #[ink(message)]
        pub fn get_fee_percentage(&self) -> u16 {
            self.fee_percentage
        }

        /// Set fee percentage (owner only)
        #[ink(message)]
        pub fn set_fee_percentage(&mut self, new_fee_percentage: u16) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotAuthorized);
            }

            if new_fee_percentage > 1000 {
                return Err(Error::InvalidState);
            }

            self.fee_percentage = new_fee_percentage;
            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = EscrowVesting::new(250); // 2.5% fee
            assert_eq!(contract.get_fee_percentage(), 250);
        }

        #[ink::test]
        fn create_escrow_works() {
            let mut contract = EscrowVesting::new(250);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Set transferred value
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            
            let result = contract.create_escrow(
                accounts.bob,
                accounts.charlie,
                1000000, // Future deadline
                "Test escrow".to_string()
            );
            
            assert!(result.is_ok());
            let escrow_id = result.unwrap();
            assert_eq!(escrow_id, 1);
            
            let escrow = contract.get_escrow(escrow_id).unwrap();
            assert_eq!(escrow.amount, 1000);
            assert_eq!(escrow.payee, accounts.bob);
            assert_eq!(escrow.state, EscrowState::Funded);
        }

        #[ink::test]
        fn complete_escrow_works() {
            let mut contract = EscrowVesting::new(250);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Set transferred value and create escrow
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            let escrow_id = contract.create_escrow(
                accounts.bob,
                accounts.charlie,
                1000000,
                "Test escrow".to_string()
            ).unwrap();
            
            // Complete escrow
            assert_eq!(contract.complete_escrow(escrow_id), Ok(()));
            
            let escrow = contract.get_escrow(escrow_id).unwrap();
            assert_eq!(escrow.state, EscrowState::Completed);
            
            // Check pending withdrawals (amount - fee)
            let fee = (1000 * 250) / 10000; // 2.5% fee
            let expected_amount = 1000 - fee;
            assert_eq!(contract.get_pending_withdrawal(accounts.bob), expected_amount);
        }

        #[ink::test]
        fn create_vesting_works() {
            let mut contract = EscrowVesting::new(250);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Set transferred value
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            
            let result = contract.create_vesting(accounts.bob, 200); // 200 seconds
            assert!(result.is_ok());
            
            let vesting_id = result.unwrap();
            let vesting = contract.get_vesting_schedule(vesting_id).unwrap();
            assert_eq!(vesting.beneficiary, accounts.bob);
            assert_eq!(vesting.total_amount, 1000);
            assert_eq!(vesting.duration, 200 * 1000); // Converted to milliseconds
        }

        #[ink::test]
        fn release_vested_tokens_works() {
            let mut contract = EscrowVesting::new(250);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Set transferred value and create vesting
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            let vesting_id = contract.create_vesting(accounts.bob, 0).unwrap(); // 0 duration = immediate vest
            
            // Release vested tokens
            assert_eq!(contract.release_vested_tokens(vesting_id), Ok(()));
            
            // Check pending withdrawals
            assert_eq!(contract.get_pending_withdrawal(accounts.bob), 1000);
        }

        #[ink::test]
        fn dispute_and_resolve_works() {
            let mut contract = EscrowVesting::new(250);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            // Create escrow
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
            let escrow_id = contract.create_escrow(
                accounts.bob,
                accounts.charlie,
                1000000,
                "Test escrow".to_string()
            ).unwrap();
            
            // Dispute escrow
            assert_eq!(contract.dispute_escrow(escrow_id), Ok(()));
            
            let escrow = contract.get_escrow(escrow_id).unwrap();
            assert_eq!(escrow.state, EscrowState::Disputed);
            
            // Resolve dispute (arbiter decides)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            assert_eq!(contract.resolve_dispute(escrow_id, accounts.bob), Ok(()));
            
            // Check that bob gets the full amount (no fee on disputes)
            assert_eq!(contract.get_pending_withdrawal(accounts.bob), 1000);
        }
    }
}
```

## Key Migration Points

### 1. State Machine Management
**Solidity:**
```solidity
enum EscrowState { Created, Funded, Completed, Disputed, Refunded }

modifier inState(uint256 escrowId, EscrowState requiredState) {
    EscrowState currentState = escrows[escrowId].state;
    if (currentState != requiredState) {
        revert InvalidState(escrowId, currentState, requiredState);
    }
    _;
}
```

**ink!:**
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum EscrowState {
    Created,
    Funded,
    Completed,
    Disputed,
    Refunded,
}

if escrow.state != EscrowState::Funded {
    return Err(Error::InvalidState);
}
```

### 2. Time-based Operations
**Solidity:**
```solidity
require(deadline > block.timestamp, "Deadline must be in the future");
if (block.timestamp <= escrow.deadline) {
    revert DeadlineNotExpired(escrowId);
}
```

**ink!:**
```rust
let now = self.env().block_timestamp();
if deadline <= now {
    return Err(Error::DeadlineExpired);
}
if now <= escrow.deadline {
    return Err(Error::DeadlineNotExpired);
}
```

### 3. Complex Data Structures
**Solidity:**
```solidity
struct Escrow {
    address payer;
    address payee;
    address arbiter;
    uint256 amount;
    EscrowState state;
    uint256 createdAt;
    uint256 deadline;
    string description;
}
```

**ink!:**
```rust
#[derive(Debug, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct Escrow {
    pub payer: AccountId,
    pub payee: AccountId,
    pub arbiter: AccountId,
    pub amount: Balance,
    pub state: EscrowState,
    pub created_at: Timestamp,
    pub deadline: Timestamp,
    pub description: String,
}
```

### 4. Vesting Calculations
**Solidity:**
```solidity
function vesting_schedule(
    uint256 total_allocation,
    uint256 current_time
) internal view returns (uint256) {
    if (current_time < start_time) {
        return 0;
    } else if (current_time >= end_time) {
        return total_allocation;
    } else {
        return (total_allocation * (current_time - start_time)) / duration_time;
    }
}
```

**ink!:**
```rust
fn calculate_vested_amount(&self, vesting: &VestingSchedule, current_time: Timestamp) -> Result<Balance> {
    if current_time < vesting.start_time {
        return Ok(0);
    }
    
    let end_time = vesting.start_time.checked_add(vesting.duration).ok_or(Error::ArithmeticOverflow)?;
    
    if current_time >= end_time {
        return Ok(vesting.total_amount);
    }
    
    let elapsed = current_time.saturating_sub(vesting.start_time);
    let vested = vesting.total_amount
        .checked_mul(elapsed as u128)
        .ok_or(Error::ArithmeticOverflow)?
        .checked_div(vesting.duration as u128)
        .ok_or(Error::ArithmeticOverflow)?;
    
    Ok(vested)
}
```

### 5. Payable Functions
**Solidity:**
```solidity
function createEscrow(
    address payee,
    address arbiter,
    uint256 deadline,
    string memory description
) public payable returns (uint256) {
    require(msg.value > 0, "Escrow amount must be greater than 0");
    // Use msg.value for amount
}
```

**ink!:**
```rust
#[ink(message, payable)]
pub fn create_escrow(
    &mut self,
    payee: AccountId,
    arbiter: AccountId,
    deadline: Timestamp,
    description: String,
) -> Result<u64> {
    let amount = self.env().transferred_value();
    if amount == 0 {
        return Err(Error::ZeroAmount);
    }
    // Use transferred value for amount
}
```

## Migration Steps

### Step 1: Convert Data Structures
1. Replace `struct` with `#[derive]` annotations
2. Use `AccountId` instead of `address`
3. Use `Balance` instead of `uint256`
4. Use `Timestamp` for time values
5. Add proper serialization traits

### Step 2: Convert State Management
1. Replace `enum` with `#[ink::scale_derive]` annotations
2. Use explicit state checks instead of modifiers
3. Return `Result<T, Error>` for state validation
4. Handle state transitions explicitly

### Step 3: Convert Time Operations
1. Use `self.env().block_timestamp()` instead of `block.timestamp`
2. Handle time arithmetic with overflow checks
3. Use `Timestamp` type consistently
4. Convert seconds to milliseconds for ink! timestamps

### Step 4: Convert Payment Handling
1. Use `#[ink(message, payable)]` for functions that receive payments
2. Use `self.env().transferred_value()` instead of `msg.value`
3. Use `self.env().transfer()` for sending payments
4. Handle transfer failures explicitly

### Step 5: Convert Storage Operations
1. Use `Mapping<K, V>` for key-value storage
2. Handle `Option` types from mapping access
3. Use `Vec<T>` for array storage
4. Implement proper default values

### Step 6: Add Error Handling
1. Define comprehensive error types
2. Use `Result<T, Error>` for fallible operations
3. Handle arithmetic overflow explicitly
4. Use `?` operator for error propagation

## Common Patterns

### Multi-Party Authorization
**Solidity:**
```solidity
modifier onlyParties(uint256 escrowId) {
    Escrow storage escrow = escrows[escrowId];
    if (msg.sender != escrow.payer && msg.sender != escrow.payee && msg.sender != escrow.arbiter) {
        revert NotAuthorized(msg.sender);
    }
    _;
}
```

**ink!:**
```rust
fn check_authorization(&self, escrow: &Escrow, caller: AccountId) -> Result<()> {
    if caller != escrow.payer && caller != escrow.payee && caller != escrow.arbiter {
        return Err(Error::NotAuthorized);
    }
    Ok(())
}
```

### Fee Calculations
**Solidity:**
```solidity
uint256 fee = (escrow.amount * feePercentage) / 10000;
uint256 payeeAmount = escrow.amount - fee;
```

**ink!:**
```rust
let fee = (escrow.amount * self.fee_percentage as u128) / 10000;
let payee_amount = escrow.amount - fee;
```

### Pending Withdrawals Pattern
**Solidity:**
```solidity
mapping(address => uint256) public pendingWithdrawals;

function withdraw() public {
    uint256 amount = pendingWithdrawals[msg.sender];
    require(amount > 0, "No funds to withdraw");
    
    pendingWithdrawals[msg.sender] = 0;
    payable(msg.sender).transfer(amount);
}
```

**ink!:**
```rust
pending_withdrawals: Mapping<AccountId, Balance>,

#[ink(message)]
pub fn withdraw(&mut self) -> Result<()> {
    let caller = self.env().caller();
    let amount = self.pending_withdrawals.get(caller).unwrap_or(0);
    
    if amount == 0 {
        return Err(Error::InsufficientFunds);
    }
    
    self.pending_withdrawals.remove(caller);
    self.env().transfer(caller, amount).map_err(|_| Error::InsufficientFunds)?;
    
    Ok(())
}
```

### Linear Vesting Schedule
**Solidity:**
```solidity
function vesting_schedule(
    uint256 total_allocation,
    uint256 current_time
) internal view returns (uint256) {
    if (current_time < start_time) {
        return 0;
    } else if (current_time >= end_time) {
        return total_allocation;
    } else {
        return (total_allocation * (current_time - start_time)) / duration_time;
    }
}
```

**ink!:**
```rust
fn calculate_vested_amount(&self, vesting: &VestingSchedule, current_time: Timestamp) -> Result<Balance> {
    if current_time < vesting.start_time {
        return Ok(0);
    }
    
    let end_time = vesting.start_time.checked_add(vesting.duration).ok_or(Error::ArithmeticOverflow)?;
    
    if current_time >= end_time {
        return Ok(vesting.total_amount);
    }
    
    let elapsed = current_time.saturating_sub(vesting.start_time);
    let vested = vesting.total_amount
        .checked_mul(elapsed as u128)
        .ok_or(Error::ArithmeticOverflow)?
        .checked_div(vesting.duration as u128)
        .ok_or(Error::ArithmeticOverflow)?;
    
    Ok(vested)
}
```

## Best Practices

### 1. State Management
- Use explicit state validation instead of modifiers
- Return descriptive error types for invalid states
- Handle state transitions atomically
- Use enum variants for complex state machines

### 2. Time Operations
- Always use checked arithmetic for time calculations
- Handle time overflow and underflow explicitly
- Use consistent time units (milliseconds in ink!)
- Cache timestamps to avoid multiple calls

### 3. Financial Operations
- Use pending withdrawals pattern for security
- Always validate amounts before processing
- Handle fee calculations with proper precision
- Use checked arithmetic for all financial operations

### 4. Access Control
- Implement role-based access control explicitly
- Use descriptive error messages for unauthorized access
- Validate caller permissions before state changes
- Consider multi-signature patterns for critical operations

### 5. Data Structures
- Use proper serialization traits for complex structures
- Handle `Option` types from storage access
- Use `Vec<T>` for dynamic arrays
- Consider storage costs for large data structures

This migration demonstrates how Solidity's escrow and vesting patterns translate to ink! with better type safety, explicit error handling, and more robust time-based operations while maintaining the security guarantees of the original patterns.