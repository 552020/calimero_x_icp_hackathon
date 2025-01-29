# Hello App - Calimero Node Application

A simple Hello World application written in Rust that compiles to WebAssembly (WASM) for deployment on Calimero Network nodes.

## Overview

This is the Rust implementation of the Hello App that will be compiled to WebAssembly and deployed on a Calimero node. The application provides a simple greeting functionality through a JSON-RPC interface.

## Project Structure

```bash
.
├── Cargo.toml        # Rust dependencies and build configuration
└── src/
    ├── lib.rs        # Main Calimero application code
    └── main.rs       # Optional local testing executable
```

## Calimero Application Setup

1. Configure `Cargo.toml` with required dependencies and WASM settings:

```toml
[package]
name = "hello-app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
calimero-sdk = { git = "https://github.com/calimero-network/core" }

[profile.app-release]
inherits = "release"
codegen-units = 1
opt-level = "z"
lto = true
debug = false
panic = "abort"
overflow-checks = true
```

2. Create the Calimero application in `src/lib.rs`:

```rust
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::app;

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct HelloApp {}

#[app::logic]
impl HelloApp {
    #[app::init]
    pub fn init() -> Self {
        HelloApp {}
    }

    pub fn say_hello(&self) -> String {
        "Hello from Calimero Node!".to_string()
    }
}
```

3. Build the WASM application using the provided build script:

```bash
#!/bin/bash
set -e

cd "$(dirname $0)"
TARGET="${CARGO_TARGET_DIR:-target}"

rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --profile app-release
mkdir -p res
cp $TARGET/wasm32-unknown-unknown/app-release/hello_app.wasm ./res/
```

Make the script executable:

```bash
chmod +x build.sh
```

## Local Development (Optional)

For local testing, you can create a simple `main.rs`:

```rust
use hello_app::add;

fn main() {
    let result = add(5, 7);
    println!("5 + 7 = {}", result);
}
```

This local executable is separate from the Calimero node functionality and won't be included in the WASM build.

## Deploying to Calimero Node

For deployment instructions and node interaction, see the [Node API Testing](../docs/node_api_testing.md) documentation.

## Next Steps

1. Build the WASM application: `./build.sh`
2. Deploy to your Calimero node
3. Test the interaction using the provided API endpoints
4. Integrate with the frontend application
