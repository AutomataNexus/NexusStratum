# stratum-tailwind-macros

## Purpose

Procedural macros that power the `tw!` macro in `stratum-tailwind`, handling compile-time validation and class merging.

## Position in Pipeline

```
   stratum-tailwind-macros
            |
     stratum-tailwind
            |
     stratum-components
```

No dependencies on other stratum crates (proc-macro crates cannot depend on runtime crates).
Used by: `stratum-tailwind`

## Key Public API

| Item | Description |
|------|-------------|
| `tw!` proc macro | Parses Tailwind utility classes at compile time, validates them, and emits merged class strings |

## Usage Example

This crate is not used directly. It is re-exported through `stratum-tailwind`:

```rust
// In stratum-tailwind (re-exports the proc macro)
pub use stratum_tailwind_macros::tw;

// End users write:
use stratum_tailwind::tw;

let class = tw!["flex", "items-center", "gap-2"];
```

## How to Run Tests

```bash
cargo test -p stratum-tailwind-macros
```
