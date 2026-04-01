# stratum-icons

## Purpose

Icon library providing framework-agnostic SVG icon components with support for Lucide icons and custom icon registration.

## Position in Pipeline

```
        stratum-core
             |
        stratum-icons
             |
   +---------+---------+
   |         |         |
stratum-  stratum-  stratum-
components leptos   dioxus
```

Depends on: `stratum-core`
Used by: `stratum-components`, `stratum-leptos`, `stratum-dioxus`

## Key Public API

| Item | Description |
|------|-------------|
| `IconProps` | Common props for all icons (size, color, stroke-width, etc.) |
| Lucide icons | Full set of Lucide icons as individual types |
| `register!` macro | Register custom SVG icons for use alongside built-in icons |

## Usage Example

```rust
use stratum_icons::{IconProps, lucide, register};

// Use a built-in Lucide icon
let props = IconProps::new()
    .size("24")
    .color("currentColor")
    .stroke_width(2.0);

let icon = lucide::ChevronDown(props);

// Register a custom icon
register! {
    name: "my-logo",
    svg: include_str!("../assets/logo.svg"),
}
```

## How to Run Tests

```bash
cargo test -p stratum-icons
```
