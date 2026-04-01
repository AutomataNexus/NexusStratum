# stratum-primitives

## Purpose

Headless, unstyled primitive components that implement correct behavior and accessibility without imposing any visual design.

## Position in Pipeline

```
   stratum-core    stratum-a11y
        |               |
        +-------+-------+
                |
        stratum-primitives
                |
        stratum-components
                |
        +-------+-------+
        |               |
   stratum-leptos  stratum-dioxus
```

Depends on: `stratum-core`, `stratum-a11y`
Used by: `stratum-components`

## Key Public API

| Item | Description |
|------|-------------|
| `Pressable` | Handles press interactions across pointer and keyboard input |
| `Disclosure` | Expandable/collapsible content pattern |
| `Dialog` | Modal and non-modal dialog behavior |
| `Select` | Single and multi-select listbox behavior |
| `Tabs` | Tabbed interface with panel management |

## Usage Example

```rust
use stratum_primitives::{Pressable, Dialog, Tabs};

// Pressable with keyboard and pointer support
let pressable = Pressable::new()
    .on_press(|_| println!("pressed"))
    .disabled(false);

// Tabs with automatic activation
let tabs = Tabs::new()
    .add_tab("tab-1", "Overview", overview_panel)
    .add_tab("tab-2", "Details", details_panel)
    .activation(Activation::Automatic);
```

## How to Run Tests

```bash
cargo test -p stratum-primitives
```
