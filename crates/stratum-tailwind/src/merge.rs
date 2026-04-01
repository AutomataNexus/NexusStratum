//! Tailwind class merging with conflict resolution.
//!
//! Similar to the `tailwind-merge` JS library, this module resolves
//! conflicting Tailwind CSS classes so that later classes override
//! earlier ones when they target the same CSS property.

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

/// Merge multiple class strings, resolving conflicts.
/// Later classes override earlier ones when they target the same CSS property.
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
    let mut seen_groups: Vec<&str> = Vec::new();
    let mut keep: Vec<bool> = vec![false; all_classes.len()];

    for i in (0..all_classes.len()).rev() {
        let group = class_group(all_classes[i]);
        if seen_groups.contains(&group) {
            // This class is overridden by a later one.
            keep[i] = false;
        } else {
            seen_groups.push(group);
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

/// Check if two Tailwind classes conflict (target the same CSS property).
#[allow(dead_code)]
fn classes_conflict(a: &str, b: &str) -> bool {
    class_group(a) == class_group(b)
}

/// Extract the "property group" from a Tailwind class.
///
/// Classes that share a group conflict with each other.
/// A class with a unique group (its own name) never conflicts.
///
/// Examples:
/// - `"h-8"` -> `"h"`
/// - `"px-3"` -> `"px"`
/// - `"bg-primary"` -> `"bg"`
/// - `"text-sm"` -> `"text-size"`
/// - `"text-red-500"` -> `"text-color"`
/// - `"font-bold"` -> `"font"`
/// - `"flex"` -> `"display"`
/// - `"absolute"` -> `"position"`
fn class_group(class: &str) -> &str {
    // Strip any responsive / state prefixes (e.g., "sm:", "hover:", "dark:").
    let base = strip_prefixes(class);

    // Check standalone display classes.
    if DISPLAY_CLASSES.contains(&base) {
        return "display";
    }

    // Check standalone position classes.
    if POSITION_CLASSES.contains(&base) {
        return "position";
    }

    // Flex direction classes.
    if base == "flex-row" || base == "flex-col" || base == "flex-row-reverse" || base == "flex-col-reverse" {
        return "flex-direction";
    }

    // Split on the first '-' to get the prefix.
    if let Some(dash_pos) = base.find('-') {
        let prefix = &base[..dash_pos];
        let value = &base[dash_pos + 1..];

        match prefix {
            // Height and width.
            "h" => "h",
            "w" => "w",
            "min" => {
                // min-h-*, min-w-*
                if value.starts_with("h-") {
                    "min-h"
                } else if value.starts_with("w-") {
                    "min-w"
                } else {
                    base
                }
            }
            "max" => {
                // max-h-*, max-w-*
                if value.starts_with("h-") {
                    "max-h"
                } else if value.starts_with("w-") {
                    "max-w"
                } else {
                    base
                }
            }
            // Padding.
            "p" | "px" | "py" | "pt" | "pr" | "pb" | "pl" => prefix,
            // Margin (including negative margins like -m-*).
            "m" | "mx" | "my" | "mt" | "mr" | "mb" | "ml" => prefix,
            // Background.
            "bg" => "bg",
            // Text: disambiguate between text-size and text-color.
            "text" => {
                if is_text_size(value) {
                    "text-size"
                } else {
                    "text-color"
                }
            }
            // Font weight.
            "font" => {
                if FONT_WEIGHTS.contains(&value) {
                    "font"
                } else {
                    // font-sans, font-serif, font-mono -> font-family
                    "font-family"
                }
            }
            // Border radius.
            "rounded" => "rounded",
            // Flex utilities that aren't direction.
            "flex" => {
                // flex-1, flex-auto, flex-initial, flex-none, flex-grow, flex-shrink, etc.
                base
            }
            // Alignment.
            "items" => "items",
            "justify" => "justify",
            // Gap.
            "gap" => "gap",
            // Overflow.
            "overflow" => "overflow",
            // Z-index.
            "z" => "z",
            // Opacity.
            "opacity" => "opacity",
            // Default: use the full class as its own group (no conflicts).
            _ => base,
        }
    } else {
        // No dash -- standalone class, already handled display/position above.
        // Return the class itself as its own unique group.
        base
    }
}

/// Strip responsive and state variant prefixes (e.g., `"sm:hover:text-lg"` -> `"text-lg"`).
fn strip_prefixes(class: &str) -> &str {
    // Find the last ':' -- everything after it is the base utility.
    if let Some(pos) = class.rfind(':') {
        &class[pos + 1..]
    } else {
        class
    }
}

/// Determine whether a text-* value represents a size rather than a color.
fn is_text_size(value: &str) -> bool {
    TEXT_SIZES.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // text-sm (size) and text-red-500 (color) should NOT conflict.
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

    #[test]
    fn class_group_identification() {
        assert_eq!(class_group("h-8"), "h");
        assert_eq!(class_group("px-3"), "px");
        assert_eq!(class_group("bg-primary"), "bg");
        assert_eq!(class_group("text-sm"), "text-size");
        assert_eq!(class_group("text-red-500"), "text-color");
        assert_eq!(class_group("font-bold"), "font");
        assert_eq!(class_group("rounded-lg"), "rounded");
        assert_eq!(class_group("items-center"), "items");
        assert_eq!(class_group("justify-between"), "justify");
        assert_eq!(class_group("gap-4"), "gap");
        assert_eq!(class_group("z-10"), "z");
        assert_eq!(class_group("opacity-50"), "opacity");
        assert_eq!(class_group("overflow-hidden"), "overflow");
        assert_eq!(class_group("w-full"), "w");
        assert_eq!(class_group("flex"), "display");
        assert_eq!(class_group("hidden"), "display");
        assert_eq!(class_group("absolute"), "position");
        assert_eq!(class_group("sticky"), "position");
        assert_eq!(class_group("flex-row"), "flex-direction");
        assert_eq!(class_group("flex-col"), "flex-direction");
    }

    #[test]
    fn classes_conflict_works() {
        assert!(classes_conflict("h-8", "h-12"));
        assert!(classes_conflict("text-sm", "text-lg"));
        assert!(!classes_conflict("text-sm", "text-red-500"));
        assert!(!classes_conflict("h-8", "w-4"));
        assert!(classes_conflict("flex", "hidden"));
        assert!(classes_conflict("absolute", "relative"));
    }

    #[test]
    fn strip_prefixes_works() {
        assert_eq!(strip_prefixes("sm:text-lg"), "text-lg");
        assert_eq!(strip_prefixes("hover:bg-blue-500"), "bg-blue-500");
        assert_eq!(strip_prefixes("sm:hover:text-lg"), "text-lg");
        assert_eq!(strip_prefixes("text-lg"), "text-lg");
    }
}
