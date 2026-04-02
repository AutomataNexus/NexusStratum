//! Tailwind class merging with conflict resolution.
//!
//! Similar to the `tailwind-merge` JS library, this module resolves
//! conflicting Tailwind CSS classes so that later classes override
//! earlier ones when they target the same CSS property.

use std::collections::HashSet;

/// Known text sizes used by Tailwind.
const TEXT_SIZES: &[&str] = &[
    "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl",
];

/// Known font weights used by Tailwind.
const FONT_WEIGHTS: &[&str] = &[
    "thin",
    "extralight",
    "light",
    "normal",
    "medium",
    "semibold",
    "bold",
    "extrabold",
    "black",
];

/// Display-related classes that all belong to the same "display" group.
const DISPLAY_CLASSES: &[&str] = &[
    "block",
    "inline-block",
    "inline",
    "flex",
    "inline-flex",
    "table",
    "inline-table",
    "table-caption",
    "table-cell",
    "table-column",
    "table-column-group",
    "table-footer-group",
    "table-header-group",
    "table-row-group",
    "table-row",
    "flow-root",
    "grid",
    "inline-grid",
    "contents",
    "list-item",
    "hidden",
];

/// Position-related classes that all belong to the same "position" group.
const POSITION_CLASSES: &[&str] = &["static", "relative", "absolute", "fixed", "sticky"];

/// Shadow size keywords (not colors).
const SHADOW_SIZES: &[&str] = &[
    "sm", "md", "lg", "xl", "2xl", "inner", "none",
];

/// Ring size keywords (not colors).
const RING_SIZES: &[&str] = &["0", "1", "2", "4", "8"];

/// Merge multiple class strings, resolving conflicts.
/// Later classes override earlier ones when they target the same CSS property.
///
/// Variant prefixes (e.g. `sm:`, `hover:`, `md:hover:`) are preserved and
/// only classes sharing the **same** variant prefix can conflict.
///
/// # Examples
///
/// ```
/// use stratum_tailwind::merge_classes;
///
/// let result = merge_classes(&["h-8 px-3", "h-12"]);
/// assert_eq!(result, "px-3 h-12");
/// ```
pub fn merge_classes(class_lists: &[&str]) -> String {
    // Collect all individual classes preserving order.
    let all_classes: Vec<&str> = class_lists
        .iter()
        .flat_map(|list| list.split_whitespace())
        .collect();

    if all_classes.is_empty() {
        return String::new();
    }

    // Walk right-to-left, tracking which groups we have already seen.
    // If a group was already claimed by a later class, the earlier class is dropped.
    let mut seen_groups: HashSet<String> = HashSet::new();
    let mut keep: Vec<bool> = vec![false; all_classes.len()];

    for i in (0..all_classes.len()).rev() {
        let group = class_group_key(all_classes[i]);
        if seen_groups.contains(&group) {
            keep[i] = false;
        } else {
            seen_groups.insert(group);
            keep[i] = true;
        }
    }

    // Collect surviving classes in their original order.
    let result: Vec<&str> = all_classes
        .iter()
        .enumerate()
        .filter(|(i, _)| keep[*i])
        .map(|(_, cls)| *cls)
        .collect();

    result.join(" ")
}

/// Build the full group key for a class, including its variant prefix.
///
/// For example:
/// - `"h-8"` -> `"h"`
/// - `"sm:h-8"` -> `"sm:h"`
/// - `"md:hover:text-lg"` -> `"md:hover:text-size"`
fn class_group_key(class: &str) -> String {
    let (variant, base) = split_variant(class);
    let group = base_class_group(base);
    if variant.is_empty() {
        group
    } else {
        format!("{}:{}", variant, group)
    }
}

/// Split a class into its variant prefix and the base utility.
///
/// `"sm:hover:text-lg"` -> `("sm:hover", "text-lg")`
/// `"text-lg"` -> `("", "text-lg")`
fn split_variant(class: &str) -> (&str, &str) {
    if let Some(pos) = class.rfind(':') {
        (&class[..pos], &class[pos + 1..])
    } else {
        ("", class)
    }
}

