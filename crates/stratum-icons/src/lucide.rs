//! Lucide icon set — a subset of commonly used icons.
//!
//! All icons from the Lucide project (MIT license).
//! SVG paths are embedded as static strings — zero network requests.

use crate::icon::Icon;

macro_rules! define_icon {
    ($name:ident, $display_name:expr, $svg:expr) => {
        pub static $name: Icon = Icon {
            name: $display_name,
            svg_content: $svg,
            view_box: "0 0 24 24",
        };
    };
}

// Navigation & Arrows
define_icon!(CHEVRON_DOWN, "chevron-down", "<path d=\"m6 9 6 6 6-6\"/>");
define_icon!(CHEVRON_UP, "chevron-up", "<path d=\"m18 15-6-6-6 6\"/>");
define_icon!(CHEVRON_LEFT, "chevron-left", "<path d=\"m15 18-6-6 6-6\"/>");
define_icon!(CHEVRON_RIGHT, "chevron-right", "<path d=\"m9 18 6-6-6-6\"/>");
define_icon!(ARROW_UP, "arrow-up", "<path d=\"m5 12 7-7 7 7\"/><path d=\"M12 19V5\"/>");
define_icon!(ARROW_DOWN, "arrow-down", "<path d=\"m19 12-7 7-7-7\"/><path d=\"M12 5v14\"/>");
define_icon!(ARROW_LEFT, "arrow-left", "<path d=\"m12 19-7-7 7-7\"/><path d=\"M19 12H5\"/>");
define_icon!(ARROW_RIGHT, "arrow-right", "<path d=\"M5 12h14\"/><path d=\"m12 5 7 7-7 7\"/>");

// Actions
define_icon!(X, "x", "<path d=\"M18 6 6 18\"/><path d=\"m6 6 12 12\"/>");
define_icon!(CHECK, "check", "<path d=\"M20 6 9 17l-5-5\"/>");
define_icon!(PLUS, "plus", "<path d=\"M5 12h14\"/><path d=\"M12 5v14\"/>");
define_icon!(MINUS, "minus", "<path d=\"M5 12h14\"/>");
define_icon!(SEARCH, "search", "<circle cx=\"11\" cy=\"11\" r=\"8\"/><path d=\"m21 21-4.3-4.3\"/>");
define_icon!(SETTINGS, "settings", "<path d=\"M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z\"/><circle cx=\"12\" cy=\"12\" r=\"3\"/>");
define_icon!(COPY, "copy", "<rect width=\"14\" height=\"14\" x=\"8\" y=\"8\" rx=\"2\" ry=\"2\"/><path d=\"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2\"/>");
define_icon!(TRASH, "trash", "<path d=\"M3 6h18\"/><path d=\"M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6\"/><path d=\"M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2\"/>");
define_icon!(EDIT, "edit", "<path d=\"M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z\"/>");

// UI Elements
define_icon!(MENU, "menu", "<line x1=\"4\" x2=\"20\" y1=\"12\" y2=\"12\"/><line x1=\"4\" x2=\"20\" y1=\"6\" y2=\"6\"/><line x1=\"4\" x2=\"20\" y1=\"18\" y2=\"18\"/>");
define_icon!(MORE_HORIZONTAL, "more-horizontal", "<circle cx=\"12\" cy=\"12\" r=\"1\"/><circle cx=\"19\" cy=\"12\" r=\"1\"/><circle cx=\"5\" cy=\"12\" r=\"1\"/>");
define_icon!(MORE_VERTICAL, "more-vertical", "<circle cx=\"12\" cy=\"12\" r=\"1\"/><circle cx=\"12\" cy=\"5\" r=\"1\"/><circle cx=\"12\" cy=\"19\" r=\"1\"/>");
define_icon!(EXTERNAL_LINK, "external-link", "<path d=\"M15 3h6v6\"/><path d=\"M10 14 21 3\"/><path d=\"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6\"/>");

// Status & Feedback
define_icon!(ALERT_CIRCLE, "alert-circle", "<circle cx=\"12\" cy=\"12\" r=\"10\"/><line x1=\"12\" x2=\"12\" y1=\"8\" y2=\"12\"/><line x1=\"12\" x2=\"12.01\" y1=\"16\" y2=\"16\"/>");
define_icon!(ALERT_TRIANGLE, "alert-triangle", "<path d=\"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3\"/><path d=\"M12 9v4\"/><path d=\"M12 17h.01\"/>");
define_icon!(INFO, "info", "<circle cx=\"12\" cy=\"12\" r=\"10\"/><path d=\"M12 16v-4\"/><path d=\"M12 8h.01\"/>");
define_icon!(CHECK_CIRCLE, "check-circle", "<path d=\"M22 11.08V12a10 10 0 1 1-5.93-9.14\"/><path d=\"m9 11 3 3L22 4\"/>");

