//! # gdac - "*** **** IT - another config _language_"
//!
//! A library to define serdable Rust structs and configure them using Lua-based configs.
//!
//! This library provides utilities to:
//! - Define configuration structures using Rust structs with `serde`
//! - Load and parse Lua-based configuration files using `mlua`
//! - Bridge between Lua and Rust data types seamlessly
//!
//! ## Example
//!
//! ```rust,ignore
//! use gdac::prelude::*;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Config {
//!     name: String,
//!     value: i32,
//! }
//!
//! // Load configuration from Lua
//! // let config: Config = gdac::load_from_lua("config.lua")?;
//! ```

pub mod error;
pub mod prelude;

pub use error::{Error, Result};

/// Re-export commonly used types
pub use mlua;
pub use serde;