/// Check if two Tailwind classes conflict (target the same CSS property).
#[allow(dead_code)]
fn classes_conflict(a: &str, b: &str) -> bool {
    class_group_key(a) == class_group_key(b)
}

/// Determine whether a value looks like a Tailwind color (e.g. "red-500", "black", "[#fff]").
fn looks_like_color_value(value: &str) -> bool {
    if value.starts_with('[') {
        return true;
    }
    let color_keywords = [
        "inherit",
        "current",
        "transparent",
        "black",
        "white",
    ];
    if color_keywords.contains(&value) {
        return true;
    }
    // Pattern: word-number like "red-500", "gray-100", "emerald-50"
    if let Some(dash) = value.rfind('-') {
        let suffix = &value[dash + 1..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    false
}

/// Extract the "property group" from a base Tailwind utility (no variant prefix).
///
/// Classes that share a group conflict with each other.
///
/// This function handles negative values by stripping a leading `-` before
/// matching (e.g., `-mx-4` base is `-mx-4`, strip to `mx-4`, group is `mx`).
fn base_class_group(base: &str) -> String {
    // Handle negative values: strip leading dash before matching.
    let effective = base.strip_prefix('-').unwrap_or(base);

    // Check standalone display classes.
    if DISPLAY_CLASSES.contains(&effective) {
        return "display".to_string();
    }

    // Check standalone position classes.
    if POSITION_CLASSES.contains(&effective) {
        return "position".to_string();
    }

    // sr-only / not-sr-only
    if effective == "sr-only" || effective == "not-sr-only" {
        return "sr-only".to_string();
    }

    // Flex direction classes.
    if effective == "flex-row"
        || effective == "flex-col"
        || effective == "flex-row-reverse"
        || effective == "flex-col-reverse"
    {
        return "flex-direction".to_string();
    }

    // Flex shorthand classes: flex-1, flex-auto, flex-initial, flex-none.
    if effective == "flex-1"
        || effective == "flex-auto"
        || effective == "flex-initial"
        || effective == "flex-none"
    {
        return "flex".to_string();
    }

    // Flex-grow: flex-grow, flex-grow-0.
    if effective == "flex-grow" || effective.starts_with("flex-grow-") {
        return "flex-grow".to_string();
    }

    // Flex-shrink: flex-shrink, flex-shrink-0.
    if effective == "flex-shrink" || effective.starts_with("flex-shrink-") {
        return "flex-shrink".to_string();
    }

    // Flex-wrap: flex-wrap, flex-wrap-reverse, flex-nowrap.
    if effective == "flex-wrap" || effective == "flex-wrap-reverse" || effective == "flex-nowrap" {
        return "flex-wrap".to_string();
    }

    // Split on the first '-' to get the prefix.
    if let Some(dash_pos) = effective.find('-') {
        let prefix = &effective[..dash_pos];
        let value = &effective[dash_pos + 1..];

        let group: String = match prefix {
            // Height and width.
            "h" => "h".into(),
            "w" => "w".into(),
            "min" => {
                if value.starts_with("h-") || value == "h" {
                    "min-h".into()
                } else if value.starts_with("w-") || value == "w" {
                    "min-w".into()
                } else {
                    effective.into()
                }
            }
            "max" => {
                if value.starts_with("h-") || value == "h" {
                    "max-h".into()
                } else if value.starts_with("w-") || value == "w" {
                    "max-w".into()
                } else {
                    effective.into()
                }
            }
            // Padding.
            "p" | "px" | "py" | "pt" | "pr" | "pb" | "pl" => prefix.into(),
            // Margin.
            "m" | "mx" | "my" | "mt" | "mr" | "mb" | "ml" => prefix.into(),
            // Space between.
            "space" => {
                if value.starts_with("x-") || value == "x" {
                    "space-x".into()
                } else if value.starts_with("y-") || value == "y" {
                    "space-y".into()
                } else {
                    effective.into()
                }
            }
            // Background.
            "bg" => "bg".into(),
            // Text: disambiguate between text-size and text-color.
            "text" => {
                if is_text_size(value) {
                    "text-size".into()
                } else {
                    "text-color".into()
                }
            }
            // Font weight vs family.
            "font" => {
                if FONT_WEIGHTS.contains(&value) {
                    "font".into()
                } else {
                    "font-family".into()
                }
            }
            // Border radius.
            "rounded" => {
                match value {
                    "t" | "r" | "b" | "l" | "tl" | "tr" | "br" | "bl" => effective.into(),
                    v if v.starts_with("t-") || v.starts_with("r-") || v.starts_with("b-")
                        || v.starts_with("l-") || v.starts_with("tl-") || v.starts_with("tr-")
                        || v.starts_with("br-") || v.starts_with("bl-") =>
                    {
                        if let Some(d) = v.find('-') {
                            format!("rounded-{}", &v[..d])
                        } else {
                            effective.into()
                        }
                    }
                    _ => "rounded".into(),
                }
            }
            // Border utilities.
            "border" => {
                match value {
                    "t" | "r" | "b" | "l" => effective.into(),
                    v if v.starts_with("t-") || v.starts_with("r-")
                        || v.starts_with("b-") || v.starts_with("l-") =>
                    {
                        let dir = &v[..1];
                        let rest = &v[2..];
                        if looks_like_color_value(rest) {
                            format!("border-{}-color", dir)
                        } else {
                            format!("border-{}", dir)
                        }
                    }
                    v if !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()) => "border-width".into(),
                    "solid" | "dashed" | "dotted" | "double" | "hidden" | "none" => "border-style".into(),
                    "collapse" | "separate" => "border-collapse".into(),
                    _ => "border-color".into(),
                }
            }
            // Shadow.
            "shadow" => {
                if SHADOW_SIZES.contains(&value) {
                    "shadow".into()
                } else {
                    "shadow-color".into()
                }
            }
            // Ring.
            "ring" => {
                if value == "inset" {
                    "ring-inset".into()
                } else if value.starts_with("offset") {
                    "ring-offset".into()
                } else if RING_SIZES.contains(&value) {
                    "ring".into()
                } else {
                    "ring-color".into()
                }
            }
            // Outline.
            "outline" => {
                match value {
                    "none" | "dashed" | "dotted" | "double" => "outline-style".into(),
                    "offset" => "outline-offset".into(),
                    v if v.starts_with("offset-") => "outline-offset".into(),
                    v if !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()) => "outline-width".into(),
                    _ => "outline-color".into(),
                }
            }
            // Alignment.
            "items" => "items".into(),
            "justify" => "justify".into(),
            "self" => "self".into(),
            "place" => {
                if value.starts_with("content-") || value == "content" {
                    "place-content".into()
                } else if value.starts_with("items-") || value == "items" {
                    "place-items".into()
                } else if value.starts_with("self-") || value == "self" {
                    "place-self".into()
                } else {
                    effective.into()
                }
            }
            "content" => "content".into(),
            // Gap.
            "gap" => {
                if value.starts_with("x-") {
                    "gap-x".into()
                } else if value.starts_with("y-") {
                    "gap-y".into()
                } else {
                    "gap".into()
                }
            }
            // Overflow.
            "overflow" => {
                match value {
                    v if v.starts_with("x-") => "overflow-x".into(),
                    v if v.starts_with("y-") => "overflow-y".into(),
                    _ => "overflow".into(),
                }
            }
            // Z-index.
            "z" => "z".into(),
            // Opacity.
            "opacity" => "opacity".into(),
            // Inset.
            "inset" => {
                if value.starts_with("x-") {
                    "inset-x".into()
                } else if value.starts_with("y-") {
                    "inset-y".into()
                } else {
                    "inset".into()
                }
            }
            "top" => "top".into(),
            "right" => "right".into(),
            "bottom" => "bottom".into(),
            "left" => "left".into(),
            // Whitespace.
            "whitespace" => "whitespace".into(),
            // Overflow-wrap.
            "break" => {
                match value {
                    "normal" | "words" | "all" | "keep" => "overflow-wrap".into(),
                    _ => effective.into(),
                }
            }
            // List style.
            "list" => {
                match value {
                    "inside" | "outside" => "list-style-position".into(),
                    _ => "list-style-type".into(),
                }
            }
            // Transition.
            "transition" => "transition".into(),
            "duration" => "duration".into(),
            "ease" => "ease".into(),
            "delay" => "delay".into(),
            // Transform.
            "scale" => {
                if value.starts_with("x-") {
                    "scale-x".into()
                } else if value.starts_with("y-") {
                    "scale-y".into()
                } else {
                    "scale".into()
                }
            }
            "rotate" => "rotate".into(),
            "translate" => {
                if value.starts_with("x-") {
                    "translate-x".into()
                } else if value.starts_with("y-") {
                    "translate-y".into()
                } else {
                    "translate".into()
                }
            }
            "skew" => {
                if value.starts_with("x-") {
                    "skew-x".into()
                } else if value.starts_with("y-") {
                    "skew-y".into()
                } else {
                    "skew".into()
                }
            }
            "origin" => "origin".into(),
            // Cursor.
            "cursor" => "cursor".into(),
            // User select.
            "select" => "select".into(),
            // Resize.
            "resize" => "resize".into(),
            // Scroll snap.
            "snap" => {
                match value {
                    "start" | "end" | "center" | "align-none" => "snap-align".into(),
                    "normal" | "always" => "snap-stop".into(),
                    "none" | "x" | "y" | "both" | "mandatory" | "proximity" => "snap-type".into(),
                    _ => effective.into(),
                }
            }
            // Scroll behavior / margin / padding.
            "scroll" => {
                if value == "auto" || value == "smooth" {
                    "scroll-behavior".into()
                } else if value.starts_with("m") {
                    "scroll-margin".into()
                } else if value.starts_with("p") {
                    "scroll-padding".into()
                } else {
                    effective.into()
                }
            }
            // Touch action.
            "touch" => "touch".into(),
            // Accent color.
            "accent" => "accent".into(),
            // Caret color.
            "caret" => "caret".into(),
            // Fill and stroke (SVG).
            "fill" => "fill".into(),
            "stroke" => {
                if !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()) {
                    "stroke-width".into()
                } else {
                    "stroke-color".into()
                }
            }
            // Grid.
            "grid" => {
                if value.starts_with("cols-") {
                    "grid-cols".into()
                } else if value.starts_with("rows-") {
                    "grid-rows".into()
                } else if value.starts_with("flow-") {
                    "grid-flow".into()
                } else {
                    effective.into()
                }
            }
            "col" => {
                if value.starts_with("span-") || value == "auto" {
                    "col-span".into()
                } else if value.starts_with("start-") {
                    "col-start".into()
                } else if value.starts_with("end-") {
                    "col-end".into()
                } else {
                    effective.into()
                }
            }
            "row" => {
                if value.starts_with("span-") || value == "auto" {
                    "row-span".into()
                } else if value.starts_with("start-") {
                    "row-start".into()
                } else if value.starts_with("end-") {
                    "row-end".into()
                } else {
                    effective.into()
                }
            }
            "auto" => {
                if value.starts_with("cols-") {
                    "auto-cols".into()
                } else if value.starts_with("rows-") {
                    "auto-rows".into()
                } else {
                    effective.into()
                }
            }
            // Order.
            "order" => "order".into(),
            // Aspect ratio.
            "aspect" => "aspect".into(),
            // Object fit / position.
            "object" => {
                match value {
                    "contain" | "cover" | "fill" | "none" | "scale-down" => "object-fit".into(),
                    _ => "object-position".into(),
                }
            }
            // Leading (line-height).
            "leading" => "leading".into(),
            // Tracking (letter-spacing).
            "tracking" => "tracking".into(),
            // Decoration.
            "decoration" => {
                match value {
                    "solid" | "double" | "dotted" | "dashed" | "wavy" => "decoration-style".into(),
                    "auto" | "from-font" => "decoration-thickness".into(),
                    v if !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()) => "decoration-thickness".into(),
                    _ => "decoration-color".into(),
                }
            }
            // Text decoration line.
            "underline" => "text-decoration".into(),
            "overline" => "text-decoration".into(),
            "line" => {
                if value == "through" {
                    "text-decoration".into()
                } else if value == "clamp" || value.starts_with("clamp-") {
                    "line-clamp".into()
                } else {
                    effective.into()
                }
            }
            "no" => {
                if value == "underline" {
                    "text-decoration".into()
                } else {
                    effective.into()
                }
            }
            // Columns.
            "columns" => "columns".into(),
            // Will-change.
            "will" => {
                if value.starts_with("change") {
                    "will-change".into()
                } else {
                    effective.into()
                }
            }
            // Pointer events.
            "pointer" => {
                if value.starts_with("events") {
                    "pointer-events".into()
                } else {
                    effective.into()
                }
            }
            // Default: use the full effective class as its own group.
            _ => effective.into(),
        };
        group
    } else {
        // No dash in effective -- standalone classes.
        match effective {
            "shadow" => "shadow".into(),
            "ring" => "ring".into(),
            "border" => "border-width".into(),
            "outline" => "outline-style".into(),
            "transition" => "transition".into(),
            "truncate" => "text-overflow".into(),
            "antialiased" | "subpixel-antialiased" => "font-smoothing".into(),
            "underline" => "text-decoration".into(),
            "overline" => "text-decoration".into(),
            "visible" => "visibility".into(),
            "invisible" => "visibility".into(),
            "collapse" => "visibility".into(),
            "resize" => "resize".into(),
            "transform" => "transform".into(),
            "uppercase" | "lowercase" | "capitalize" => "text-transform".into(),
            "italic" => "font-style".into(),
            _ => effective.into(),
        }
    }
}

