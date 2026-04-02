//! # stratum
//!
//! NexusStratum -- Production-ready UI component library for Rust.
//!
//! This is the umbrella crate that re-exports all core NexusStratum crates
//! for convenience. Import this single crate to get access to primitives,
//! components, theming, and accessibility utilities.
//!
//! ## Feature flags
//!
//! The following optional features enable additional integrations:
//!
//! | Feature      | Crate              | Description                          |
//! |--------------|--------------------|--------------------------------------|
//! | `leptos`     | `stratum-leptos`   | Leptos framework adapter             |
//! | `dioxus`     | `stratum-dioxus`   | Dioxus framework adapter             |
//! | `tailwind`   | `stratum-tailwind` | Tailwind CSS class generation        |
//! | `css`        | `stratum-css`      | CSS-in-Rust style output             |
//! | `icons`      | `stratum-icons`    | Icon library integration             |
//! | `motion`     | `stratum-motion`   | Animation and transition utilities   |
//! | `security`   | `stratum-security` | Content sanitization and CSP helpers |
//! | `full`       | *(all of above except `dioxus`)* | Enable everything  |

// Core crates (always available)
pub use stratum_core;
pub use stratum_primitives;
pub use stratum_components;
pub use stratum_theme;
pub use stratum_a11y;

// Optional framework adapters
#[cfg(feature = "leptos")]
pub use stratum_leptos;
#[cfg(feature = "dioxus")]
pub use stratum_dioxus;

// Optional style backends
#[cfg(feature = "tailwind")]
pub use stratum_tailwind;
#[cfg(feature = "css")]
pub use stratum_css;

// Optional add-ons
#[cfg(feature = "icons")]
pub use stratum_icons;
#[cfg(feature = "motion")]
pub use stratum_motion;
#[cfg(feature = "security")]
pub use stratum_security;
