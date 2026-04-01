# stratum-dioxus

## Purpose

Dioxus framework adapter that exposes all NexusStratum components as native Dioxus `#[component]` functions.

## Position in Pipeline

```
        stratum-components
                |
          stratum-dioxus
                |
          applications
```

Depends on: `stratum-components`
Used by: Dioxus applications, `stratum-test`

## Key Public API

| Item | Description |
|------|-------------|
| All components as `#[component]` fns | `Button`, `Input`, `Dialog`, `Select`, `Tabs`, etc. |
| `ThemeProvider` | Dioxus context provider for injecting a theme |
| `Icon` | Dioxus component wrapping `stratum-icons` |

## Usage Example

```rust
use dioxus::prelude::*;
use stratum_dioxus::{Button, ThemeProvider, Input};
use stratum_theme::Theme;

#[component]
fn App() -> Element {
    let theme = Theme::default();

    rsx! {
        ThemeProvider { theme: theme,
            Button { variant: "solid", onclick: |_| log!("clicked"),
                "Save"
            }
            Input { placeholder: "Email", input_type: "email" }
        }
    }
}
```

## How to Run Tests

```bash
cargo test -p stratum-dioxus
```
