use std::fmt;

/// CSRF token for form submissions.
///
/// Generates and validates tokens to prevent Cross-Site Request Forgery
/// attacks on forms submitted by NexusStratum form components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsrfToken {
    value: String,
}

impl CsrfToken {
    /// Create a token from a known value (e.g., from server session).
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Generate a new cryptographically secure CSRF token.
    ///
    /// Uses the OS CSPRNG via `getrandom`.
    pub fn generate() -> Self {
        let token = format!("csrf_{}", crate::csp::generate_secure_token(32));
        Self { value: token }
    }

    /// Get the token value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Generate a hidden input HTML string for embedding in forms.
    pub fn to_hidden_input(&self) -> String {
        format!(
            "<input type=\"hidden\" name=\"_csrf\" value=\"{}\" />",
            stratum_core::security::escape_attr(&self.value)
        )
    }

    /// Validate that a submitted token matches this token.
    ///
    /// Uses constant-time comparison to prevent timing attacks.
    pub fn validate(&self, submitted: &str) -> bool {
        constant_time_eq(self.value.as_bytes(), submitted.as_bytes())
    }
}

impl fmt::Display for CsrfToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Constant-time byte comparison to prevent timing attacks.
///
/// Both the content and the length comparison are constant-time.
/// The longer slice is always fully iterated to prevent length leakage.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len_eq = a.len() == b.len();
    // Always iterate the longer of the two to prevent length-based timing
    let max_len = a.len().max(b.len());
    let mut result = 0u8;
    for i in 0..max_len {
        let x = if i < a.len() { a[i] } else { 0 };
        let y = if i < b.len() { b[i] } else { 0 };
        result |= x ^ y;
    }
    result == 0 && len_eq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csrf_token_validation() {
        let token = CsrfToken::new("secret-token-123");
        assert!(token.validate("secret-token-123"));
        assert!(!token.validate("wrong-token"));
        assert!(!token.validate("secret-token-12"));
    }

    #[test]
    fn csrf_token_hidden_input() {
        let token = CsrfToken::new("abc123");
        let html = token.to_hidden_input();
        assert!(html.contains("type=\"hidden\""));
        assert!(html.contains("name=\"_csrf\""));
        assert!(html.contains("value=\"abc123\""));
    }

    #[test]
    fn csrf_token_generate_unique() {
        let a = CsrfToken::generate();
        let b = CsrfToken::generate();
        assert_ne!(a, b);
        assert!(a.value().starts_with("csrf_"));
    }

    #[test]
    fn csrf_token_generate_sufficient_entropy() {
        let token = CsrfToken::generate();
        // "csrf_" prefix + 64 hex chars (32 bytes)
        assert!(token.value().len() >= 69);
    }

    #[test]
    fn csrf_token_xss_prevention() {
        let token = CsrfToken::new("test\" onclick=\"alert(1)");
        let html = token.to_hidden_input();
        // Quotes are escaped so the injected attribute can't break out of the value
        assert!(html.contains("&quot;"));
        assert!(!html.contains("\" onclick=\""));
    }

    #[test]
    fn constant_time_eq_works() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
    }
}
