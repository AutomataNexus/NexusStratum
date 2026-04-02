/// Input sanitizer for form components.
///
/// Provides configurable sanitization policies for user input
/// beyond the basic HTML escaping in `stratum-core`.
#[derive(Debug, Clone)]
pub struct Sanitizer {
    /// Strip all HTML tags.
    pub strip_tags: bool,
    /// Maximum input length in characters (0 = unlimited).
    pub max_length: usize,
    /// Trim whitespace from start and end.
    pub trim: bool,
    /// Collapse multiple whitespace characters to single spaces.
    pub collapse_whitespace: bool,
    /// Strip null bytes.
    pub strip_null_bytes: bool,
    /// Strip control characters (except newline `\n`, tab `\t`, and carriage return `\r`).
    pub strip_control_chars: bool,
}

impl Default for Sanitizer {
    fn default() -> Self {
        Self {
            strip_tags: true,
            max_length: 0,
            trim: true,
            collapse_whitespace: false,
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

    /// Create a strict sanitizer (all protections enabled, 10k char limit).
    pub fn strict() -> Self {
        Self {
            strip_tags: true,
            max_length: 10000,
            trim: true,
            collapse_whitespace: true,
            strip_null_bytes: true,
            strip_control_chars: true,
        }
    }

    /// Create a permissive sanitizer (minimal sanitization — only null bytes stripped).
    pub fn permissive() -> Self {
        Self {
            strip_tags: false,
            max_length: 0,
            trim: false,
            collapse_whitespace: false,
            strip_null_bytes: true,
            strip_control_chars: false,
        }
    }

    /// Set maximum length in characters.
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

        // Strip control characters (preserve \n, \t, \r)
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

        // Enforce max length (measured in characters, not bytes)
        if self.max_length > 0 && result.chars().count() > self.max_length {
            result = result.chars().take(self.max_length).collect();
        }

        result
    }
}

/// Strip HTML tags from a string.
///
/// Handles malformed tags (unclosed `<`) by treating everything after
/// an unmatched `<` as inside a tag until end-of-string. This is the
/// safe default — it errs on the side of stripping too much rather
/// than allowing potentially dangerous content through.
///
/// Note: this is a basic state-machine approach. For untrusted HTML with
/// complex payloads (encoded entities, nested attributes with angle brackets),
/// use a dedicated HTML sanitizer library like `ammonia`.
fn strip_html_tags(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;
    let mut in_attr_quote: Option<char> = None;

    for ch in input.chars() {
        if in_tag {
            // Track quoted attribute values so > inside quotes doesn't end the tag
            match in_attr_quote {
                Some(q) if ch == q => in_attr_quote = None,
                Some(_) => {}
                None if ch == '"' || ch == '\'' => in_attr_quote = Some(ch),
                None if ch == '>' => {
                    in_tag = false;
                    in_attr_quote = None;
                }
                _ => {}
            }
        } else if ch == '<' {
            in_tag = true;
            in_attr_quote = None;
        } else {
            result.push(ch);
        }
    }
    // If we ended inside a tag (unclosed <), everything after the < is stripped.
    // This is intentional — unclosed tags are suspicious.

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
        assert_eq!(s.sanitize("<div><p>hello</p></div>"), "hello");
    }

    #[test]
    fn sanitize_strips_unclosed_tags() {
        let s = Sanitizer::new();
        // Unclosed tag — everything after < is stripped (safe default)
        assert_eq!(s.sanitize("hello<img src=x onerror=alert(1)"), "hello");
    }

    #[test]
    fn sanitize_handles_quotes_in_attrs() {
        let s = Sanitizer::new();
        // > inside a quoted attribute should not end the tag
        assert_eq!(s.sanitize("<a href=\"x>y\">link</a>"), "link");
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
        // Tab, newline, and carriage return should be preserved
        assert_eq!(s.sanitize("hello\tworld"), "hello\tworld");
        assert_eq!(s.sanitize("hello\nworld"), "hello\nworld");
        assert_eq!(s.sanitize("hello\rworld"), "hello\rworld");
        // Other control chars stripped
        assert_eq!(s.sanitize("hello\x07world"), "helloworld");
    }

    #[test]
    fn sanitize_max_length_counts_chars() {
        let s = Sanitizer::new().with_max_length(5);
        assert_eq!(s.sanitize("hello world"), "hello");
        // Multi-byte: 5 characters, not 5 bytes
        assert_eq!(s.sanitize("héllo world"), "héllo");
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
        let result = s.sanitize("helloéàü");
        assert_eq!(result.chars().count(), 5);
    }
}
