use serde::{Deserialize, Serialize};

/// Typed style properties that can be applied to components.
/// These map to CSS properties but are validated at compile time.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StyleProps {
    pub color: Option<String>,
    pub background: Option<String>,
    pub padding: Option<String>,
    pub padding_x: Option<String>,
    pub padding_y: Option<String>,
    pub margin: Option<String>,
    pub margin_x: Option<String>,
    pub margin_y: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub min_width: Option<String>,
    pub min_height: Option<String>,
    pub max_width: Option<String>,
    pub max_height: Option<String>,
    pub font_size: Option<String>,
    pub font_weight: Option<String>,
    pub font_family: Option<String>,
    pub line_height: Option<String>,
    pub letter_spacing: Option<String>,
    pub text_align: Option<TextAlign>,
    pub border_radius: Option<String>,
    pub border: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<String>,
    pub display: Option<Display>,
    pub flex_direction: Option<FlexDirection>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub gap: Option<String>,
    pub position: Option<Position>,
    pub top: Option<String>,
    pub right: Option<String>,
    pub bottom: Option<String>,
    pub left: Option<String>,
    pub z_index: Option<i32>,
    pub opacity: Option<f64>,
    pub overflow: Option<Overflow>,
    pub cursor: Option<Cursor>,
    pub transition: Option<String>,
    pub box_shadow: Option<String>,
    pub transform: Option<String>,
}

impl StyleProps {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert to inline CSS string.
    pub fn to_inline_css(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        macro_rules! push_str_prop {
            ($field:ident, $css_name:expr) => {
                if let Some(ref val) = self.$field {
                    parts.push(format!("{}: {}", $css_name, val));
                }
            };
        }

        push_str_prop!(color, "color");
        push_str_prop!(background, "background");
        push_str_prop!(padding, "padding");

        // padding_x expands to padding-left + padding-right
        if let Some(ref val) = self.padding_x {
            parts.push(format!("padding-left: {}", val));
            parts.push(format!("padding-right: {}", val));
        }
        // padding_y expands to padding-top + padding-bottom
        if let Some(ref val) = self.padding_y {
            parts.push(format!("padding-top: {}", val));
            parts.push(format!("padding-bottom: {}", val));
        }

        push_str_prop!(margin, "margin");

        // margin_x expands to margin-left + margin-right
        if let Some(ref val) = self.margin_x {
            parts.push(format!("margin-left: {}", val));
            parts.push(format!("margin-right: {}", val));
        }
        // margin_y expands to margin-top + margin-bottom
        if let Some(ref val) = self.margin_y {
            parts.push(format!("margin-top: {}", val));
            parts.push(format!("margin-bottom: {}", val));
        }

        push_str_prop!(width, "width");
        push_str_prop!(height, "height");
        push_str_prop!(min_width, "min-width");
        push_str_prop!(min_height, "min-height");
        push_str_prop!(max_width, "max-width");
        push_str_prop!(max_height, "max-height");
        push_str_prop!(font_size, "font-size");
        push_str_prop!(font_weight, "font-weight");
        push_str_prop!(font_family, "font-family");
        push_str_prop!(line_height, "line-height");
        push_str_prop!(letter_spacing, "letter-spacing");

        if let Some(ref val) = self.text_align {
            parts.push(format!("text-align: {}", val.as_css()));
        }

        push_str_prop!(border_radius, "border-radius");
        push_str_prop!(border, "border");
        push_str_prop!(border_color, "border-color");
        push_str_prop!(border_width, "border-width");

        if let Some(ref val) = self.display {
            parts.push(format!("display: {}", val.as_css()));
        }
        if let Some(ref val) = self.flex_direction {
            parts.push(format!("flex-direction: {}", val.as_css()));
        }
        if let Some(ref val) = self.align_items {
            parts.push(format!("align-items: {}", val.as_css()));
        }
        if let Some(ref val) = self.justify_content {
            parts.push(format!("justify-content: {}", val.as_css()));
        }

        push_str_prop!(gap, "gap");

        if let Some(ref val) = self.position {
            parts.push(format!("position: {}", val.as_css()));
        }

        push_str_prop!(top, "top");
        push_str_prop!(right, "right");
        push_str_prop!(bottom, "bottom");
        push_str_prop!(left, "left");

        if let Some(val) = self.z_index {
            parts.push(format!("z-index: {}", val));
        }
        if let Some(val) = self.opacity {
            parts.push(format!("opacity: {}", val));
        }

        if let Some(ref val) = self.overflow {
            parts.push(format!("overflow: {}", val.as_css()));
        }
        if let Some(ref val) = self.cursor {
            parts.push(format!("cursor: {}", val.as_css()));
        }

        push_str_prop!(transition, "transition");
        push_str_prop!(box_shadow, "box-shadow");
        push_str_prop!(transform, "transform");

        parts.join("; ")
    }

    /// Whether any properties are set.
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

// ---------------------------------------------------------------------------
// CSS enum types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Grid,
    InlineGrid,
    None,
    Contents,
}

impl Display {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Inline => "inline",
            Self::InlineBlock => "inline-block",
            Self::Flex => "flex",
            Self::InlineFlex => "inline-flex",
            Self::Grid => "grid",
            Self::InlineGrid => "inline-grid",
            Self::None => "none",
            Self::Contents => "contents",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::RowReverse => "row-reverse",
            Self::Column => "column",
            Self::ColumnReverse => "column-reverse",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::SpaceBetween => "space-between",
            Self::SpaceAround => "space-around",
            Self::SpaceEvenly => "space-evenly",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Position {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Relative => "relative",
            Self::Absolute => "absolute",
            Self::Fixed => "fixed",
            Self::Sticky => "sticky",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl Overflow {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Hidden => "hidden",
            Self::Scroll => "scroll",
            Self::Auto => "auto",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cursor {
    Default,
    Pointer,
    Text,
    Wait,
    NotAllowed,
    Grab,
    Grabbing,
}

impl Cursor {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Pointer => "pointer",
            Self::Text => "text",
            Self::Wait => "wait",
            Self::NotAllowed => "not-allowed",
            Self::Grab => "grab",
            Self::Grabbing => "grabbing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl TextAlign {
    pub fn as_css(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
        }
    }
}
