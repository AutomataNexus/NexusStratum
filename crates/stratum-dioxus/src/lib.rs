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

pub use components::accordion::*;
pub use components::alert::*;
pub use components::aspect_ratio::AspectRatio;
pub use components::avatar::Avatar;
pub use components::badge::*;
pub use components::breadcrumb::*;
pub use components::button::*;
pub use components::card::*;
pub use components::carousel::*;
pub use components::checkbox::Checkbox;
pub use components::collapsible::*;
pub use components::command::*;
pub use components::context_menu::*;
pub use components::dialog::*;
pub use components::dropdown_menu::*;
pub use components::empty_state::EmptyState;
pub use components::heading::*;
pub use components::hover_card::*;
pub use components::input::*;
pub use components::kbd::Kbd;
pub use components::label::Label;
pub use components::menubar::*;
pub use components::navigation_menu::*;
pub use components::number_input::NumberInput;
pub use components::pagination::*;
pub use components::popover::*;
pub use components::progress::Progress;
pub use components::radio::*;
pub use components::resizable::*;
pub use components::scroll_area::ScrollArea;
pub use components::select::*;
pub use components::separator::*;
pub use components::sheet::*;
pub use components::skeleton::*;
pub use components::slider::Slider;
pub use components::spinner::*;
pub use components::switch::Switch;
pub use components::tabs::*;
pub use components::text::*;
pub use components::textarea::Textarea;
pub use components::toast::*;
pub use components::toggle::*;
pub use components::toggle_group::*;
pub use components::tooltip::Tooltip;
pub use components::visually_hidden::VisuallyHidden;

pub use adapter::DioxusAdapter;
pub use provider::ThemeContext;
pub use stratum_theme::Theme;
