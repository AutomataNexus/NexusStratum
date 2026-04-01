# stratum-leptos

## Purpose

Leptos framework adapter that exposes all NexusStratum components as native Leptos `#[component]` functions.

## Position in Pipeline

```
        stratum-components
                |
          stratum-leptos
                |
          applications
```

Depends on: `stratum-components`
Used by: Leptos applications, `stratum-test`, `stratum-explorer`

## Key Public API

| Item | Description |
|------|-------------|
| All components as `#[component]` fns | `Button`, `Input`, `Dialog`, `Select`, `Tabs`, etc. |
| `ThemeProvider` | Leptos context provider for injecting a theme |
| `Icon` | Leptos component wrapping `stratum-icons` |

## Usage Example

```rust
use leptos::*;
use stratum_leptos::{Button, ThemeProvider, Input};
use stratum_theme::Theme;

#[component]
fn App() -> impl IntoView {
    let theme = Theme::default();

    view! {
        <ThemeProvider theme=theme>
            <Button variant="solid" on:click=|_| log!("clicked")>
                "Save"
            </Button>
            <Input placeholder="Email" input_type="email" />
        </ThemeProvider>
    }
}
```

## How to Run Tests

```bash
cargo test -p stratum-leptos
```