// Form Elements
define_icon!(EYE, "eye", "<path d=\"M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0\"/><circle cx=\"12\" cy=\"12\" r=\"3\"/>");
define_icon!(EYE_OFF, "eye-off", "<path d=\"M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49\"/><path d=\"M14.084 14.158a3 3 0 0 1-4.242-4.242\"/><path d=\"M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143\"/><path d=\"m2 2 20 20\"/>");
define_icon!(CALENDAR, "calendar", "<path d=\"M8 2v4\"/><path d=\"M16 2v4\"/><rect width=\"18\" height=\"18\" x=\"3\" y=\"4\" rx=\"2\"/><path d=\"M3 10h18\"/>");
define_icon!(CLOCK, "clock", "<circle cx=\"12\" cy=\"12\" r=\"10\"/><polyline points=\"12 6 12 12 16 14\"/>");
define_icon!(UPLOAD, "upload", "<path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\"/><polyline points=\"17 8 12 3 7 8\"/><line x1=\"12\" x2=\"12\" y1=\"3\" y2=\"15\"/>");
define_icon!(DOWNLOAD, "download", "<path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\"/><polyline points=\"7 10 12 15 17 10\"/><line x1=\"12\" x2=\"12\" y1=\"15\" y2=\"3\"/>");

// Layout
define_icon!(GRIP_VERTICAL, "grip-vertical", "<circle cx=\"9\" cy=\"12\" r=\"1\"/><circle cx=\"9\" cy=\"5\" r=\"1\"/><circle cx=\"9\" cy=\"19\" r=\"1\"/><circle cx=\"15\" cy=\"12\" r=\"1\"/><circle cx=\"15\" cy=\"5\" r=\"1\"/><circle cx=\"15\" cy=\"19\" r=\"1\"/>");
define_icon!(MAXIMIZE, "maximize", "<path d=\"M8 3H5a2 2 0 0 0-2 2v3\"/><path d=\"M21 8V5a2 2 0 0 0-2-2h-3\"/><path d=\"M3 16v3a2 2 0 0 0 2 2h3\"/><path d=\"M16 21h3a2 2 0 0 0 2-2v-3\"/>");
define_icon!(MINIMIZE, "minimize", "<path d=\"M8 3v3a2 2 0 0 1-2 2H3\"/><path d=\"M21 8h-3a2 2 0 0 1-2-2V3\"/><path d=\"M3 16h3a2 2 0 0 1 2 2v3\"/><path d=\"M16 21v-3a2 2 0 0 1 2-2h3\"/>");

// Media
define_icon!(IMAGE, "image", "<rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\" ry=\"2\"/><circle cx=\"9\" cy=\"9\" r=\"2\"/><path d=\"m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21\"/>");

