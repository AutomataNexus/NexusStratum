//! # stratum-explorer
//!
//! Component explorer for NexusStratum — a browsable catalog of all
//! components with their props, variants, ARIA roles, and keyboard patterns.
//!
//! The explorer reads component metadata from `stratum-sdk` and generates
//! a static HTML site that can be served locally or deployed.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use stratum_explorer::Explorer;
//!
//! let explorer = Explorer::new();
//! explorer.generate("./explorer-output");
//! ```

pub mod generator;

pub use generator::Explorer;
