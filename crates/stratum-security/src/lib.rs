//! # stratum-security
//!
//! Security hardening for NexusStratum applications.
//!
//! While `stratum-core` provides built-in XSS prevention and HTML escaping
//! for all components, this crate provides additional hardened security
//! features for production deployments:
//!
//! - **CSP nonce management** — Content Security Policy compliant style injection
//! - **Input sanitization** — Configurable sanitization for form inputs
//! - **Security headers** — SSR response header helpers
//! - **CSRF protection** — Token generation and validation for forms
//! - **Subresource integrity** — SRI hash generation for external resources

pub mod csp;
pub mod csrf;
pub mod headers;
pub mod sanitize;

pub use csp::CspNonce;
pub use csrf::CsrfToken;
pub use headers::SecurityHeaders;
pub use sanitize::Sanitizer;
