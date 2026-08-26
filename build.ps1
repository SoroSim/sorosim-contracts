# PowerShell Build Script for SoroSim Contracts
# Usage: .\build.ps1 [command] [contract-name]
# Examples:
#   .\build.ps1 build
#   .\build.ps1 build counter
#   .\build.ps1 test token
#   .\build.ps1 clean

param(
    [Parameter(Position=0)]
    [string]$Command = "build",
    
    [Parameter(Position=1)]
    [string]$Contract = ""
)

$ErrorActionPreference = "Stop"

# Contract list
$Contracts = @(
    "counter", "token", "nft", "voting", "escrow", "multisig",
    "oracle", "amm", "auth", "events", "cross-caller", "cross-callee", "storage"
)

function Show-Help {
    Write-Host "SoroSim Contracts Build System" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\build.ps1 [command] [contract-name]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Commands:" -ForegroundColor Green
    Write-Host "  build [contract]    - Build all contracts or specific contract to WASM"
    Write-Host "  test [contract]     - Run tests for all contracts or specific contract"
    Write-Host "  check               - Check code without building"
    Write-Host "  fmt                 - Format all code"
    Write-Host "  clippy              - Run clippy linter"
    Write-Host "  clean               - Remove build artifacts"
    Write-Host "  list                - List all available contracts"
    Write-Host "  install-target      - Install wasm32-unknown-unknown target"
    Write-Host "  help                - Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\build.ps1 build              # Build all contracts"
    Write-Host "  .\build.ps1 build counter      # Build only counter contract"
    Write-Host "  .\build.ps1 test token         # Test only token contract"
    Write-Host "  .\build.ps1 clean              # Clean build artifacts"
}

function Build-All {
    Write-Host "Building all contracts..." -ForegroundColor Cyan
    cargo build --target wasm32-unknown-unknown --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Build complete!" -ForegroundColor Green
    } else {
        Write-Host "Build failed!" -ForegroundColor Red
        exit 1
    }
}

function Build-Contract {
    param([string]$Name)
    
    if ($Contracts -notcontains $Name) {
        Write-Host "Unknown contract: $Name" -ForegroundColor Red
        Write-Host "Available contracts: $($Contracts -join ', ')" -ForegroundColor Yellow
        exit 1
    }
    
    Write-Host "Building $Name contract..." -ForegroundColor Cyan
    cargo build -p "sorosim-$Name" --target wasm32-unknown-unknown --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "$Name contract built successfully!" -ForegroundColor Green
    } else {
        Write-Host "Build failed!" -ForegroundColor Red
        exit 1
    }
}

function Test-All {
    Write-Host "Running all tests..." -ForegroundColor Cyan
    cargo test --workspace
    if ($LASTEXITCODE -eq 0) {
        Write-Host "All tests passed!" -ForegroundColor Green
    } else {
        Write-Host "Tests failed!" -ForegroundColor Red
        exit 1
    }
}

function Test-Contract {
    param([string]$Name)
    
    if ($Contracts -notcontains $Name) {
        Write-Host "Unknown contract: $Name" -ForegroundColor Red
        Write-Host "Available contracts: $($Contracts -join ', ')" -ForegroundColor Yellow
        exit 1
    }
    
    Write-Host "Running tests for $Name contract..." -ForegroundColor Cyan
    cargo test -p "sorosim-$Name"
    if ($LASTEXITCODE -eq 0) {
        Write-Host "$Name tests passed!" -ForegroundColor Green
    } else {
        Write-Host "Tests failed!" -ForegroundColor Red
        exit 1
    }
}

function Check-Code {
    Write-Host "Checking code..." -ForegroundColor Cyan
    cargo check --workspace
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Check complete!" -ForegroundColor Green
    } else {
        Write-Host "Check failed!" -ForegroundColor Red
        exit 1
    }
}

function Format-Code {
    Write-Host "Formatting code..." -ForegroundColor Cyan
    cargo fmt --all
    Write-Host "Format complete!" -ForegroundColor Green
}

function Run-Clippy {
    Write-Host "Running clippy..." -ForegroundColor Cyan
    cargo clippy --workspace --all-targets -- -D warnings
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Clippy check passed!" -ForegroundColor Green
    } else {
        Write-Host "Clippy found issues!" -ForegroundColor Red
        exit 1
    }
}

function Clean-Build {
    Write-Host "Cleaning..." -ForegroundColor Cyan
    cargo clean
    Write-Host "Clean complete!" -ForegroundColor Green
}

function List-Contracts {
    Write-Host "Available contracts:" -ForegroundColor Cyan
    foreach ($contract in $Contracts) {
        Write-Host "  - $contract" -ForegroundColor White
    }
}

function Install-Target {
    Write-Host "Installing wasm32-unknown-unknown target..." -ForegroundColor Cyan
    rustup target add wasm32-unknown-unknown
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Target installed successfully!" -ForegroundColor Green
    } else {
        Write-Host "Installation failed!" -ForegroundColor Red
        exit 1
    }
}

# Main command dispatcher
switch ($Command.ToLower()) {
    "build" {
        if ($Contract -eq "") {
            Build-All
        } else {
            Build-Contract $Contract
        }
    }
    "test" {
        if ($Contract -eq "") {
            Test-All
        } else {
            Test-Contract $Contract
        }
    }
    "check" {
        Check-Code
    }
    "fmt" {
        Format-Code
    }
    "clippy" {
        Run-Clippy
    }
    "clean" {
        Clean-Build
    }
    "list" {
        List-Contracts
    }
    "install-target" {
        Install-Target
    }
    "help" {
        Show-Help
    }
    default {
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Write-Host ""
        Show-Help
        exit 1
    }
}
