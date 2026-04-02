use crate::csp::CspNonce;

/// Security response headers for SSR applications.
///
/// Generates recommended security headers for responses that include
/// NexusStratum components.
#[derive(Debug, Clone)]
pub struct SecurityHeaders {
    /// CSP nonce for style injection.
    pub csp_nonce: Option<CspNonce>,
    /// Whether to include X-Frame-Options.
    pub frame_options: FrameOptions,
    /// Whether to include X-Content-Type-Options: nosniff.
    pub nosniff: bool,
    /// Referrer policy.
    pub referrer_policy: ReferrerPolicy,
    /// Permissions-Policy directives (e.g., `"camera=()"`, `"microphone=()"`).
    pub permissions_policy: Vec<String>,
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self {
            csp_nonce: None,
            frame_options: FrameOptions::Deny,
            nosniff: true,
            referrer_policy: ReferrerPolicy::StrictOriginWhenCrossOrigin,
            permissions_policy: vec![],
        }
    }
}

impl SecurityHeaders {
    /// Create security headers with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a CSP nonce for style injection.
    pub fn with_nonce(mut self, nonce: CspNonce) -> Self {
        self.csp_nonce = Some(nonce);
        self
    }

    /// Set frame options.
    pub fn with_frame_options(mut self, options: FrameOptions) -> Self {
        self.frame_options = options;
        self
    }

    /// Add a Permissions-Policy directive.
    ///
    /// Example: `headers.with_permissions_policy("camera=()")`.
    pub fn with_permissions_policy(mut self, directive: impl Into<String>) -> Self {
        self.permissions_policy.push(directive.into());
        self
    }

    /// Generate all security headers as key-value pairs.
    pub fn to_header_pairs(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();

        // Content-Security-Policy
        let style_src = match &self.csp_nonce {
            Some(nonce) => format!("style-src 'self' {}", nonce.csp_directive()),
            None => "style-src 'self' 'unsafe-inline'".to_string(),
        };
        headers.push((
            "Content-Security-Policy".to_string(),
            format!(
                "default-src 'self'; script-src 'self'; {}; img-src 'self' data:; font-src 'self'",
                style_src
            ),
        ));

        // X-Frame-Options
        headers.push((
            "X-Frame-Options".to_string(),
            self.frame_options.as_str().to_string(),
        ));

        // X-Content-Type-Options
        if self.nosniff {
            headers.push((
                "X-Content-Type-Options".to_string(),
                "nosniff".to_string(),
            ));
        }

        // Referrer-Policy
        headers.push((
            "Referrer-Policy".to_string(),
            self.referrer_policy.as_str().to_string(),
        ));

        // X-XSS-Protection: 0 — XSS Auditor is deprecated in all modern browsers.
        // Setting to 0 disables it, which is the current OWASP recommendation.
        headers.push(("X-XSS-Protection".to_string(), "0".to_string()));

        // Permissions-Policy
        if !self.permissions_policy.is_empty() {
            headers.push((
                "Permissions-Policy".to_string(),
                self.permissions_policy.join(", "),
            ));
        }

        headers
    }
}

/// X-Frame-Options values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameOptions {
    Deny,
    SameOrigin,
}

impl FrameOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Deny => "DENY",
            Self::SameOrigin => "SAMEORIGIN",
        }
    }
}

/// Referrer-Policy values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
}

impl ReferrerPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoReferrer => "no-referrer",
            Self::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            Self::Origin => "origin",
            Self::OriginWhenCrossOrigin => "origin-when-cross-origin",
            Self::SameOrigin => "same-origin",
            Self::StrictOrigin => "strict-origin",
            Self::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_headers() {
        let headers = SecurityHeaders::new();
        let pairs = headers.to_header_pairs();

        assert!(pairs.iter().any(|(k, _)| k == "Content-Security-Policy"));
        assert!(pairs.iter().any(|(k, _)| k == "X-Frame-Options"));
        assert!(pairs.iter().any(|(k, _)| k == "X-Content-Type-Options"));
    }

    #[test]
    fn headers_with_nonce() {
        let nonce = CspNonce::new("test-nonce-123");
        let headers = SecurityHeaders::new().with_nonce(nonce);
        let pairs = headers.to_header_pairs();

        let csp = pairs.iter().find(|(k, _)| k == "Content-Security-Policy").unwrap();
        assert!(csp.1.contains("nonce-test-nonce-123"));
        assert!(!csp.1.contains("unsafe-inline"));
    }

    #[test]
    fn headers_without_nonce_allows_unsafe_inline() {
        let headers = SecurityHeaders::new();
        let pairs = headers.to_header_pairs();

        let csp = pairs.iter().find(|(k, _)| k == "Content-Security-Policy").unwrap();
        assert!(csp.1.contains("unsafe-inline"));
    }

    #[test]
    fn xss_protection_disabled() {
        let headers = SecurityHeaders::new();
        let pairs = headers.to_header_pairs();
        let xss = pairs.iter().find(|(k, _)| k == "X-XSS-Protection").unwrap();
        assert_eq!(xss.1, "0");
    }

    #[test]
    fn permissions_policy_emitted() {
        let headers = SecurityHeaders::new()
            .with_permissions_policy("camera=()")
            .with_permissions_policy("microphone=()");
        let pairs = headers.to_header_pairs();
        let pp = pairs.iter().find(|(k, _)| k == "Permissions-Policy").unwrap();
        assert_eq!(pp.1, "camera=(), microphone=()");
    }

    #[test]
    fn permissions_policy_omitted_when_empty() {
        let headers = SecurityHeaders::new();
        let pairs = headers.to_header_pairs();
        assert!(!pairs.iter().any(|(k, _)| k == "Permissions-Policy"));
    }

    #[test]
    fn frame_options() {
        assert_eq!(FrameOptions::Deny.as_str(), "DENY");
        assert_eq!(FrameOptions::SameOrigin.as_str(), "SAMEORIGIN");
    }
}
