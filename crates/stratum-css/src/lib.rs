//! CSS-in-Rust system for NexusStratum.
//!
//! Provides a runtime CSS registration system with typed style properties
//! and a global registry for collecting and deduplicating CSS class definitions.

pub use stratum_css_macros::css;

mod registry;
mod style_props;

pub use registry::{StyleRegistry, GLOBAL_REGISTRY};
pub use style_props::{
    AlignItems, Cursor, Display, FlexDirection, JustifyContent, Overflow, Position, StyleProps,
    TextAlign,
};

#[cfg(test)]
mod tests {
    use super::*;

    // -- StyleRegistry tests --

    #[test]
    fn registry_register_and_dedup() {
        let reg = StyleRegistry::new();
        assert!(reg.register("btn", ".btn { color: red; }"));
        assert!(reg.register("card", ".card { padding: 8px; }"));
        // Duplicate should return false
        assert!(!reg.register("btn", ".btn { color: red; }"));
        assert_eq!(reg.len(), 2);
    }

    #[test]
    fn registry_to_css_string() {
        let reg = StyleRegistry::new();
        reg.register("a", ".a { color: red; }");
        reg.register("b", ".b { color: blue; }");
        let css = reg.to_css_string();
        assert!(css.contains(".a { color: red; }"));
        assert!(css.contains(".b { color: blue; }"));
    }

    #[test]
    fn registry_clear() {
        let reg = StyleRegistry::new();
        reg.register("x", ".x { display: none; }");
        assert_eq!(reg.len(), 1);
        reg.clear();
        assert!(reg.is_empty());
    }

    // -- StyleProps tests --

    #[test]
    fn style_props_to_inline_css_empty() {
        let props = StyleProps::new();
        assert!(props.is_empty());
        assert_eq!(props.to_inline_css(), "");
    }

    #[test]
    fn style_props_to_inline_css() {
        let mut props = StyleProps::new();
        props.color = Some("red".into());
        props.padding = Some("8px".into());
        props.display = Some(Display::Flex);
        props.z_index = Some(10);
        props.opacity = Some(0.5);

        let css = props.to_inline_css();
        assert!(css.contains("color: red"));
        assert!(css.contains("padding: 8px"));
        assert!(css.contains("display: flex"));
        assert!(css.contains("z-index: 10"));
        assert!(css.contains("opacity: 0.5"));
        assert!(!props.is_empty());
    }

    #[test]
    fn style_props_padding_xy() {
        let mut props = StyleProps::new();
        props.padding_x = Some("16px".into());
        props.padding_y = Some("8px".into());
        let css = props.to_inline_css();
        assert!(css.contains("padding-left: 16px"));
        assert!(css.contains("padding-right: 16px"));
        assert!(css.contains("padding-top: 8px"));
        assert!(css.contains("padding-bottom: 8px"));
    }

    #[test]
    fn style_props_margin_xy() {
        let mut props = StyleProps::new();
        props.margin_x = Some("auto".into());
        props.margin_y = Some("0".into());
        let css = props.to_inline_css();
        assert!(css.contains("margin-left: auto"));
        assert!(css.contains("margin-right: auto"));
        assert!(css.contains("margin-top: 0"));
        assert!(css.contains("margin-bottom: 0"));
    }

    // -- Enum as_css tests --

    #[test]
    fn display_as_css() {
        assert_eq!(Display::Block.as_css(), "block");
        assert_eq!(Display::Inline.as_css(), "inline");
        assert_eq!(Display::InlineBlock.as_css(), "inline-block");
        assert_eq!(Display::Flex.as_css(), "flex");
        assert_eq!(Display::InlineFlex.as_css(), "inline-flex");
        assert_eq!(Display::Grid.as_css(), "grid");
        assert_eq!(Display::InlineGrid.as_css(), "inline-grid");
        assert_eq!(Display::None.as_css(), "none");
        assert_eq!(Display::Contents.as_css(), "contents");
    }

    #[test]
    fn flex_direction_as_css() {
        assert_eq!(FlexDirection::Row.as_css(), "row");
        assert_eq!(FlexDirection::RowReverse.as_css(), "row-reverse");
        assert_eq!(FlexDirection::Column.as_css(), "column");
        assert_eq!(FlexDirection::ColumnReverse.as_css(), "column-reverse");
    }

    #[test]
    fn align_items_as_css() {
        assert_eq!(AlignItems::Start.as_css(), "start");
        assert_eq!(AlignItems::End.as_css(), "end");
        assert_eq!(AlignItems::Center.as_css(), "center");
        assert_eq!(AlignItems::Baseline.as_css(), "baseline");
        assert_eq!(AlignItems::Stretch.as_css(), "stretch");
    }

    #[test]
    fn justify_content_as_css() {
        assert_eq!(JustifyContent::Start.as_css(), "start");
        assert_eq!(JustifyContent::End.as_css(), "end");
        assert_eq!(JustifyContent::Center.as_css(), "center");
        assert_eq!(JustifyContent::SpaceBetween.as_css(), "space-between");
        assert_eq!(JustifyContent::SpaceAround.as_css(), "space-around");
        assert_eq!(JustifyContent::SpaceEvenly.as_css(), "space-evenly");
    }

    #[test]
    fn position_as_css() {
        assert_eq!(Position::Static.as_css(), "static");
        assert_eq!(Position::Relative.as_css(), "relative");
        assert_eq!(Position::Absolute.as_css(), "absolute");
        assert_eq!(Position::Fixed.as_css(), "fixed");
        assert_eq!(Position::Sticky.as_css(), "sticky");
    }

    #[test]
    fn overflow_as_css() {
        assert_eq!(Overflow::Visible.as_css(), "visible");
        assert_eq!(Overflow::Hidden.as_css(), "hidden");
        assert_eq!(Overflow::Scroll.as_css(), "scroll");
        assert_eq!(Overflow::Auto.as_css(), "auto");
    }

    #[test]
    fn cursor_as_css() {
        assert_eq!(Cursor::Default.as_css(), "default");
        assert_eq!(Cursor::Pointer.as_css(), "pointer");
        assert_eq!(Cursor::Text.as_css(), "text");
        assert_eq!(Cursor::Wait.as_css(), "wait");
        assert_eq!(Cursor::NotAllowed.as_css(), "not-allowed");
        assert_eq!(Cursor::Grab.as_css(), "grab");
        assert_eq!(Cursor::Grabbing.as_css(), "grabbing");
    }

    #[test]
    fn text_align_as_css() {
        assert_eq!(TextAlign::Left.as_css(), "left");
        assert_eq!(TextAlign::Center.as_css(), "center");
        assert_eq!(TextAlign::Right.as_css(), "right");
        assert_eq!(TextAlign::Justify.as_css(), "justify");
    }
}