// Misc
define_icon!(LOADER, "loader", "<path d=\"M12 2v4\"/><path d=\"m16.2 7.8 2.9-2.9\"/><path d=\"M18 12h4\"/><path d=\"m16.2 16.2 2.9 2.9\"/><path d=\"M12 18v4\"/><path d=\"m4.9 19.1 2.9-2.9\"/><path d=\"M2 12h4\"/><path d=\"m4.9 4.9 2.9 2.9\"/>");
define_icon!(SUN, "sun", "<circle cx=\"12\" cy=\"12\" r=\"4\"/><path d=\"M12 2v2\"/><path d=\"M12 20v2\"/><path d=\"m4.93 4.93 1.41 1.41\"/><path d=\"m17.66 17.66 1.41 1.41\"/><path d=\"M2 12h2\"/><path d=\"M20 12h2\"/><path d=\"m6.34 17.66-1.41 1.41\"/><path d=\"m19.07 4.93-1.41 1.41\"/>");
define_icon!(MOON, "moon", "<path d=\"M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z\"/>");
define_icon!(USER, "user", "<path d=\"M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2\"/><circle cx=\"12\" cy=\"7\" r=\"4\"/>");
define_icon!(HOME, "home", "<path d=\"M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8\"/><path d=\"M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\"/>");
define_icon!(STAR, "star", "<path d=\"M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a.53.53 0 0 0 .4.29l5.16.754a.53.53 0 0 1 .294.904l-3.733 3.638a.53.53 0 0 0-.152.469l.882 5.14a.53.53 0 0 1-.77.56L12.3 15.903a.53.53 0 0 0-.493 0l-4.616 2.426a.53.53 0 0 1-.77-.56l.881-5.139a.53.53 0 0 0-.152-.47L3.42 8.922a.53.53 0 0 1 .294-.906l5.165-.755a.53.53 0 0 0 .4-.29z\"/>");
define_icon!(HEART, "heart", "<path d=\"M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z\"/>");
define_icon!(MAIL, "mail", "<rect width=\"20\" height=\"16\" x=\"2\" y=\"4\" rx=\"2\"/><path d=\"m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7\"/>");
define_icon!(BELL, "bell", "<path d=\"M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9\"/><path d=\"M10.3 21a1.94 1.94 0 0 0 3.4 0\"/>");
define_icon!(LOCK, "lock", "<rect width=\"18\" height=\"11\" x=\"3\" y=\"11\" rx=\"2\" ry=\"2\"/><path d=\"M7 11V7a5 5 0 0 1 10 0v4\"/>");
define_icon!(SHIELD, "shield", "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"/>");
define_icon!(SHIELD_CHECK, "shield-check", "<path d=\"M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z\"/><path d=\"m9 12 2 2 4-4\"/>");

/// Look up an icon by name (case-insensitive, supports both "chevron-down" and "chevron_down").
pub fn by_name(name: &str) -> Option<&'static Icon> {
    let normalized = name.to_lowercase().replace('_', "-");
    match normalized.as_str() {
        "chevron-down" => Some(&CHEVRON_DOWN),
        "chevron-up" => Some(&CHEVRON_UP),
        "chevron-left" => Some(&CHEVRON_LEFT),
        "chevron-right" => Some(&CHEVRON_RIGHT),
        "arrow-up" => Some(&ARROW_UP),
        "arrow-down" => Some(&ARROW_DOWN),
        "arrow-left" => Some(&ARROW_LEFT),
        "arrow-right" => Some(&ARROW_RIGHT),
        "x" => Some(&X),
        "check" => Some(&CHECK),
        "plus" => Some(&PLUS),
        "minus" => Some(&MINUS),
        "search" => Some(&SEARCH),
        "settings" => Some(&SETTINGS),
        "copy" => Some(&COPY),
        "trash" => Some(&TRASH),
        "edit" => Some(&EDIT),
        "menu" => Some(&MENU),
        "more-horizontal" => Some(&MORE_HORIZONTAL),
        "more-vertical" => Some(&MORE_VERTICAL),
        "external-link" => Some(&EXTERNAL_LINK),
        "alert-circle" => Some(&ALERT_CIRCLE),
        "alert-triangle" => Some(&ALERT_TRIANGLE),
        "info" => Some(&INFO),
        "check-circle" => Some(&CHECK_CIRCLE),
        "eye" => Some(&EYE),
        "eye-off" => Some(&EYE_OFF),
        "calendar" => Some(&CALENDAR),
        "clock" => Some(&CLOCK),
        "upload" => Some(&UPLOAD),
        "download" => Some(&DOWNLOAD),
        "grip-vertical" => Some(&GRIP_VERTICAL),
        "maximize" => Some(&MAXIMIZE),
        "minimize" => Some(&MINIMIZE),
        "image" => Some(&IMAGE),
        "loader" => Some(&LOADER),
        "sun" => Some(&SUN),
        "moon" => Some(&MOON),
        "user" => Some(&USER),
        "home" => Some(&HOME),
        "star" => Some(&STAR),
        "heart" => Some(&HEART),
        "mail" => Some(&MAIL),
        "bell" => Some(&BELL),
        "lock" => Some(&LOCK),
        "shield" => Some(&SHIELD),
        "shield-check" => Some(&SHIELD_CHECK),
        _ => None,
    }
}

/// List all available icon names.
pub fn all_names() -> &'static [&'static str] {
    &[
        "chevron-down", "chevron-up", "chevron-left", "chevron-right",
        "arrow-up", "arrow-down", "arrow-left", "arrow-right",
        "x", "check", "plus", "minus", "search", "settings", "copy", "trash", "edit",
        "menu", "more-horizontal", "more-vertical", "external-link",
        "alert-circle", "alert-triangle", "info", "check-circle",
        "eye", "eye-off", "calendar", "clock", "upload", "download",
        "grip-vertical", "maximize", "minimize", "image",
        "loader", "sun", "moon", "user", "home", "star", "heart",
        "mail", "bell", "lock", "shield", "shield-check",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icon::IconProps;

    #[test]
    fn by_name_finds_icons() {
        assert!(by_name("chevron-down").is_some());
        assert!(by_name("x").is_some());
        assert!(by_name("search").is_some());
    }

    #[test]
    fn by_name_case_insensitive() {
        assert!(by_name("CHEVRON-DOWN").is_some());
        assert!(by_name("Chevron-Down").is_some());
    }

    #[test]
    fn by_name_underscore_support() {
        assert!(by_name("chevron_down").is_some());
        assert!(by_name("alert_circle").is_some());
    }

    #[test]
    fn by_name_unknown_returns_none() {
        assert!(by_name("nonexistent-icon").is_none());
    }

    #[test]
    fn all_names_are_findable() {
        for name in all_names() {
            assert!(
                by_name(name).is_some(),
                "Icon '{}' listed in all_names() but not findable via by_name()",
                name
            );
        }
    }

    #[test]
    fn icon_renders_valid_svg() {
        let icon = by_name("check").unwrap();
        let svg = icon.render(&IconProps::default());
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));
    }

    #[test]
    fn icon_count() {
        assert!(all_names().len() >= 45);
    }
}
