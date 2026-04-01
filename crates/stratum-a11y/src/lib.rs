//! # stratum-a11y
//!
//! Accessibility utilities for the NexusStratum UI component library.
//!
//! Provides framework-agnostic helpers for:
//! - Keyboard navigation (ARIA APG patterns)
//! - Live region announcements
//! - Media query preference detection
//! - Focus visible strategies

pub mod focus_visible;
pub mod keyboard;
pub mod live_region;
pub mod media_query;

pub use focus_visible::{FocusVisibleStrategy, InputModality};
pub use keyboard::{KeyboardNav, NavStrategy};
pub use live_region::{Announcement, LiveRegion};
pub use media_query::{
    is_keyboard_user, prefers_color_scheme_dark, prefers_high_contrast, prefers_reduced_motion,
    MediaQueryPreference,
};
