//! Example application demonstrating gdac usage

use anyhow::Result;
use clap::Parser;
use gdac::prelude::*;
use tracing::{debug, info};

/// Example application for gdac - Lua-based configuration in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, default_value = "config.lua")]
    config: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Setup environment-sensitive logging with tracing
    // This respects RUST_LOG environment variable
    let default_filter = if args.verbose { "debug" } else { "info" };
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| default_filter.to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    info!("Starting gdac example application");
    debug!("Configuration file: {}", args.config);

    // Demonstrate basic usage
    info!("gdac library initialized successfully");
    
    // Create a simple Lua context
    let lua = mlua::Lua::new();
    
    // Example: Execute simple Lua code
    let result: i32 = lua.load("return 42").eval()?;
    info!("Lua execution test: 42 = {}", result);
    
    // Example: Create a Lua table
    lua.load(r#"
        config = {
            name = "example",
            value = 123
        }
    "#).exec()?;
    
    let globals = lua.globals();
    let config: mlua::Table = globals.get("config")?;
    let name: String = config.get("name")?;
    let value: i32 = config.get("value")?;
    
    info!("Loaded Lua config: name={}, value={}", name, value);
    
    if args.verbose {
        debug!("Verbose mode enabled - showing additional details");
        debug!("Lua version: {}", mlua::Lua::new().load("return _VERSION").eval::<String>()?);
    }

    info!("Example completed successfully");
    Ok(())
}
