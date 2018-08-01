//! # Users
//!
//! This is the Ayase developer documentation. It's probably not what you're looking for, but feel free to keep reading.
//!
//! # Background
//!
//! ## Definitions
//!
//! We highly recommend you read the [`shamrock` documentation](../shamrock/index.html) to understand what the following terms mean:
//!
//! - Construct
//! - Format
//! - Layout
//! - Source
//! - Store
//!
//! ## References
//!
//! - [Asagi](https://github.com/eksopl/asagi)
//!   - Imageboard archiver (backend only)
//!   - When we refer to its "layout", we're talking about how it's tied to a DB with a schema.
//!
//! # Crates
//!
//! - `ayase`
//!   - A nicely packaged wrapper over `ayase_core` that allows it to function as a fully-featured chan archiver.
//!   - Can replace Asagi out-of-the-box.
//! - `ayase_asagi`
//!   - Defines the Asagi layout.
//! - `ayase_chan`
//!   - Defines a trait for all imageboard Sources to implement (reduces boilerplate).
//!   - Defines some Constructs for common imageboard data structures.
//! - `ayase_config` (still deciding on scope)
//!   - Converts Asagi config files to Ayase ones.
//!   - Stores central config data structures.
//! - `ayase_core`
//!   - Runs event loops for archival.
//!   - Manages constructs/layouts/stores/sources.
//! - `ayase_json`
//!   - Defines a Format for JSON.
//! - `ayase_mysql`
//!   - Defines a Format and Store for MySQL.
//! - `ayase_postgresql`
//!   - Defines a Format and Store for PostgreSQL.
//! - `shamrock` (name is subject to change)
//!   - It defines a number of important traits that we build our entire archival infrastructure on.

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate shamrock;
extern crate mysql;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate r2d2_postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_value;

pub mod error;

use error::*;

quick_main!(run);

/// The heart of the program. This is where all of the coordination (for futures, threads, etc.) happens.
fn run() -> Result<()> {
    
    
    Ok(())
}