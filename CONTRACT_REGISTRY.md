# Contract Registry

Complete reference of all contracts in the SoroSim collection, including their entry points, functions, and usage examples.

## Table of Contents

- [Counter](#counter)
- [Token](#token)
- [NFT](#nft)
- [Voting](#voting)
- [Escrow](#escrow)
- [Multisig](#multisig)
- [Oracle](#oracle)
- [AMM](#amm)
- [Auth](#auth)
- [Events](#events)
- [Cross-Contract (Caller/Callee)](#cross-contract)
- [Storage](#storage)

---

## Counter

**Path:** `contracts/counter/src/lib.rs`  
**WASM:** `sorosim_counter.wasm`  
**Contract:** `CounterContract`

### Description
Simple counter demonstrating basic state management with increment, decrement, and query operations.

### Entry Points

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `increment` | `env: Env` | `i32` | Increments counter by 1 |
| `decrement` | `env: Env` | `i32` | Decrements counter by 1 |
| `get` | `env: Env` | `i32` | Returns current counter value |

### Storage
- **Type:** Instance Storage
- **Key:** `COUNTER` (Symbol)
- **Value:** `i32`

### Example Usage
```rust
let client = CounterContractClient::new(&env, &contract_id);
client.increment(); // Returns 1
client.increment(); // Returns 2
client.get();       // Returns 2
client.decrement(); // Returns 1
```

---

## Token

**Path:** `contracts/token/src/lib.rs`  
**WASM:** `sorosim_token.wasm`  
**Contract:** `TokenContract`

### Description
Fungible token with mint, transfer, approve, and transferFrom functionality (ERC-20-like).

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `mint` | `env: Env, to: Address, amount: i128` | `()` | Recipient | Mints tokens to address |
| `transfer` | `env: Env, from: Address, to: Address, amount: i128` | `()` | From | Transfers tokens |
| `balance` | `env: Env, account: Address` | `i128` | None | Gets token balance |
| `approve` | `env: Env, owner: Address, spender: Address, amount: i128` | `()` | Owner | Approves allowance |
| `allowance` | `env: Env, owner: Address, spender: Address` | `i128` | None | Gets allowance amount |
| `transfer_from` | `env: Env, spender: Address, from: Address, to: Address, amount: i128` | `()` | Spender | Transfers using allowance |

### Storage
- **Balances:** Persistent Storage (Address → i128)
- **Allowances:** Persistent Storage (AllowanceKey → i128)

### Data Structures
```rust
struct AllowanceKey {
    owner: Address,
    spender: Address,
}
```

### Example Usage
```rust
let client = TokenContractClient::new(&env, &contract_id);

// Mint tokens
client.mint(&user, &1000);

// Transfer
client.transfer(&alice, &bob, &100);

// Approve and transferFrom
client.approve(&owner, &spender, &500);
client.transfer_from(&spender, &owner, &recipient, &100);
```

---

## NFT

**Path:** `contracts/nft/src/lib.rs`  
**WASM:** `sorosim_nft.wasm`  
**Contract:** `NftContract`

### Description
Non-fungible token contract with sequential token IDs and ownership tracking.

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `mint` | `env: Env, to: Address` | `u64` | Recipient | Mints new NFT, returns token ID |
| `transfer` | `env: Env, token_id: u64, from: Address, to: Address` | `()` | From | Transfers NFT |
| `owner_of` | `env: Env, token_id: u64` | `Address` | None | Returns NFT owner |
| `total_supply` | `env: Env` | `u64` | None | Returns total NFTs minted |

### Storage
- **Counter:** Instance Storage (u64)
- **Ownership:** Persistent Storage (DataKey::Owner(u64) → Address)

### Example Usage
```rust
let client = NftContractClient::new(&env, &contract_id);

// Mint NFT
let token_id = client.mint(&user); // Returns 0 (first NFT)

// Transfer NFT
client.transfer(&token_id, &alice, &bob);

// Query owner
let owner = client.owner_of(&token_id);
```

---

## Voting

**Path:** `contracts/voting/src/lib.rs`  
**WASM:** `sorosim_voting.wasm`  
**Contract:** `VotingContract`

### Description
Proposal voting system with time-based deadlines and finalization.

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `create_proposal` | `env: Env, creator: Address, description: String, deadline: u64` | `u64` | Creator | Creates new proposal |
| `vote` | `env: Env, proposal_id: u64, voter: Address, in_favor: bool` | `()` | Voter | Casts a vote |
| `finalize` | `env: Env, proposal_id: u64` | `()` | None | Finalizes proposal after deadline |
| `get_proposal` | `env: Env, proposal_id: u64` | `Proposal` | None | Gets proposal details |
| `get_status` | `env: Env, proposal_id: u64` | `ProposalStatus` | None | Gets proposal status |
| `get_tally` | `env: Env, proposal_id: u64` | `(u64, u64)` | None | Gets (yes, no) vote counts |
| `has_voted` | `env: Env, proposal_id: u64, voter: Address` | `bool` | None | Checks if address voted |
| `is_finalized` | `env: Env, proposal_id: u64` | `bool` | None | Checks if finalized |
| `total_proposals` | `env: Env` | `u64` | None | Gets total proposals |

### Data Structures
```rust
struct Proposal {
    id: u64,
    description: String,
    creator: Address,
    yes_votes: u64,
    no_votes: u64,
    deadline: u64,
    status: ProposalStatus,
    finalized: bool,
}

enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Tied,
}
```

### Example Usage
```rust
let client = VotingContractClient::new(&env, &contract_id);

// Create proposal
let proposal_id = client.create_proposal(
    &creator,
    &String::from_str(&env, "Increase budget"),
    &deadline
);

// Vote
client.vote(&proposal_id, &voter1, &true);  // Yes
client.vote(&proposal_id, &voter2, &false); // No

// Finalize after deadline
client.finalize(&proposal_id);
let status = client.get_status(&proposal_id); // Returns Passed/Rejected/Tied
```

---

## Escrow

**Path:** `contracts/escrow/src/lib.rs`  
**WASM:** `sorosim_escrow.wasm`  
**Contract:** `EscrowContract`

### Description
Time-locked escrow with release and refund capabilities.

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `deposit` | `env: Env, depositor: Address, beneficiary: Address, amount: i128, release_time: u64` | `u64` | Depositor | Creates escrow |
| `release` | `env: Env, escrow_id: u64` | `()` | None | Releases funds after time |
| `refund` | `env: Env, escrow_id: u64, depositor: Address` | `()` | Depositor | Refunds before time |
| `get_escrow` | `env: Env, escrow_id: u64` | `Escrow` | None | Gets escrow details |
| `can_release` | `env: Env, escrow_id: u64` | `bool` | None | Checks if releasable |
| `total_escrows` | `env: Env` | `u64` | None | Gets total escrows |

### Data Structures
```rust
struct Escrow {
    id: u64,
    depositor: Address,
    beneficiary: Address,
    amount: i128,
    release_time: u64,
    status: EscrowStatus,
}

enum EscrowStatus {
    Active,
    Released,
    Refunded,
}
```

### Example Usage
```rust
let client = EscrowContractClient::new(&env, &contract_id);

// Create escrow
let escrow_id = client.deposit(
    &depositor,
    &beneficiary,
    &1000,
    &release_time
);

// Refund before release time
client.refund(&escrow_id, &depositor);

// Or release after time
client.release(&escrow_id);
```

---

## Multisig

**Path:** `contracts/multisig/src/lib.rs`  
**WASM:** `sorosim_multisig.wasm`  
**Contract:** `MultisigContract`

### Description
Multi-signature wallet with configurable threshold and transaction proposals.

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `initialize` | `env: Env, owners: Vec<Address>, threshold: u32` | `()` | None | Initializes multisig |
| `propose` | `env: Env, proposer: Address, to: Address, amount: i128` | `u64` | Proposer (must be owner) | Proposes transaction |
| `approve` | `env: Env, tx_id: u64, approver: Address` | `()` | Approver (must be owner) | Approves transaction |
| `execute` | `env: Env, tx_id: u64` | `()` | None | Executes if threshold met |
| `cancel` | `env: Env, tx_id: u64, canceller: Address` | `()` | Canceller (must be proposer) | Cancels transaction |
| `get_transaction` | `env: Env, tx_id: u64` | `Transaction` | None | Gets transaction details |
| `get_approval_count` | `env: Env, tx_id: u64` | `u32` | None | Gets approval count |
| `is_threshold_met` | `env: Env, tx_id: u64` | `bool` | None | Checks if threshold met |
| `get_owners` | `env: Env` | `Vec<Address>` | None | Gets owner list |
| `get_threshold` | `env: Env` | `u32` | None | Gets threshold |
| `is_owner` | `env: Env, address: Address` | `bool` | None | Checks if address is owner |
| `total_transactions` | `env: Env` | `u64` | None | Gets total transactions |

### Data Structures
```rust
struct Transaction {
    id: u64,
    proposer: Address,
    to: Address,
    amount: i128,
    approvals: Vec<Address>,
    status: TxStatus,
}

enum TxStatus {
    Pending,
    Executed,
    Cancelled,
}
```

### Example Usage
```rust
let client = MultisigContractClient::new(&env, &contract_id);

// Initialize
let owners = vec![&env, owner1, owner2, owner3];
client.initialize(&owners, &2); // 2 of 3 required

// Propose transaction
let tx_id = client.propose(&owner1, &recipient, &1000);

// Approve
client.approve(&tx_id, &owner2);

// Execute when threshold met
client.execute(&tx_id);
```

---

## Oracle

**Path:** `contracts/oracle/src/lib.rs`  
**WASM:** `sorosim_oracle.wasm`  
**Contract:** `OracleContract`

### Description
Mock price oracle with admin-controlled price feeds for testing.

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `initialize` | `env: Env, admin: Address` | `()` | None | Initializes oracle |
| `set_price` | `env: Env, admin: Address, asset: String, price: i128, decimals: u32` | `()` | Admin | Sets asset price |
| `get_price` | `env: Env, asset: String` | `i128` | None | Gets asset price |
| `get_price_data` | `env: Env, asset: String` | `PriceData` | None | Gets full price data |
| `get_price_age` | `env: Env, asset: String` | `u64` | None | Gets price data age |
| `has_price` | `env: Env, asset: String` | `bool` | None | Checks if price exists |
| `get_admin` | `env: Env` | `Address` | None | Gets admin address |
| `transfer_admin` | `env: Env, current_admin: Address, new_admin: Address` | `()` | Current Admin | Transfers admin role |

### Data Structures
```rust
struct PriceData {
    asset: String,
    price: i128,
    decimals: u32,
    timestamp: u64,
}
```

### Example Usage
```rust
let client = OracleContractClient::new(&env, &contract_id);

// Initialize
client.initialize(&admin);

// Set prices
client.set_price(&admin, &String::from_str(&env, "BTC"), &50000_00, &2);
client.set_price(&admin, &String::from_str(&env, "ETH"), &3000_00, &2);

// Query price
let btc_price = client.get_price(&String::from_str(&env, "BTC"));
```

---

## AMM

**Path:** `contracts/amm/src/lib.rs`  
**WASM:** `sorosim_amm.wasm`  
**Contract:** `AmmContract`

### Description
Constant-product Automated Market Maker (x * y = k) with swap and liquidity functions.

### Entry Points

| Function | Parameters | Returns | Authorization | Description |
|----------|-----------|---------|---------------|-------------|
| `initialize` | `env: Env, token_a: Address, token_b: Address` | `()` | None | Initializes AMM pool |
| `add_liquidity` | `env: Env, provider: Address, amount_a: i128, amount_b: i128` | `i128` | Provider | Adds liquidity, returns LP shares |
| `remove_liquidity` | `env: Env, provider: Address, shares: i128` | `(i128, i128)` | Provider | Removes liquidity, returns (amount_a, amount_b) |
| `swap_a_for_b` | `env: Env, user: Address, amount_a_in: i128, min_amount_b_out: i128` | `i128` | User | Swaps token A for B |
| `swap_b_for_a` | `env: Env, user: Address, amount_b_in: i128, min_amount_a_out: i128` | `i128` | User | Swaps token B for A |
| `get_reserves` | `env: Env` | `(i128, i128)` | None | Gets (reserve_a, reserve_b) |
| `get_tokens` | `env: Env` | `(Address, Address)` | None | Gets token addresses |
| `get_shares` | `env: Env, address: Address` | `i128` | None | Gets LP shares for address |
| `get_total_shares` | `env: Env` | `i128` | None | Gets total LP shares |
| `get_k` | `env: Env` | `i128` | None | Gets constant product |
| `get_price_a` | `env: Env` | `i128` | None | Gets price of A in terms of B |
| `get_price_b` | `env: Env` | `i128` | None | Gets price of B in terms of A |

### Example Usage
```rust
let client = AmmContractClient::new(&env, &contract_id);

// Initialize
client.initialize(&token_a_id, &token_b_id);

// Add liquidity
let shares = client.add_liquidity(&provider, &1000, &2000);

// Swap
let amount_out = client.swap_a_for_b(&user, &100, &190);

// Remove liquidity
let (amount_a, amount_b) = client.remove_liquidity(&provider, &shares);
```

---

## Auth

**Path:** `contracts/auth/src/lib.rs`  
**WASM:** `sorosim_auth.wasm`  
**Contract:** `AuthContract`

### Description
Demonstrates various authorization patterns: simple auth, admin-only, role-based, multi-auth, and pausable.

### Entry Points

| Function | Parameters | Authorization Pattern | Description |
|----------|-----------|----------------------|-------------|
| `initialize` | `env: Env, admin: Address` | Admin | Initializes contract |
| `set_balance` | `env: Env, user: Address, amount: i128` | User | Simple single-auth |
| `admin_set_balance` | `env: Env, admin: Address, user: Address, amount: i128` | Admin | Admin-only function |
| `moderate_set_balance` | `env: Env, caller: Address, user: Address, amount: i128` | Caller (Admin or Moderator) | Role-based auth |
| `multi_auth_transfer` | `env: Env, admin: Address, from: Address, to: Address, amount: i128` | Admin + From | Multi-signature auth |
| `pausable_operation` | `env: Env, user: Address` | User | Respects pause state |
| `pause` | `env: Env, admin: Address` | Admin | Pauses contract |
| `unpause` | `env: Env, admin: Address` | Admin | Unpauses contract |
| `grant_role` | `env: Env, admin: Address, user: Address, role: Role` | Admin | Grants role to user |
| `get_balance` | `env: Env, user: Address` | None | Gets balance |
| `get_role` | `env: Env, user: Address` | None | Gets user role |
| `is_paused` | `env: Env` | None | Checks if paused |
| `get_admin` | `env: Env` | None | Gets admin address |

### Data Structures
```rust
enum Role {
    Admin,
    Moderator,
    User,
}
```

---

## Events

**Path:** `contracts/events/src/lib.rs`  
**WASM:** `sorosim_events.wasm`  
**Contract:** `EventsContract`

### Description
Comprehensive examples of event emission for all Soroban data types.

### Entry Points

All functions emit events demonstrating different patterns:

| Function | Event Pattern |
|----------|--------------|
| `emit_simple` | Simple topic, no data |
| `emit_with_value` | Topic + i128 data |
| `emit_with_address` | Topic + Address data |
| `emit_transfer` | Indexed topics (from, to) + amount |
| `emit_approval` | Indexed topics (owner, spender) + amount |
| `emit_transfer_data` | Custom struct data |
| `emit_status_change` | Enum data |
| `emit_with_string` | String data |
| `emit_with_tuple` | Tuple data |
| `emit_with_vec` | Vector data |
| `emit_multiple` | Multiple events in sequence |
| `emit_multi_topic` | Multiple custom topics |
| `emit_with_bool` | Boolean data |
| `emit_with_u64` | u64 data |
| `emit_with_u32` | u32 data |
| `transfer_with_events` | Real-world transfer with events |

---

## Cross-Contract

### Callee

**Path:** `contracts/cross-contract/callee/src/lib.rs`  
**WASM:** `sorosim_cross_callee.wasm`  
**Contract:** `CalleeContract`

#### Description
Target contract for cross-contract invocation demonstrations.

#### Entry Points

| Function | Parameters | Returns | Description |
|----------|-----------|---------|-------------|
| `initialize` | `env: Env, admin: Address` | `()` | Initializes contract |
| `get_counter` | `env: Env` | `i128` | Returns counter value |
| `increment` | `env: Env` | `i128` | Increments counter |
| `add_value` | `env: Env, value: i128` | `i128` | Adds value to counter |
| `set_balance` | `env: Env, user: Address, amount: i128` | `()` | Sets user balance |
| `get_balance` | `env: Env, user: Address` | `i128` | Gets user balance |
| `get_stats` | `env: Env` | `(i128, Address)` | Returns (counter, admin) |
| `get_data` | `env: Env, caller: Address` | `CalleeData` | Returns struct data |
| `multiply` | `env: Env, a: i128, b: i128` | `i128` | Multiplies two numbers |
| `reset_counter` | `env: Env, admin: Address` | `()` | Resets counter (admin-only) |

### Caller

**Path:** `contracts/cross-contract/caller/src/lib.rs`  
**WASM:** `sorosim_cross_caller.wasm`  
**Contract:** `CallerContract`

#### Description
Demonstrates all cross-contract invocation patterns by calling the Callee contract.

#### Entry Points

| Function | Demonstrates |
|----------|-------------|
| `initialize` | Setting callee address |
| `call_get_counter` | Simple read call |
| `call_increment` | State modification call |
| `call_add_value` | Call with parameters |
| `call_get_balance` | Call with Address parameter |
| `call_get_stats` | Call returning tuple |
| `call_get_data` | Call returning struct |
| `call_sequence` | Multiple sequential calls |
| `call_multiply` | Call with computation |
| `call_and_process` | Composability example |

---

## Storage

**Path:** `contracts/storage/src/lib.rs`  
**WASM:** `sorosim_storage.wasm`  
**Contract:** `StorageContract`

### Description
Demonstrates all three Soroban storage types: instance, persistent, and temporary.

### Entry Points

#### Instance Storage (Contract Metadata)
| Function | Description |
|----------|-------------|
| `initialize` | Initializes with admin and timestamp |
| `increment_counter` | Increments global counter |
| `get_counter` | Gets global counter |
| `get_admin` | Gets admin address |
| `get_init_time` | Gets initialization timestamp |

#### Persistent Storage (Critical Data)
| Function | Description |
|----------|-------------|
| `set_balance` | Sets user balance |
| `get_balance` | Gets user balance |
| `set_user_data` | Sets complex user data |
| `get_user_data` | Gets user data struct |
| `remove_user_data` | Removes user data |
| `has_user_data` | Checks if user data exists |

#### Temporary Storage (Cache/Sessions)
| Function | Description |
|----------|-------------|
| `set_session` | Sets temporary session data |
| `get_session` | Gets session data |
| `set_cache` | Sets cache value |
| `get_cache` | Gets cache value |
| `has_session` | Checks if session exists |

#### Mixed Operations
| Function | Description |
|----------|-------------|
| `process_transaction` | Uses all three storage types |
| `get_storage_summary` | Reads from all storage types |

### Storage Types Summary

| Type | Lifetime | Cost | Use Case |
|------|----------|------|----------|
| Instance | Contract instance | Medium | Contract config, counters |
| Persistent | Permanent | High | Balances, ownership |
| Temporary | ~5 minutes | Low | Sessions, cache |

---

## Contract Size Summary

| Contract | Functions | Complexity | Primary Use Case |
|----------|-----------|------------|------------------|
| Counter | 3 | Simple | Learning |
| Token | 6 | Medium | Fungible tokens |
| NFT | 4 | Simple | Non-fungible tokens |
| Voting | 9 | Medium | Governance |
| Escrow | 6 | Medium | Conditional payments |
| Multisig | 12 | Complex | Multi-party wallets |
| Oracle | 8 | Medium | Price feeds |
| AMM | 12 | Complex | Token swaps |
| Auth | 13 | Medium | Authorization patterns |
| Events | 15 | Simple | Event logging |
| Cross-Contract | 10 + 10 | Medium | Contract interaction |
| Storage | 15 | Medium | Storage patterns |

---

## Deployment

### Using Soroban CLI

```bash
# Deploy a contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sorosim_counter.wasm \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Invoke a function
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- \
  increment
```

### Contract Interaction

All contracts follow standard Soroban patterns:
1. Deploy WASM
2. Initialize if required
3. Invoke functions with proper authorization
4. Query state as needed

---

## Testing

Each contract includes comprehensive unit tests. Run tests with:

```bash
# All tests
cargo test --workspace

# Specific contract
cargo test -p sorosim-counter
cargo test -p sorosim-token
```

---

## Documentation

Generate full API documentation:

```bash
cargo doc --workspace --no-deps --open
```

---

## License

MIT License - See LICENSE file for details

---

## Contributing

See CONTRIBUTING.md for guidelines on adding new contracts or improving existing ones.
