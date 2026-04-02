//! # stratum-sdk
//!
//! SDK and editor tooling for NexusStratum.
//!
//! Provides structured metadata about NexusStratum components for use by
//! documentation generators, IDE plugins, the component explorer, and
//! third-party tooling.
//!
//! ## Key Types
//!
//! - [`ComponentMeta`] — Metadata for a single component (name, props, ARIA, keyboard)
//! - [`PropMeta`] — Metadata for a single prop (name, type, default, values)
//! - [`RegistryMeta`] — The full component registry with lookup and JSON serialization

pub mod metadata;

pub use metadata::{ComponentMeta, KeyboardMeta, PropMeta, RegistryMeta};
