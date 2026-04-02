//! # stratum-sdk
//!
//! SDK and editor tooling for NexusStratum.
//!
//! **Status: Phase 3** -- This crate is scaffolded but not yet implemented.
//! The SDK will provide metadata extraction, code generation helpers, and
//! editor integration utilities for NexusStratum component authors.
//!
//! ## Planned modules
//!
//! - [`metadata`] -- Extract component metadata (props, variants, slots) for
//!   documentation generators and IDE plugins
//! - `codegen` -- Code generation utilities for component boilerplate
//! - `lint` -- Lint rules for common component authoring mistakes
//!
//! ## Dependencies
//!
//! Only `stratum-core` is required at this stage. Additional dependencies
//! (`stratum-components`, `serde`, `serde_json`) will be added when
//! implementation begins.

pub mod metadata;
