# sorosim-contracts

Sample and reference Soroban smart contracts for the **SoroSim** sandbox.

These contracts serve two purposes:

1. **Reference implementations** — clean, well-documented examples developers can
   learn from and adapt.
2. **Simulation targets** — pre-built WASMs bundled with the SoroSim browser sandbox
   so users can explore contract behaviour without writing any code.

---

## Contracts

| Crate | Path | Description |
|-------|------|-------------|
| `sorosim-counter` | `contracts/counter` | Basic increment/decrement counter with persistent storage |
| `sorosim-token` | `contracts/token` | Minimal fungible token (mint, transfer, allowance) |
| `sorosim-nft` | `contracts/nft` | Non-fungible token with mint and ownership tracking |
| `sorosim-voting` | `contracts/voting` | Proposal creation, vote casting, and result finalization |
| `sorosim-escrow` | `contracts/escrow` | Time-locked escrow with release and refund paths |
| `sorosim-multisig` | `contracts/multisig` | Threshold multisig authorization and execution |
| `sorosim-oracle` | `contracts/oracle` | Admin-controlled mock price oracle |
| `sorosim-amm` | `contracts/amm` | Constant-product AMM (swap, add/remove liquidity) |
| `sorosim-auth` | `contracts/auth` | Custom auth patterns using `require_auth` |
| `sorosim-events` | `contracts/events` | Event emission for every supported topic/data type |
| `sorosim-cross-caller` | `contracts/cross-contract/caller` | Cross-contract invocation — caller side |
| `sorosim-cross-callee` | `contracts/cross-contract/callee` | Cross-contract invocation — callee side |
| `sorosim-storage` | `contracts/storage` | Temporary, persistent, and instance storage patterns |

---

## Prerequisites

| Tool | Version |
|------|---------|
| Rust | ≥ 1.81 |
| `wasm32-unknown-unknown` target | via `rustup target add wasm32-unknown-unknown` |
| `soroban-cli` (optional, for manual deploys) | ≥ 22.x |

---

## Building

```bash
# Build all contracts as WASM
make build

# Run all unit tests (native target)
make test

# Build a single contract
cargo build -p sorosim-counter --target wasm32-unknown-unknown --release
```

---

## Repository structure

```
sorosim-contracts/
├── Cargo.toml                  # Workspace manifest
├── Makefile                    # Build & test helpers
├── CONTRACT_REGISTRY.md        # Contract → entry-point mapping
└── contracts/
    ├── counter/
    ├── token/
    ├── nft/
    ├── voting/
    ├── escrow/
    ├── multisig/
    ├── oracle/
    ├── amm/
    ├── auth/
    ├── events/
    ├── cross-contract/
    │   ├── caller/
    │   └── callee/
    └── storage/
```

---

## License

Apache-2.0
