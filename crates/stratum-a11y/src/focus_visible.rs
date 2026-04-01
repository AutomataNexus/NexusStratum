//! Focus visible detection utilities.
//!
//! Provides strategies for determining when focus indicators should be shown,
//! based on the user's input modality.

/// The input modality currently being used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputModality {
    /// User is navigating with a keyboard.
    Keyboard,
    /// User is navigating with a pointer (mouse, touch).
    Pointer,
    /// User is navigating with a screen reader or other virtual input.
    Virtual,
}

/// Describes when focus indicators should be displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FocusVisibleStrategy {
    /// Show focus indicator when input modality is keyboard.
    pub show_on_keyboard: bool,
    /// Show focus indicator when input modality is pointer.
    pub show_on_pointer: bool,
}

impl FocusVisibleStrategy {
    /// Only show focus indicators for keyboard users.
    ///
    /// This matches the browser's `:focus-visible` behavior.
    pub fn keyboard_only() -> Self {
        Self {
            show_on_keyboard: true,
            show_on_pointer: false,
        }
    }

    /// Always show focus indicators regardless of input modality.
    pub fn always() -> Self {
        Self {
            show_on_keyboard: true,
            show_on_pointer: true,
        }
    }

    /// Returns whether the focus indicator should be visible for the given modality.
    pub fn should_show(&self, modality: InputModality) -> bool {
        match modality {
            InputModality::Keyboard | InputModality::Virtual => self.show_on_keyboard,
            InputModality::Pointer => self.show_on_pointer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_only_shows_on_keyboard() {
        let strategy = FocusVisibleStrategy::keyboard_only();
        assert!(strategy.should_show(InputModality::Keyboard));
    }

    #[test]
    fn keyboard_only_hides_on_pointer() {
        let strategy = FocusVisibleStrategy::keyboard_only();
        assert!(!strategy.should_show(InputModality::Pointer));
    }

    #[test]
    fn keyboard_only_shows_on_virtual() {
        let strategy = FocusVisibleStrategy::keyboard_only();
        assert!(strategy.should_show(InputModality::Virtual));
    }

    #[test]
    fn always_shows_on_keyboard() {
        let strategy = FocusVisibleStrategy::always();
        assert!(strategy.should_show(InputModality::Keyboard));
    }

    #[test]
    fn always_shows_on_pointer() {
        let strategy = FocusVisibleStrategy::always();
        assert!(strategy.should_show(InputModality::Pointer));
    }

    #[test]
    fn always_shows_on_virtual() {
        let strategy = FocusVisibleStrategy::always();
        assert!(strategy.should_show(InputModality::Virtual));
    }

    #[test]
    fn keyboard_only_fields() {
        let strategy = FocusVisibleStrategy::keyboard_only();
        assert!(strategy.show_on_keyboard);
        assert!(!strategy.show_on_pointer);
    }

    #[test]
    fn always_fields() {
        let strategy = FocusVisibleStrategy::always();
        assert!(strategy.show_on_keyboard);
        assert!(strategy.show_on_pointer);
    }
}
