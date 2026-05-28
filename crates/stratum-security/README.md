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

// CSRF protection — synchronizer-token pattern (server stores the token):
let token = CsrfToken::generate();
let is_valid = token.validate(&submitted_token);

// Time-based CSRF — stateless HMAC-signed tokens (OWASP HMAC-based pattern).
// The issue time is signed into the token, so it can't be tampered with, and
// stale tokens are rejected without any server-side session storage.
use stratum_security::{CsrfKey, CsrfError};
use stratum_security::csrf::DEFAULT_MAX_AGE;

// Once per app: generate the secret and persist it (env/secret manager).
let key = CsrfKey::generate();        // or CsrfKey::from_secret(loaded_bytes)

let token = key.issue();              // embed token.value() in the form
match key.verify(&submitted_token, DEFAULT_MAX_AGE) {
    Ok(())                       => { /* accept */ }
    Err(CsrfError::Expired)      => { /* token too old — re-issue */ }
    Err(CsrfError::BadSignature) => { /* forged/tampered — reject + log */ }
    Err(CsrfError::Malformed)    => { /* not a token — reject */ }
}

// For authenticated users, bind the token to the session so it can't be
// replayed under a different user. The session id is signed in, not stored
// in the token; supply the same id at verify time.
let token = key.issue_for(session_id_bytes);
let ok = key.verify_for(&submitted_token, session_id_bytes, DEFAULT_MAX_AGE).is_ok();
```

The HMAC **secret** is one long-lived per-deployment value — generate it once with
`CsrfKey::generate()` and load it from your env/secret manager via
`CsrfKey::from_secret(..)`. It is never committed to the repo: this code is
open source, but each deployment supplies its own secret. Every user gets a
unique token (random nonce per `issue`); you do **not** create a key per user.

## How to Run Tests

```bash
cargo test -p stratum-security
```
