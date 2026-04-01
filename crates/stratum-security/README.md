# stratum-security

## Purpose

Security hardening utilities including HTML sanitization, CSP nonce injection, security headers, and CSRF protection.

## Position in Pipeline

```
        stratum-core
             |
       stratum-security
             |
       stratum-components (optional dep)
             |
        +----+----+
        |         |
   stratum-   stratum-
   leptos     dioxus
```

Depends on: `stratum-core`
Optional dependency for: `stratum-components`

## Key Public API

| Item | Description |
|------|-------------|
| `Sanitizer` | Sanitize user-provided HTML to prevent XSS |
| `CspNonce` | Generate and inject Content Security Policy nonces |
| `SecurityHeaders` | Produce recommended security response headers |
| `CsrfToken` | Generate and validate CSRF tokens |

## Usage Example

```rust
use stratum_security::{Sanitizer, CspNonce, SecurityHeaders, CsrfToken};

// Sanitize user input before rendering
let safe_html = Sanitizer::new()
    .allow_tags(&["b", "i", "a"])
    .sanitize(user_input);

// Generate a CSP nonce for inline scripts
let nonce = CspNonce::generate();
let header = format!("script-src 'nonce-{}'", nonce);

// Produce all recommended security headers
let headers = SecurityHeaders::strict();

// CSRF protection
let token = CsrfToken::generate();
let is_valid = CsrfToken::validate(&token, &submitted_token);
```

## How to Run Tests

```bash
cargo test -p stratum-security
```
