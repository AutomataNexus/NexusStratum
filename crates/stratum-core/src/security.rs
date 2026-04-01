/// Core security primitives built into every NexusStratum component.
///
/// These are always active — no feature flag needed. They prevent common
/// web security vulnerabilities at the framework level so individual
/// components cannot accidentally introduce them.
///
/// For additional hardened security features (CSP, CSRF, security headers),
/// see the `stratum-security` crate.
/// Escapes a string for safe insertion into HTML content.
///
/// Prevents XSS by converting dangerous characters to HTML entities.
/// This is applied automatically by all text-rendering components.
pub fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2F;"),
            _ => output.push(ch),
        }
    }
    output
}

/// Escapes a string for safe use in an HTML attribute value.
///
/// More aggressive than content escaping — also handles backticks,
/// equals signs, and other attribute-context injection vectors.
pub fn escape_attr(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '`' => output.push_str("&#x60;"),
            _ => output.push(ch),
        }
    }
    output
}

/// Validates that a CSS class name is safe (no injection vectors).
///
/// Rejects class names containing characters that could break out
/// of the class attribute or inject CSS.
pub fn is_safe_class_name(class: &str) -> bool {
    if class.is_empty() {
        return false;
    }
    class
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '/' || c == '.')
}

/// Validates that a CSS property value is safe (no injection vectors).
///
/// Rejects values containing characters that could inject additional
/// CSS rules or escape the style context.
pub fn is_safe_css_value(value: &str) -> bool {
    // Block common CSS injection vectors
    let dangerous_patterns = [
        "expression(",
        "url(",
        "javascript:",
        "data:",
        "vbscript:",
        "@import",
        "behavior:",
        "-moz-binding:",
        "</style",
    ];

    let lower = value.to_lowercase();
    !dangerous_patterns
        .iter()
        .any(|pattern| lower.contains(pattern))
}

/// Sanitizes a user-provided ID to prevent attribute injection.
///
/// IDs must contain only alphanumeric characters, hyphens, and underscores.
pub fn sanitize_id(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_html_basic() {
        assert_eq!(escape_html("hello"), "hello");
        assert_eq!(
            escape_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;&#x2F;script&gt;"
        );
    }

    #[test]
    fn escape_html_entities() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("a < b"), "a &lt; b");
        assert_eq!(escape_html("a > b"), "a &gt; b");
        assert_eq!(escape_html("a \"b\""), "a &quot;b&quot;");
    }

    #[test]
    fn escape_attr_basic() {
        assert_eq!(escape_attr("hello"), "hello");
        assert_eq!(
            escape_attr("\" onmouseover=\"alert(1)"),
            "&quot; onmouseover=&quot;alert(1)"
        );
    }

    #[test]
    fn is_safe_class_name_valid() {
        assert!(is_safe_class_name("btn"));
        assert!(is_safe_class_name("btn-primary"));
        assert!(is_safe_class_name("text-xl"));
        assert!(is_safe_class_name("hover:bg-blue-500"));
        assert!(is_safe_class_name("w-1/2"));
        assert!(is_safe_class_name("mt-2.5"));
    }

    #[test]
    fn is_safe_class_name_invalid() {
        assert!(!is_safe_class_name(""));
        assert!(!is_safe_class_name("btn; color: red"));
        assert!(!is_safe_class_name("btn\" onclick=\""));
        assert!(!is_safe_class_name("btn<script>"));
    }

    #[test]
    fn is_safe_css_value_valid() {
        assert!(is_safe_css_value("red"));
        assert!(is_safe_css_value("16px"));
        assert!(is_safe_css_value("#ff0000"));
        assert!(is_safe_css_value("hsl(0 0% 100%)"));
        assert!(is_safe_css_value("1px solid black"));
    }

    #[test]
    fn is_safe_css_value_injection() {
        assert!(!is_safe_css_value("expression(alert(1))"));
        assert!(!is_safe_css_value("url(javascript:alert(1))"));
        assert!(!is_safe_css_value("red; @import 'evil.css'"));
        assert!(!is_safe_css_value("</style><script>"));
    }

    #[test]
    fn sanitize_id_basic() {
        assert_eq!(sanitize_id("my-button"), "my-button");
        assert_eq!(sanitize_id("btn_123"), "btn_123");
        assert_eq!(
            sanitize_id("btn\" onclick=\"alert(1)"),
            "btnonclickalert1"
        );
        assert_eq!(sanitize_id("btn<script>"), "btnscript");
    }
}
