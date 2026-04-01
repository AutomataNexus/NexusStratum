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
    /// Permissions policy directives.
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

        // X-XSS-Protection (legacy but still useful)
        headers.push((
            "X-XSS-Protection".to_string(),
            "1; mode=block".to_string(),
        ));

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

        let has_csp = pairs.iter().any(|(k, _)| k == "Content-Security-Policy");
        let has_frame = pairs.iter().any(|(k, _)| k == "X-Frame-Options");
        let has_nosniff = pairs.iter().any(|(k, _)| k == "X-Content-Type-Options");

        assert!(has_csp);
        assert!(has_frame);
        assert!(has_nosniff);
    }

    #[test]
    fn headers_with_nonce() {
        let nonce = CspNonce::new("test-nonce-123");
        let headers = SecurityHeaders::new().with_nonce(nonce);
        let pairs = headers.to_header_pairs();

        let csp = pairs
            .iter()
            .find(|(k, _)| k == "Content-Security-Policy")
            .unwrap();
        assert!(csp.1.contains("nonce-test-nonce-123"));
        assert!(!csp.1.contains("unsafe-inline"));
    }

    #[test]
    fn headers_without_nonce_allows_unsafe_inline() {
        let headers = SecurityHeaders::new();
        let pairs = headers.to_header_pairs();

        let csp = pairs
            .iter()
            .find(|(k, _)| k == "Content-Security-Policy")
            .unwrap();
        assert!(csp.1.contains("unsafe-inline"));
    }

    #[test]
    fn frame_options() {
        assert_eq!(FrameOptions::Deny.as_str(), "DENY");
        assert_eq!(FrameOptions::SameOrigin.as_str(), "SAMEORIGIN");
    }
}
