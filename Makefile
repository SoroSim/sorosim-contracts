# Makefile for SoroSim Contracts
# Build all Soroban contracts to WASM

.PHONY: all build clean test fmt check help

# Default target
all: build

# Build all contracts
build:
	@echo "Building all contracts..."
	@cargo build --target wasm32-unknown-unknown --release
	@echo "Build complete!"

# Build a specific contract
build-counter:
	@cargo build -p sorosim-counter --target wasm32-unknown-unknown --release

build-token:
	@cargo build -p sorosim-token --target wasm32-unknown-unknown --release

build-nft:
	@cargo build -p sorosim-nft --target wasm32-unknown-unknown --release

build-voting:
	@cargo build -p sorosim-voting --target wasm32-unknown-unknown --release

build-escrow:
	@cargo build -p sorosim-escrow --target wasm32-unknown-unknown --release

build-multisig:
	@cargo build -p sorosim-multisig --target wasm32-unknown-unknown --release

build-oracle:
	@cargo build -p sorosim-oracle --target wasm32-unknown-unknown --release

build-amm:
	@cargo build -p sorosim-amm --target wasm32-unknown-unknown --release

build-auth:
	@cargo build -p sorosim-auth --target wasm32-unknown-unknown --release

build-events:
	@cargo build -p sorosim-events --target wasm32-unknown-unknown --release

build-cross-caller:
	@cargo build -p sorosim-cross-caller --target wasm32-unknown-unknown --release

build-cross-callee:
	@cargo build -p sorosim-cross-callee --target wasm32-unknown-unknown --release

build-storage:
	@cargo build -p sorosim-storage --target wasm32-unknown-unknown --release

# Run tests
test:
	@echo "Running tests..."
	@cargo test --workspace

# Run tests for specific contract
test-counter:
	@cargo test -p sorosim-counter

test-token:
	@cargo test -p sorosim-token

test-voting:
	@cargo test -p sorosim-voting

test-escrow:
	@cargo test -p sorosim-escrow

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt --all

# Check code without building
check:
	@echo "Checking code..."
	@cargo check --workspace

# Check with clippy
clippy:
	@echo "Running clippy..."
	@cargo clippy --workspace --all-targets -- -D warnings

# Clean build artifacts
clean:
	@echo "Cleaning..."
	@cargo clean
	@echo "Clean complete!"

# Install wasm32 target
install-target:
	@echo "Installing wasm32-unknown-unknown target..."
	@rustup target add wasm32-unknown-unknown

# Optimize WASM files (requires wasm-opt from binaryen)
optimize:
	@echo "Optimizing WASM files..."
	@for file in target/wasm32-unknown-unknown/release/*.wasm; do \
		if [ -f "$$file" ]; then \
			wasm-opt -Oz "$$file" -o "$$file.opt"; \
			mv "$$file.opt" "$$file"; \
			echo "Optimized $$file"; \
		fi \
	done
	@echo "Optimization complete!"

# Build and optimize
build-opt: build optimize

# List all contracts
list:
	@echo "Available contracts:"
	@echo "  - counter"
	@echo "  - token"
	@echo "  - nft"
	@echo "  - voting"
	@echo "  - escrow"
	@echo "  - multisig"
	@echo "  - oracle"
	@echo "  - amm"
	@echo "  - auth"
	@echo "  - events"
	@echo "  - cross-caller"
	@echo "  - cross-callee"
	@echo "  - storage"

# Help target
help:
	@echo "SoroSim Contracts Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build              - Build all contracts to WASM"
	@echo "  make build-<contract>   - Build specific contract (e.g., make build-counter)"
	@echo "  make test               - Run all tests"
	@echo "  make test-<contract>    - Run tests for specific contract"
	@echo "  make check              - Check code without building"
	@echo "  make clippy             - Run clippy linter"
	@echo "  make fmt                - Format all code"
	@echo "  make clean              - Remove build artifacts"
	@echo "  make optimize           - Optimize WASM files (requires wasm-opt)"
	@echo "  make build-opt          - Build and optimize"
	@echo "  make install-target     - Install wasm32-unknown-unknown target"
	@echo "  make list               - List all available contracts"
	@echo "  make help               - Show this help message"
	@echo ""
	@echo "Example usage:"
	@echo "  make build              # Build all contracts"
	@echo "  make build-token        # Build only token contract"
	@echo "  make test-counter       # Test only counter contract"
