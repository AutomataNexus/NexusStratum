# stratum-theme

## Purpose

Design token system for defining and distributing colors, typography, spacing, and other visual tokens across components.

## Position in Pipeline

```
        stratum-core
             |
        stratum-theme
             |
    +--------+--------+
    |                  |
stratum-components  stratum-tailwind
    |
+---+---+
|       |
leptos  dioxus
```

Depends on: `stratum-core`
Used by: `stratum-components`, `stratum-tailwind`

## Key Public API

| Item | Description |
|------|-------------|
| `Theme` | Complete theme definition containing all design tokens |
| `ColorScale` | 12-step color scale (inspired by Radix Colors) |
| `Typography` | Font family, size, weight, and line-height tokens |
| `ThemeProvider` | Context provider that makes a theme available to descendants |

## Usage Example

```rust
use stratum_theme::{Theme, ColorScale, Typography, ThemeProvider};

let theme = Theme::builder()
    .color("primary", ColorScale::blue())
    .color("neutral", ColorScale::slate())
    .typography(Typography::default())
    .radius("md", "0.375rem")
    .build();

// Provide theme to component tree
ThemeProvider::new(theme)
    .child(app)
    .render();
```

## How to Run Tests

```bash
cargo test -p stratum-theme
```
