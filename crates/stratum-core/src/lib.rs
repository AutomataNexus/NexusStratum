//! # stratum-core
//!
//! Foundation crate for the NexusStratum UI component library.
//! Provides framework-agnostic abstractions used by every other crate.
//!
//! ## Key Types
//!
//! - [`Component`] — The core trait all UI components implement
//! - [`Props`] — Trait for component property types
//! - [`ComponentEvent`] — Events components can handle
//! - [`AriaAttributes`] — ARIA accessibility attributes
//! - [`FocusManager`] — Focus management strategies
//! - [`IdGenerator`] — Unique ID generation for ARIA cross-references
//! - [`RenderOutput`] — Framework-agnostic render description

pub mod aria;
pub mod callback;
pub mod component;
pub mod event;
pub mod focus;
pub mod id;
pub mod props;
pub mod render;
pub mod security;
pub mod state;

pub use aria::{
    AriaAttributes, AriaAutocomplete, AriaCurrent, AriaHasPopup, AriaLive, AriaRole, AriaSort,
    Orientation, TriState,
};
pub use callback::{ActionCallback, BoolCallback, Callback, IndexCallback, StringCallback};
pub use component::Component;
pub use event::{ComponentEvent, EventResult, Key, ModifierKeys, MouseButton};
pub use focus::{FocusInstruction, FocusManager, FocusStrategy};
pub use id::IdGenerator;
pub use props::Props;
pub use render::{AttrValue, ChildrenSpec, RenderOutput};
pub use state::State;
