//! # stratum-leptos
//!
//! Real Leptos UI components for NexusStratum.
//!
//! Every component is a native Leptos `#[component]` function that renders
//! actual HTML with Tailwind CSS classes. Use in any Leptos `view!` macro.
//!
//! ```ignore
//! use stratum_leptos::components::button::*;
//!
//! view! {
//!     <Button variant=ButtonVariant::Destructive>"Delete"</Button>
//! }
//! ```

pub mod adapter;
pub mod components;
pub mod provider;

// Re-export everything at crate root for ergonomic imports
pub use components::alert::*;
pub use components::badge::*;
pub use components::button::*;
pub use components::card::*;
pub use components::checkbox::*;
pub use components::heading::*;
pub use components::input::*;
pub use components::label::*;
pub use components::progress::*;
pub use components::radio::*;
pub use components::select::*;
pub use components::separator::*;
pub use components::skeleton::*;
pub use components::spinner::*;
pub use components::switch::*;
pub use components::text::*;
pub use components::textarea::*;

pub use adapter::StratumAdapter;
pub use provider::ThemeContext;
pub use stratum_theme::Theme;
