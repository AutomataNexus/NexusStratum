//! # stratum-motion
//!
//! Animation and transition system for NexusStratum.
//! All animations are CSS-based with zero JavaScript runtime overhead.
//! Automatically respects `prefers-reduced-motion`.

pub mod easing;
pub mod transition;

pub use easing::Easing;
pub use transition::{AnimationStyle, Transition, TransitionConfig};
