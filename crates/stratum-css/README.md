# stratum-css

## Purpose

CSS-in-Rust solution providing type-safe, scoped style generation via the `css!` macro and a runtime style registry.

## Position in Pipeline

```
   stratum-core    stratum-css-macros
        |               |
        +-------+-------+
                |
           stratum-css
                |
        stratum-components
```

Depends on: `stratum-core`, `stratum-css-macros` (proc macros)
Used by: `stratum-components`

## Key Public API

| Item | Description |
|------|-------------|
| `css!` macro | Write CSS inline with compile-time validation and scoped class names |
| `StyleProps` | Trait for components that accept style overrides |
| `StyleRegistry` | Collects and deduplicates generated styles for SSR extraction |

## Usage Example

```rust
use stratum_css::{css, StyleRegistry};

let class = css! {
    display: "flex";
    align_items: "center";
    gap: "0.5rem";
    color: "var(--color-primary-11)";
};

// Collect all styles for server-side rendering
let registry = StyleRegistry::global();
let stylesheet = registry.to_css_string();
```

## How to Run Tests

```bash
cargo test -p stratum-css
```
