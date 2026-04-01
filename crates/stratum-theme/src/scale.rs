//! Scale types for spacing, radii, shadows, transitions, z-indices, and breakpoints.

use serde::{Deserialize, Serialize};

use crate::token::{Px, Rem};

/// Spacing scale following Tailwind-like conventions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacingScale {
    pub s0: Rem,
    pub s1: Rem,
    pub s2: Rem,
    pub s3: Rem,
    pub s4: Rem,
    pub s5: Rem,
    pub s6: Rem,
    pub s8: Rem,
    pub s10: Rem,
    pub s12: Rem,
    pub s16: Rem,
    pub s20: Rem,
    pub s24: Rem,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            s0: Rem(0.0),
            s1: Rem(0.25),
            s2: Rem(0.5),
            s3: Rem(0.75),
            s4: Rem(1.0),
            s5: Rem(1.25),
            s6: Rem(1.5),
            s8: Rem(2.0),
            s10: Rem(2.5),
            s12: Rem(3.0),
            s16: Rem(4.0),
            s20: Rem(5.0),
            s24: Rem(6.0),
        }
    }
}

/// Border-radius scale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadiiScale {
    pub none: Rem,
    pub sm: Rem,
    pub md: Rem,
    pub lg: Rem,
    pub xl: Rem,
    pub full: Rem,
}

impl Default for RadiiScale {
    fn default() -> Self {
        Self {
            none: Rem(0.0),
            sm: Rem(0.125),
            md: Rem(0.375),
            lg: Rem(0.5),
            xl: Rem(0.75),
            full: Rem(9999.0),
        }
    }
}

/// Box-shadow scale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowScale {
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)".into(),
            md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".into(),
            lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".into(),
            xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".into(),
        }
    }
}

/// Transition duration scale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionScale {
    pub fast: String,
    pub normal: String,
    pub slow: String,
}

impl Default for TransitionScale {
    fn default() -> Self {
        Self {
            fast: "150ms cubic-bezier(0.4, 0, 0.2, 1)".into(),
            normal: "200ms cubic-bezier(0.4, 0, 0.2, 1)".into(),
            slow: "300ms cubic-bezier(0.4, 0, 0.2, 1)".into(),
        }
    }
}

/// Z-index scale for common layering contexts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZIndexScale {
    pub dropdown: i32,
    pub sticky: i32,
    pub modal: i32,
    pub popover: i32,
    pub tooltip: i32,
}

impl Default for ZIndexScale {
    fn default() -> Self {
        Self {
            dropdown: 1000,
            sticky: 1020,
            modal: 1050,
            popover: 1060,
            tooltip: 1070,
        }
    }
}

/// Responsive breakpoint scale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakpointScale {
    pub sm: Px,
    pub md: Px,
    pub lg: Px,
    pub xl: Px,
    pub xxl: Px,
}

impl Default for BreakpointScale {
    fn default() -> Self {
        Self {
            sm: Px(640.0),
            md: Px(768.0),
            lg: Px(1024.0),
            xl: Px(1280.0),
            xxl: Px(1536.0),
        }
    }
}
