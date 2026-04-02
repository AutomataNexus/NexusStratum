//! # stratum-dioxus
//!
//! Dioxus framework adapter for NexusStratum.
//!
//! **Status: Phase 2** -- This crate is scaffolded but not yet implemented.
//! The Dioxus adapter will mirror the structure of `stratum-leptos`, providing
//! idiomatic Dioxus component APIs wrapping the headless primitives from
//! `stratum-primitives` with styled defaults from `stratum-components`.
//!
//! ## Planned modules
//!
//! - [`adapter`] -- Converts `stratum-core` render output to Dioxus virtual DOM nodes
//! - `provider` -- Dioxus-specific context providers (ThemeProvider, ToasterProvider)
//!
//! ## Dependencies
//!
//! Only `stratum-core` is required at this stage. Additional dependencies
//! (`dioxus`, `stratum-components`, `stratum-theme`, `stratum-icons`) will be
//! added when implementation begins.

pub mod adapter;
