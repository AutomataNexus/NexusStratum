//! Tailwind CSS integration for NexusStratum.
//!
//! Provides class building, conflict resolution (like tailwind-merge),
//! and Tailwind config generation from NexusStratum themes.

pub use stratum_tailwind_macros::tw;

mod builder;
mod config;
mod merge;

pub use builder::ClassBuilder;
pub use config::TailwindConfig;
pub use merge::merge_classes;
