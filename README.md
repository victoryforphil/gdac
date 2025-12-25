# gdac

> "*** **** IT - another config _language_"

A Rust library for defining serdable structs and configuring them using Lua-based configuration files. gdac bridges the gap between Rust's type safety and Lua's flexibility for configuration management.

## Overview

gdac provides a simple and powerful way to:
- Define configuration structures using Rust structs with `serde` support
- Load and parse Lua-based configuration files using `mlua`
- Seamlessly bridge between Lua and Rust data types
- Handle errors gracefully with `anyhow` and `thiserror`

## Project Structure

This project is organized as a Cargo workspace:

- **`gdac/`** - The core library providing Lua configuration utilities
- **`gdac-examples/`** - Example applications demonstrating library usage

## Features

- 🦀 **Type-safe configuration** - Define configs as Rust structs
- 🌙 **Lua-powered** - Flexible configuration using Lua scripts
- ⚡ **Zero-copy deserialization** - Efficient data handling with serde
- 📝 **Comprehensive error handling** - Using anyhow and thiserror
- 🔍 **Environment-sensitive logging** - Built-in tracing support

## Dependencies

### Library (`gdac`)
- `anyhow` - Flexible error handling
- `thiserror` - Custom error types
- `mlua` - Lua integration (Lua 5.4)
- `serde` - Serialization/deserialization with derive macros

### Examples (`gdac-examples`)
- `clap` - Command-line argument parsing with derive macros
- `tracing` + `tracing-subscriber` - Structured, environment-sensitive logging

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

Build the entire workspace:

```bash
cargo build
```

Build in release mode:

```bash
cargo build --release
```

### Running Examples

Run the example application:

```bash
cargo run --bin gdac-example
```

With verbose logging:

```bash
cargo run --bin gdac-example -- --verbose
```

With custom log level using environment variable:

```bash
RUST_LOG=debug cargo run --bin gdac-example
```

### Using the Library

Add gdac to your `Cargo.toml`:

```toml
[dependencies]
gdac = { path = "path/to/gdac" }
serde = { version = "1.0", features = ["derive"] }
```

Example usage:

```rust
use gdac::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    value: i32,
}

fn main() -> gdac::Result<()> {
    // Create Lua context
    let lua = mlua::Lua::new();
    
    // Load and execute Lua configuration
    lua.load(r#"
        config = {
            name = "my-app",
            value = 42
        }
    "#).exec()?;
    
    // Access configuration
    let globals = lua.globals();
    let config: mlua::Table = globals.get("config")?;
    let name: String = config.get("name")?;
    
    println!("Config name: {}", name);
    Ok(())
}
```

## Logging

The example applications use `tracing` for structured logging. Log levels can be controlled via:

1. **Command-line flag**: `--verbose` for debug output
2. **Environment variable**: `RUST_LOG=debug|info|warn|error`

Example:

```bash
# Info level (default)
cargo run --bin gdac-example

# Debug level via flag
cargo run --bin gdac-example -- --verbose

# Trace level via environment
RUST_LOG=trace cargo run --bin gdac-example
```

## Development

### Running Tests

```bash
cargo test
```

### Checking Code

```bash
cargo check
```

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
