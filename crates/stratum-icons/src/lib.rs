//! # stratum-icons
//!
//! Icon library for NexusStratum. Ships with a comprehensive set of icons
//! from the Lucide icon set (MIT license), compiled to inline SVG strings.
//!
//! All icons are embedded in the binary — zero external HTTP requests.

pub mod icon;
pub mod lucide;

pub use icon::{Icon, IconProps};
