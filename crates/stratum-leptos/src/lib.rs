//! # stratum-leptos
//!
//! Leptos framework adapter for NexusStratum.
//!
//! Provides idiomatic Leptos component APIs wrapping the headless
//! primitives from `stratum-primitives` with styled defaults from
//! `stratum-components`.
//!
//! ## What the adapter does
//!
//! - Converts `stratum-primitives` state to Leptos signals
//! - Bridges `ComponentEvent` to Leptos event handlers
//! - Generates Leptos `view!` macro output from `RenderOutput`
//! - Provides Leptos-specific context providers (ThemeProvider, ToasterProvider)
//!
//! ## Usage
//!
//! ```ignore
//! use stratum_leptos::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <ThemeProvider theme=Theme::default()>
//!             <Button variant=ButtonVariant::Primary>"Click me"</Button>
//!         </ThemeProvider>
//!     }
//! }
//! ```

pub mod adapter;
pub mod provider;

// Re-export all styled component types for convenience
pub use stratum_components::common::Size;
pub use stratum_components::forms::button::{Button, ButtonProps, ButtonVariant};
pub use stratum_components::forms::input::{Input, InputProps};
pub use stratum_components::forms::textarea::{Textarea, TextareaProps};
pub use stratum_components::forms::checkbox::{Checkbox, CheckboxProps};
pub use stratum_components::forms::radio::{Radio, RadioProps};
pub use stratum_components::forms::switch::{Switch, SwitchProps};
pub use stratum_components::forms::select::{Select, SelectProps};
pub use stratum_components::forms::form::{Form, FormProps, FormField, FormFieldProps};
pub use stratum_components::overlay::dialog::{Dialog, DialogProps};
pub use stratum_components::overlay::alert_dialog::{AlertDialog, AlertDialogProps};
pub use stratum_components::overlay::tooltip::{Tooltip, TooltipProps};
pub use stratum_components::overlay::popover::{Popover, PopoverProps};
pub use stratum_components::overlay::toast::{Toast, ToastProps, ToastVariant};
pub use stratum_components::navigation::tabs::{Tab, TabProps, TabList, TabPanel};
pub use stratum_components::navigation::accordion::{Accordion, AccordionProps};
pub use stratum_components::data_display::badge::{Badge, BadgeProps, BadgeVariant};
pub use stratum_components::data_display::card::{Card, CardProps};
pub use stratum_components::data_display::skeleton::{Skeleton, SkeletonProps};
pub use stratum_components::data_display::spinner::{Spinner, SpinnerProps};
pub use stratum_components::layout::stack::{Stack, StackProps};
pub use stratum_components::layout::divider::{Divider, DividerProps};
pub use stratum_components::typography::text::{Text, TextProps};
pub use stratum_components::typography::heading::{Heading, HeadingProps};
pub use stratum_components::typography::link::{Link, LinkProps};
pub use stratum_components::utility::separator::{Separator, SeparatorProps};
pub use stratum_components::utility::visually_hidden::VisuallyHidden;
pub use stratum_components::utility::portal::Portal;
pub use stratum_components::utility::focus_scope::{FocusScope, FocusScopeProps};

// Re-export theme types
pub use stratum_theme::Theme;

// Re-export adapter
pub use adapter::StratumAdapter;
pub use provider::ThemeContext;
