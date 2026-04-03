//! # stratum-dioxus
//!
//! Real Dioxus UI components for NexusStratum.
//!
//! Every component is a native Dioxus `#[component]` function that renders
//! actual HTML with Tailwind CSS classes. Use in any Dioxus `rsx!` macro.
//!
//! ```ignore
//! use stratum_dioxus::components::button::*;
//!
//! rsx! { Button { variant: ButtonVariant::Destructive, "Delete" } }
//! ```

pub mod adapter;
pub mod components;
pub mod provider;

pub use components::alert::*;
pub use components::badge::*;
pub use components::button::*;
pub use components::card::*;
pub use components::heading::*;
pub use components::input::*;
pub use components::separator::*;
pub use components::skeleton::*;
pub use components::spinner::*;
pub use components::text::*;

pub use adapter::DioxusAdapter;
pub use provider::ThemeContext;
pub use stratum_theme::Theme;
