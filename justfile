set dotenv-load := true

# Run all checks and tests
check *FLAGS:
    cargo clippy --all {{FLAGS}} -- -D warnings
    cargo fmt --all --check {{FLAGS}}

# Test the warner
test *FLAGS:
    cargo run {{FLAGS}} -- --test

# Test using Nix
test-nix:
    nix run .# -- --test
