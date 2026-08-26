# Build Guide

This guide explains how to build the SoroSim contracts to WebAssembly (WASM).

## Prerequisites

1. **Rust toolchain** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm32-unknown-unknown target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
   Or use the build script:
   ```bash
   make install-target
   # or on Windows
   .\build.ps1 install-target
   ```

3. **Soroban CLI** (optional, for deployment)
   ```bash
   cargo install --locked soroban-cli
   ```

## Building Contracts

### Option 1: Using Make (Linux/macOS/WSL)

Build all contracts:
```bash
make build
```

Build a specific contract:
```bash
make build-counter
make build-token
make build-voting
```

Run tests:
```bash
make test
make test-counter
```

Other commands:
```bash
make check      # Check code without building
make fmt        # Format code
make clippy     # Run linter
make clean      # Remove build artifacts
make help       # Show all available commands
```

### Option 2: Using PowerShell Script (Windows)

Build all contracts:
```powershell
.\build.ps1 build
```

Build a specific contract:
```powershell
.\build.ps1 build counter
.\build.ps1 build token
.\build.ps1 build voting
```

Run tests:
```powershell
.\build.ps1 test
.\build.ps1 test counter
```

Other commands:
```powershell
.\build.ps1 check              # Check code
.\build.ps1 fmt                # Format code
.\build.ps1 clippy             # Run linter
.\build.ps1 clean              # Clean artifacts
.\build.ps1 list               # List all contracts
.\build.ps1 help               # Show help
```

### Option 3: Using Batch File (Windows)

The batch file is a wrapper for the PowerShell script:
```cmd
build.bat build
build.bat build counter
build.bat test
build.bat clean
```

### Option 4: Direct Cargo Commands

Build all contracts:
```bash
cargo build --target wasm32-unknown-unknown --release
```

Build a specific contract:
```bash
cargo build -p sorosim-counter --target wasm32-unknown-unknown --release
cargo build -p sorosim-token --target wasm32-unknown-unknown --release
```

Run tests:
```bash
cargo test --workspace
cargo test -p sorosim-counter
```

## Build Output

WASM files are generated in:
```
target/wasm32-unknown-unknown/release/*.wasm
```

Contract names:
- `sorosim_counter.wasm`
- `sorosim_token.wasm`
- `sorosim_nft.wasm`
- `sorosim_voting.wasm`
- `sorosim_escrow.wasm`
- `sorosim_multisig.wasm`
- `sorosim_oracle.wasm`
- `sorosim_amm.wasm`
- `sorosim_auth.wasm`
- `sorosim_events.wasm`
- `sorosim_cross_caller.wasm`
- `sorosim_cross_callee.wasm`
- `sorosim_storage.wasm`

## Optimization (Optional)

For production deployments, optimize WASM files using `wasm-opt` from [Binaryen](https://github.com/WebAssembly/binaryen):

```bash
# Install binaryen
# macOS: brew install binaryen
# Ubuntu: apt install binaryen
# Windows: Download from GitHub releases

# Optimize
make optimize
```

This reduces WASM file size by ~30-50%.

## Available Contracts

| Contract | Description |
|----------|-------------|
| counter | Simple counter with increment/decrement |
| token | Fungible token with allowances |
| nft | Non-fungible token implementation |
| voting | Proposal voting with time locks |
| escrow | Time-locked escrow with release/refund |
| multisig | Multi-signature wallet |
| oracle | Mock price oracle |
| amm | Constant-product AMM (swap, liquidity) |
| auth | Authorization patterns demo |
| events | Event emission examples |
| cross-caller | Cross-contract invocation (caller) |
| cross-callee | Cross-contract invocation (callee) |
| storage | Storage types demo (temp, persistent, instance) |

## Troubleshooting

### Error: target 'wasm32-unknown-unknown' not found

Install the target:
```bash
rustup target add wasm32-unknown-unknown
```

### Error: linker 'link.exe' not found (Windows)

Install Visual Studio Build Tools with C++ development tools.

### Build is slow

Use incremental builds and parallel compilation:
```bash
export CARGO_INCREMENTAL=1
export CARGO_BUILD_JOBS=8
```

### Tests fail with dependency errors

This is a known issue with `soroban-env-host` v22.1.3. The test code is correct; wait for SDK updates or skip tests:
```bash
cargo build --target wasm32-unknown-unknown --release  # Build without tests
```

## Deployment

To deploy to Stellar/Soroban network:

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Deploy example (testnet)
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sorosim_counter.wasm \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

See the [Soroban documentation](https://soroban.stellar.org/docs) for more details.

## CI/CD

GitHub Actions workflow is available in `.github/workflows/build.yml` for automated building and testing.
