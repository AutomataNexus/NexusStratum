# stratum-css-macros

## Purpose

Procedural macros that power the `css!` macro in `stratum-css`, handling compile-time CSS parsing and class name generation.

## Position in Pipeline

```
   stratum-css-macros
          |
     stratum-css
          |
   stratum-components
```

No dependencies on other stratum crates (proc-macro crates cannot depend on runtime crates).
Used by: `stratum-css`

## Key Public API

| Item | Description |
|------|-------------|
| `css!` proc macro | Parses CSS-like syntax at compile time and emits scoped class names and style registration code |

## Usage Example

This crate is not used directly. It is re-exported through `stratum-css`:

```rust
// In stratum-css (re-exports the proc macro)
pub use stratum_css_macros::css;

// End users write:
use stratum_css::css;

let class = css! {
    padding: "1rem";
    background: "var(--color-neutral-2)";
};
```

## How to Run Tests

```bash
cargo test -p stratum-css-macros
```
