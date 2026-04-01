# stratum-tailwind

## Purpose

Tailwind CSS integration providing the `tw!` macro for type-checked utility classes, intelligent class merging, and config generation from theme tokens.

## Position in Pipeline

```
   stratum-core   stratum-theme   stratum-tailwind-macros
        |              |                  |
        +------+-------+------------------+
               |
        stratum-tailwind
               |
        stratum-components
```

Depends on: `stratum-core`, `stratum-theme`, `stratum-tailwind-macros` (proc macros)
Used by: `stratum-components`

## Key Public API

| Item | Description |
|------|-------------|
| `tw!` macro | Compile-time validated Tailwind utility class composition |
| Class merging | Automatic conflict resolution (e.g., last `p-*` wins) |
| Config generation | Generates `tailwind.config.js` from a `Theme` |

## Usage Example

```rust
use stratum_tailwind::tw;

// Compile-time checked utility classes
let class = tw!["flex", "items-center", "gap-2", "text-primary-11"];

// Class merging resolves conflicts
let merged = tw!["p-4", "px-2"]; // px-2 overrides horizontal padding

// Generate Tailwind config from theme tokens
use stratum_tailwind::generate_config;
use stratum_theme::Theme;

let config = generate_config(&Theme::default());
std::fs::write("tailwind.config.js", config).unwrap();
```

## How to Run Tests

```bash
cargo test -p stratum-tailwind
```
