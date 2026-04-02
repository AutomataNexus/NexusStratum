//! # stratum-motion
//!
//! Animation and transition system for NexusStratum.
//! All animations are CSS-based with zero JavaScript runtime overhead.
//!
//! Reduced-motion support: [`AnimationStyle`] provides both a
//! `transition_css` field for normal animations and a `reduced_motion_css`
//! field with `0ms` duration. Framework adapters should apply
//! `reduced_motion_css` when the user's OS has `prefers-reduced-motion: reduce`
//! enabled (detected via CSS `@media` query or JavaScript `matchMedia`).

pub mod easing;
pub mod transition;

pub use easing::Easing;
pub use transition::{AnimationStyle, CssState, Transition, TransitionConfig};
