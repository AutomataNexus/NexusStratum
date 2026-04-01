/// Input sanitizer for form components.
///
/// Provides configurable sanitization policies for user input
/// beyond the basic HTML escaping in `stratum-core`.
#[derive(Debug, Clone)]
pub struct Sanitizer {
    /// Strip all HTML tags.
    pub strip_tags: bool,
    /// Maximum input length (0 = unlimited).
    pub max_length: usize,
    /// Trim whitespace from start and end.
    pub trim: bool,
    /// Collapse multiple whitespace characters to single spaces.
    pub collapse_whitespace: bool,
    /// Allowed characters regex pattern (empty = allow all after other rules).
    pub allowed_pattern: Option<String>,
    /// Strip null bytes.
    pub strip_null_bytes: bool,
    /// Strip control characters (except newline, tab).
    pub strip_control_chars: bool,
}

impl Default for Sanitizer {
    fn default() -> Self {
        Self {
            strip_tags: true,
            max_length: 0,
            trim: true,
            collapse_whitespace: false,
            allowed_pattern: None,
            strip_null_bytes: true,
            strip_control_chars: true,
        }
    }
}

impl Sanitizer {
    /// Create a sanitizer with default settings (strip tags, trim, strip control chars).
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a strict sanitizer (all protections enabled).
    pub fn strict() -> Self {
        Self {
            strip_tags: true,
            max_length: 10000,
            trim: true,
            collapse_whitespace: true,
            allowed_pattern: None,
            strip_null_bytes: true,
            strip_control_chars: true,
        }
    }

    /// Create a permissive sanitizer (minimal sanitization).
    pub fn permissive() -> Self {
        Self {
            strip_tags: false,
            max_length: 0,
            trim: false,
            collapse_whitespace: false,
            allowed_pattern: None,
            strip_null_bytes: true,
            strip_control_chars: false,
        }
    }

    /// Set maximum length.
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self
    }

    /// Enable whitespace collapsing.
    pub fn with_collapse_whitespace(mut self) -> Self {
        self.collapse_whitespace = true;
        self
    }

    /// Sanitize an input string according to the configured policy.
    pub fn sanitize(&self, input: &str) -> String {
        let mut result = input.to_string();

        // Strip null bytes first (security critical)
        if self.strip_null_bytes {
            result = result.replace('\0', "");
        }

        // Strip control characters
        if self.strip_control_chars {
            result = result
                .chars()
                .filter(|c| !c.is_control() || *c == '\n' || *c == '\t' || *c == '\r')
                .collect();
        }

        // Strip HTML tags
        if self.strip_tags {
            result = strip_html_tags(&result);
        }

        // Trim whitespace
        if self.trim {
            result = result.trim().to_string();
        }

        // Collapse whitespace
        if self.collapse_whitespace {
            result = collapse_spaces(&result);
        }

        // Enforce max length
        if self.max_length > 0 && result.len() > self.max_length {
            // Truncate at char boundary
            result = result.chars().take(self.max_length).collect();
        }

        result
    }
}

/// Strip HTML tags from a string.
fn strip_html_tags(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;

    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
}

/// Collapse multiple whitespace characters into single spaces.
fn collapse_spaces(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut prev_was_space = false;

    for ch in input.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
            }
            prev_was_space = true;
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_tags() {
        let s = Sanitizer::new();
        assert_eq!(s.sanitize("<script>alert('xss')</script>"), "alert('xss')");
        assert_eq!(s.sanitize("<b>bold</b>"), "bold");
        assert_eq!(s.sanitize("no tags here"), "no tags here");
    }

    #[test]
    fn sanitize_strips_nested_tags() {
        let s = Sanitizer::new();
        assert_eq!(
            s.sanitize("<div><p>hello</p></div>"),
            "hello"
        );
    }

    #[test]
    fn sanitize_trims() {
        let s = Sanitizer::new();
        assert_eq!(s.sanitize("  hello  "), "hello");
    }

    #[test]
    fn sanitize_strips_null_bytes() {
        let s = Sanitizer::new();
        assert_eq!(s.sanitize("he\0llo"), "hello");
    }

    #[test]
    fn sanitize_strips_control_chars() {
        let s = Sanitizer::new();
        // Tab and newline should be preserved
        assert_eq!(s.sanitize("hello\tworld"), "hello\tworld");
        assert_eq!(s.sanitize("hello\nworld"), "hello\nworld");
        // Other control chars stripped
        assert_eq!(s.sanitize("hello\x07world"), "helloworld");
    }

    #[test]
    fn sanitize_max_length() {
        let s = Sanitizer::new().with_max_length(5);
        assert_eq!(s.sanitize("hello world"), "hello");
    }

    #[test]
    fn sanitize_collapse_whitespace() {
        let s = Sanitizer::new().with_collapse_whitespace();
        assert_eq!(s.sanitize("hello   world"), "hello world");
        assert_eq!(s.sanitize("hello\n\n\nworld"), "hello world");
    }

    #[test]
    fn sanitize_strict() {
        let s = Sanitizer::strict();
        let result = s.sanitize("  <script>alert('xss')</script>  hello   world  ");
        assert_eq!(result, "alert('xss') hello world");
    }

    #[test]
    fn sanitize_permissive() {
        let s = Sanitizer::permissive();
        let result = s.sanitize("  <b>hello</b>  ");
        assert_eq!(result, "  <b>hello</b>  ");
    }

    #[test]
    fn strip_html_tags_function() {
        assert_eq!(strip_html_tags("<a href=\"x\">link</a>"), "link");
        assert_eq!(strip_html_tags("no tags"), "no tags");
        assert_eq!(strip_html_tags("<>empty</>"), "empty");
    }

    #[test]
    fn collapse_spaces_function() {
        assert_eq!(collapse_spaces("a  b  c"), "a b c");
        assert_eq!(collapse_spaces("a\t\tb"), "a b");
        assert_eq!(collapse_spaces("abc"), "abc");
    }

    #[test]
    fn sanitize_unicode_safe() {
        let s = Sanitizer::new().with_max_length(5);
        // Should not break in the middle of a multi-byte character
        let result = s.sanitize("helloéàü");
        assert_eq!(result.chars().count(), 5);
    }
}