/// Determine whether a text-* value represents a size rather than a color.
fn is_text_size(value: &str) -> bool {
    TEXT_SIZES.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------
    // Basic merging
    // -------------------------------------------------------

    #[test]
    fn merge_resolves_height_conflicts() {
        let result = merge_classes(&["h-8 px-3", "h-12"]);
        assert_eq!(result, "px-3 h-12");
    }

    #[test]
    fn merge_resolves_padding_conflicts() {
        let result = merge_classes(&["p-4 px-2", "p-6"]);
        assert_eq!(result, "px-2 p-6");
    }

    #[test]
    fn merge_preserves_non_conflicting_classes() {
        let result = merge_classes(&["h-8 w-4 text-sm", "bg-primary"]);
        assert_eq!(result, "h-8 w-4 text-sm bg-primary");
    }

    #[test]
    fn merge_handles_empty_input() {
        let result = merge_classes(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn merge_handles_empty_strings() {
        let result = merge_classes(&["", ""]);
        assert_eq!(result, "");
    }

    #[test]
    fn merge_resolves_display_conflicts() {
        let result = merge_classes(&["flex items-center", "block"]);
        assert_eq!(result, "items-center block");
    }

    #[test]
    fn merge_resolves_position_conflicts() {
        let result = merge_classes(&["relative z-10", "absolute"]);
        assert_eq!(result, "z-10 absolute");
    }

    #[test]
    fn merge_resolves_text_size_vs_color() {
        let result = merge_classes(&["text-sm text-red-500"]);
        assert_eq!(result, "text-sm text-red-500");
    }

    #[test]
    fn merge_resolves_text_size_conflicts() {
        let result = merge_classes(&["text-sm", "text-lg"]);
        assert_eq!(result, "text-lg");
    }

    #[test]
    fn merge_resolves_text_color_conflicts() {
        let result = merge_classes(&["text-red-500", "text-blue-300"]);
        assert_eq!(result, "text-blue-300");
    }

    #[test]
    fn merge_resolves_bg_conflicts() {
        let result = merge_classes(&["bg-red-500 p-4", "bg-blue-300"]);
        assert_eq!(result, "p-4 bg-blue-300");
    }

    #[test]
    fn merge_resolves_rounded_conflicts() {
        let result = merge_classes(&["rounded-sm", "rounded-lg"]);
        assert_eq!(result, "rounded-lg");
    }

    #[test]
    fn merge_resolves_font_weight_conflicts() {
        let result = merge_classes(&["font-bold", "font-semibold"]);
        assert_eq!(result, "font-semibold");
    }

    #[test]
    fn merge_resolves_width_conflicts() {
        let result = merge_classes(&["w-full", "w-1/2"]);
        assert_eq!(result, "w-1/2");
    }

    #[test]
    fn merge_resolves_z_index_conflicts() {
        let result = merge_classes(&["z-10", "z-50"]);
        assert_eq!(result, "z-50");
    }

    #[test]
    fn merge_resolves_overflow_conflicts() {
        let result = merge_classes(&["overflow-hidden", "overflow-auto"]);
        assert_eq!(result, "overflow-auto");
    }

    #[test]
    fn merge_resolves_opacity_conflicts() {
        let result = merge_classes(&["opacity-50", "opacity-100"]);
        assert_eq!(result, "opacity-100");
    }

    #[test]
    fn merge_multiple_lists() {
        let result = merge_classes(&["h-8 px-3", "bg-primary text-sm", "h-12 text-lg"]);
        assert_eq!(result, "px-3 bg-primary h-12 text-lg");
    }

    // -------------------------------------------------------
    // Issue 1: Variant-prefixed classes don't conflict with unprefixed
    // -------------------------------------------------------

    #[test]
    fn variant_prefix_no_conflict_with_unprefixed() {
        let result = merge_classes(&["sm:h-8", "h-8"]);
        assert_eq!(result, "sm:h-8 h-8");
    }

    #[test]
    fn same_variant_prefix_does_conflict() {
        let result = merge_classes(&["sm:h-8", "sm:h-12"]);
        assert_eq!(result, "sm:h-12");
    }

    #[test]
    fn different_variant_prefixes_no_conflict() {
        let result = merge_classes(&["sm:h-8", "md:h-12"]);
        assert_eq!(result, "sm:h-8 md:h-12");
    }

    #[test]
    fn compound_variant_prefix() {
        let result = merge_classes(&["md:hover:text-lg", "md:hover:text-sm"]);
        assert_eq!(result, "md:hover:text-sm");
    }

    #[test]
    fn compound_variant_vs_single_variant() {
        let result = merge_classes(&["md:hover:text-lg", "hover:text-sm"]);
        assert_eq!(result, "md:hover:text-lg hover:text-sm");
    }

    // -------------------------------------------------------
    // Issue 2: flex-1/flex-auto/flex-initial/flex-none same group
    // -------------------------------------------------------

    #[test]
    fn flex_1_and_flex_auto_conflict() {
        let result = merge_classes(&["flex-1", "flex-auto"]);
        assert_eq!(result, "flex-auto");
    }

    #[test]
    fn flex_initial_and_flex_none_conflict() {
        let result = merge_classes(&["flex-initial", "flex-none"]);
        assert_eq!(result, "flex-none");
    }

    #[test]
    fn flex_1_and_flex_none_conflict() {
        let result = merge_classes(&["flex-1", "flex-none"]);
        assert_eq!(result, "flex-none");
    }

    // -------------------------------------------------------
    // Issue 3: flex-grow / flex-grow-0 conflict; flex-shrink similarly
    // -------------------------------------------------------

    #[test]
    fn flex_grow_and_flex_grow_0_conflict() {
        let result = merge_classes(&["flex-grow", "flex-grow-0"]);
        assert_eq!(result, "flex-grow-0");
    }

    #[test]
    fn flex_shrink_and_flex_shrink_0_conflict() {
        let result = merge_classes(&["flex-shrink", "flex-shrink-0"]);
        assert_eq!(result, "flex-shrink-0");
    }

    // -------------------------------------------------------
    // Issue 5: Negative margins
    // -------------------------------------------------------

    #[test]
    fn negative_margin_conflicts_with_positive() {
        let result = merge_classes(&["mx-4", "-mx-4"]);
        assert_eq!(result, "-mx-4");
    }

    #[test]
    fn negative_margin_conflicts_with_negative() {
        let result = merge_classes(&["-mx-2", "-mx-4"]);
        assert_eq!(result, "-mx-4");
    }

    #[test]
    fn negative_margin_does_not_conflict_with_other_axis() {
        let result = merge_classes(&["-mx-4", "-my-2"]);
        assert_eq!(result, "-mx-4 -my-2");
    }

    // -------------------------------------------------------
    // Issue 4: Border groups
    // -------------------------------------------------------

    #[test]
    fn border_width_conflicts() {
        let result = merge_classes(&["border-2", "border-4"]);
        assert_eq!(result, "border-4");
    }

    #[test]
    fn border_standalone_conflicts_with_border_width() {
        let result = merge_classes(&["border", "border-0"]);
        assert_eq!(result, "border-0");
    }

    #[test]
    fn border_color_conflicts() {
        let result = merge_classes(&["border-red-500", "border-blue-300"]);
        assert_eq!(result, "border-blue-300");
    }

    #[test]
    fn border_width_and_color_no_conflict() {
        let result = merge_classes(&["border-2", "border-red-500"]);
        assert_eq!(result, "border-2 border-red-500");
    }

    #[test]
    fn border_style_conflicts() {
        let result = merge_classes(&["border-solid", "border-dashed"]);
        assert_eq!(result, "border-dashed");
    }

    #[test]
    fn border_directional_no_conflict_with_base() {
        let result = merge_classes(&["border-2", "border-t-2"]);
        assert_eq!(result, "border-2 border-t-2");
    }

    // -------------------------------------------------------
    // Shadow / Ring groups
    // -------------------------------------------------------

    #[test]
    fn shadow_size_conflicts() {
        let result = merge_classes(&["shadow-sm", "shadow-lg"]);
        assert_eq!(result, "shadow-lg");
    }

    #[test]
    fn shadow_standalone_conflicts_with_sized() {
        let result = merge_classes(&["shadow", "shadow-md"]);
        assert_eq!(result, "shadow-md");
    }

    #[test]
    fn ring_size_conflicts() {
        let result = merge_classes(&["ring", "ring-2"]);
        assert_eq!(result, "ring-2");
    }

    #[test]
    fn ring_color_no_conflict_with_size() {
        let result = merge_classes(&["ring-2", "ring-blue-500"]);
        assert_eq!(result, "ring-2 ring-blue-500");
    }

    // -------------------------------------------------------
    // Transition / duration / ease / delay
    // -------------------------------------------------------

    #[test]
    fn duration_conflicts() {
        let result = merge_classes(&["duration-150", "duration-300"]);
        assert_eq!(result, "duration-300");
    }

    #[test]
    fn ease_conflicts() {
        let result = merge_classes(&["ease-in", "ease-out"]);
        assert_eq!(result, "ease-out");
    }

    // -------------------------------------------------------
    // Transform utilities
    // -------------------------------------------------------

    #[test]
    fn scale_conflicts() {
        let result = merge_classes(&["scale-50", "scale-100"]);
        assert_eq!(result, "scale-100");
    }

    #[test]
    fn rotate_conflicts() {
        let result = merge_classes(&["rotate-45", "rotate-90"]);
        assert_eq!(result, "rotate-90");
    }

    // -------------------------------------------------------
    // Cursor / select / resize
    // -------------------------------------------------------

    #[test]
    fn cursor_conflicts() {
        let result = merge_classes(&["cursor-pointer", "cursor-default"]);
        assert_eq!(result, "cursor-default");
    }

    #[test]
    fn select_conflicts() {
        let result = merge_classes(&["select-none", "select-text"]);
        assert_eq!(result, "select-text");
    }

    // -------------------------------------------------------
    // class_group_key identification
    // -------------------------------------------------------

    #[test]
    fn class_group_identification() {
        assert_eq!(class_group_key("h-8"), "h");
        assert_eq!(class_group_key("px-3"), "px");
        assert_eq!(class_group_key("bg-primary"), "bg");
        assert_eq!(class_group_key("text-sm"), "text-size");
        assert_eq!(class_group_key("text-red-500"), "text-color");
        assert_eq!(class_group_key("font-bold"), "font");
        assert_eq!(class_group_key("rounded-lg"), "rounded");
        assert_eq!(class_group_key("items-center"), "items");
        assert_eq!(class_group_key("justify-between"), "justify");
        assert_eq!(class_group_key("gap-4"), "gap");
        assert_eq!(class_group_key("z-10"), "z");
        assert_eq!(class_group_key("opacity-50"), "opacity");
        assert_eq!(class_group_key("overflow-hidden"), "overflow");
        assert_eq!(class_group_key("w-full"), "w");
        assert_eq!(class_group_key("flex"), "display");
        assert_eq!(class_group_key("hidden"), "display");
        assert_eq!(class_group_key("absolute"), "position");
        assert_eq!(class_group_key("sticky"), "position");
        assert_eq!(class_group_key("flex-row"), "flex-direction");
        assert_eq!(class_group_key("flex-col"), "flex-direction");
    }

    #[test]
    fn class_group_with_variants() {
        assert_eq!(class_group_key("sm:h-8"), "sm:h");
        assert_eq!(class_group_key("hover:bg-blue-500"), "hover:bg");
        assert_eq!(class_group_key("md:hover:text-lg"), "md:hover:text-size");
    }

    #[test]
    fn classes_conflict_works() {
        assert!(classes_conflict("h-8", "h-12"));
        assert!(classes_conflict("text-sm", "text-lg"));
        assert!(!classes_conflict("text-sm", "text-red-500"));
        assert!(!classes_conflict("h-8", "w-4"));
        assert!(classes_conflict("flex", "hidden"));
        assert!(classes_conflict("absolute", "relative"));
        assert!(!classes_conflict("sm:h-8", "h-8"));
        assert!(classes_conflict("sm:h-8", "sm:h-12"));
    }

    #[test]
    fn split_variant_works() {
        assert_eq!(split_variant("sm:text-lg"), ("sm", "text-lg"));
        assert_eq!(split_variant("hover:bg-blue-500"), ("hover", "bg-blue-500"));
        assert_eq!(split_variant("sm:hover:text-lg"), ("sm:hover", "text-lg"));
        assert_eq!(split_variant("text-lg"), ("", "text-lg"));
    }

    // -------------------------------------------------------
    // HashSet performance (basic sanity)
    // -------------------------------------------------------

    #[test]
    fn hashset_handles_many_classes() {
        let classes: Vec<String> = (0..1000).map(|i| format!("p-{}", i)).collect();
        let class_str = classes.join(" ");
        let result = merge_classes(&[&class_str]);
        assert_eq!(result, "p-999");
    }

    // -------------------------------------------------------
    // Outline / whitespace / fill / stroke / accent / caret
    // -------------------------------------------------------

    #[test]
    fn outline_style_conflicts() {
        let result = merge_classes(&["outline-dashed", "outline-dotted"]);
        assert_eq!(result, "outline-dotted");
    }

    #[test]
    fn whitespace_conflicts() {
        let result = merge_classes(&["whitespace-nowrap", "whitespace-normal"]);
        assert_eq!(result, "whitespace-normal");
    }

    #[test]
    fn fill_conflicts() {
        let result = merge_classes(&["fill-red-500", "fill-blue-500"]);
        assert_eq!(result, "fill-blue-500");
    }

    #[test]
    fn stroke_color_conflicts() {
        let result = merge_classes(&["stroke-red-500", "stroke-blue-500"]);
        assert_eq!(result, "stroke-blue-500");
    }

    #[test]
    fn accent_conflicts() {
        let result = merge_classes(&["accent-red-500", "accent-blue-500"]);
        assert_eq!(result, "accent-blue-500");
    }

    #[test]
    fn caret_conflicts() {
        let result = merge_classes(&["caret-red-500", "caret-blue-500"]);
        assert_eq!(result, "caret-blue-500");
    }

    #[test]
    fn sr_only_conflicts() {
        let result = merge_classes(&["sr-only", "not-sr-only"]);
        assert_eq!(result, "not-sr-only");
    }
}
