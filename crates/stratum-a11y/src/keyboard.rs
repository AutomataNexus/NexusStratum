//! Keyboard navigation utilities implementing ARIA APG keyboard patterns.

use stratum_core::{Key, Orientation};

/// Strategy for keyboard navigation within a composite widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavStrategy {
    /// Arrow keys navigate between items.
    Arrow,
    /// Tab key navigates between items.
    Tab,
    /// Both arrow keys and tab key navigate.
    Both,
}

/// Keyboard navigation handler for composite widgets (tablists, menus, listboxes, etc.).
///
/// Implements ARIA Authoring Practices Guide (APG) keyboard interaction patterns.
#[derive(Debug, Clone)]
pub struct KeyboardNav {
    /// Orientation of the widget (horizontal or vertical).
    pub orientation: Orientation,
    /// Whether focus wraps around at boundaries.
    pub loop_focus: bool,
    /// Which keys are used for navigation.
    pub strategy: NavStrategy,
}

impl KeyboardNav {
    /// Create a new `KeyboardNav` with the given orientation and strategy.
    /// `loop_focus` defaults to `false`.
    pub fn new(orientation: Orientation, strategy: NavStrategy) -> Self {
        Self {
            orientation,
            loop_focus: false,
            strategy,
        }
    }

    /// Set whether focus wraps around at boundaries.
    pub fn with_loop(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    /// Handle a key press and return the new focused index.
    ///
    /// Given the current key, total number of items, and the currently focused index,
    /// returns `Some(new_index)` if the key was handled, or `None` if the key is not
    /// relevant to this navigation configuration.
    pub fn handle(&self, key: &Key, items_count: usize, current: usize) -> Option<usize> {
        if items_count == 0 {
            return None;
        }

        match key {
            Key::Home => Some(0),
            Key::End => Some(items_count.saturating_sub(1)),
            _ => {
                let direction = self.key_direction(key)?;
                Some(self.move_index(current, items_count, direction))
            }
        }
    }

    /// Map a key to a direction (-1 for prev, +1 for next), or None if not handled.
    fn key_direction(&self, key: &Key) -> Option<i8> {
        match self.strategy {
            NavStrategy::Arrow => self.arrow_direction(key),
            NavStrategy::Tab => self.tab_direction(key),
            NavStrategy::Both => self
                .arrow_direction(key)
                .or_else(|| self.tab_direction(key)),
        }
    }

    fn arrow_direction(&self, key: &Key) -> Option<i8> {
        match (self.orientation, key) {
            (Orientation::Horizontal, Key::ArrowLeft) => Some(-1),
            (Orientation::Horizontal, Key::ArrowRight) => Some(1),
            (Orientation::Vertical, Key::ArrowUp) => Some(-1),
            (Orientation::Vertical, Key::ArrowDown) => Some(1),
            _ => None,
        }
    }

    fn tab_direction(&self, key: &Key) -> Option<i8> {
        match key {
            Key::Tab => Some(1),
            _ => None,
        }
    }

    fn move_index(&self, current: usize, count: usize, direction: i8) -> usize {
        if direction > 0 {
            // Next
            if current + 1 < count {
                current + 1
            } else if self.loop_focus {
                0
            } else {
                current
            }
        } else {
            // Previous
            if current > 0 {
                current - 1
            } else if self.loop_focus {
                count - 1
            } else {
                current
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_arrow_right_moves_next() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 0), Some(1));
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 3), Some(4));
    }

    #[test]
    fn horizontal_arrow_left_moves_prev() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowLeft, 5, 3), Some(2));
        assert_eq!(nav.handle(&Key::ArrowLeft, 5, 1), Some(0));
    }

    #[test]
    fn vertical_arrow_down_moves_next() {
        let nav = KeyboardNav::new(Orientation::Vertical, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowDown, 5, 0), Some(1));
        assert_eq!(nav.handle(&Key::ArrowDown, 5, 2), Some(3));
    }

    #[test]
    fn vertical_arrow_up_moves_prev() {
        let nav = KeyboardNav::new(Orientation::Vertical, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowUp, 5, 3), Some(2));
        assert_eq!(nav.handle(&Key::ArrowUp, 5, 1), Some(0));
    }

    #[test]
    fn horizontal_ignores_vertical_arrows() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowUp, 5, 2), None);
        assert_eq!(nav.handle(&Key::ArrowDown, 5, 2), None);
    }

    #[test]
    fn vertical_ignores_horizontal_arrows() {
        let nav = KeyboardNav::new(Orientation::Vertical, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowLeft, 5, 2), None);
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 2), None);
    }

    #[test]
    fn home_moves_to_first() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::Home, 5, 3), Some(0));
        assert_eq!(nav.handle(&Key::Home, 5, 0), Some(0));
    }

    #[test]
    fn end_moves_to_last() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::End, 5, 0), Some(4));
        assert_eq!(nav.handle(&Key::End, 5, 4), Some(4));
    }

    #[test]
    fn no_loop_clamps_at_start() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowLeft, 5, 0), Some(0));
    }

    #[test]
    fn no_loop_clamps_at_end() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 4), Some(4));
    }

    #[test]
    fn loop_wraps_from_end_to_start() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow).with_loop(true);
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 4), Some(0));
    }

    #[test]
    fn loop_wraps_from_start_to_end() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow).with_loop(true);
        assert_eq!(nav.handle(&Key::ArrowLeft, 5, 0), Some(4));
    }

    #[test]
    fn loop_wraps_vertical() {
        let nav = KeyboardNav::new(Orientation::Vertical, NavStrategy::Arrow).with_loop(true);
        assert_eq!(nav.handle(&Key::ArrowDown, 3, 2), Some(0));
        assert_eq!(nav.handle(&Key::ArrowUp, 3, 0), Some(2));
    }

    #[test]
    fn tab_strategy_moves_next() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Tab);
        assert_eq!(nav.handle(&Key::Tab, 5, 2), Some(3));
    }

    #[test]
    fn tab_strategy_ignores_arrows() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Tab);
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 2), None);
    }

    #[test]
    fn both_strategy_handles_arrows_and_tab() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Both);
        assert_eq!(nav.handle(&Key::ArrowRight, 5, 2), Some(3));
        assert_eq!(nav.handle(&Key::Tab, 5, 2), Some(3));
    }

    #[test]
    fn irrelevant_keys_return_none() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::Enter, 5, 2), None);
        assert_eq!(nav.handle(&Key::Space, 5, 2), None);
        assert_eq!(nav.handle(&Key::Escape, 5, 2), None);
        assert_eq!(nav.handle(&Key::Char('a'), 5, 2), None);
        assert_eq!(nav.handle(&Key::F(1), 5, 2), None);
    }

    #[test]
    fn empty_items_returns_none() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowRight, 0, 0), None);
        assert_eq!(nav.handle(&Key::Home, 0, 0), None);
    }

    #[test]
    fn single_item_no_loop_stays() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::ArrowRight, 1, 0), Some(0));
        assert_eq!(nav.handle(&Key::ArrowLeft, 1, 0), Some(0));
    }

    #[test]
    fn single_item_with_loop_stays() {
        let nav = KeyboardNav::new(Orientation::Horizontal, NavStrategy::Arrow).with_loop(true);
        assert_eq!(nav.handle(&Key::ArrowRight, 1, 0), Some(0));
        assert_eq!(nav.handle(&Key::ArrowLeft, 1, 0), Some(0));
    }

    #[test]
    fn home_end_with_empty_returns_none() {
        let nav = KeyboardNav::new(Orientation::Vertical, NavStrategy::Arrow);
        assert_eq!(nav.handle(&Key::Home, 0, 0), None);
        assert_eq!(nav.handle(&Key::End, 0, 0), None);
    }
}
