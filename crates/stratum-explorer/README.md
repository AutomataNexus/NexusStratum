# stratum-explorer

## Purpose

Component explorer (Storybook equivalent) for browsing, testing, and documenting NexusStratum components in the browser.

## Position in Pipeline

```
   stratum-leptos   stratum-sdk
        |               |
        +-------+-------+
                |
        stratum-explorer
                |
        (dev server / browser)
```

Depends on: `stratum-leptos`, `stratum-sdk`
Used by: Developers during component development

## Key Public API

| Item | Description |
|------|-------------|
| `#[story]` macro | Annotates a function as a component story with metadata |
| `run!` macro | Launches the explorer dev server |
| Story controls | Auto-generated controls from component props |

## Usage Example

```rust
use stratum_explorer::{story, run};
use stratum_leptos::Button;

#[story(name = "Primary Button", category = "Actions")]
fn primary_button() -> impl IntoView {
    view! {
        <Button variant="solid" size="md">
            "Click me"
        </Button>
    }
}

fn main() {
    run! {
        title: "NexusStratum Explorer",
        stories: [primary_button],
    };
}
```

## How to Run Tests

```bash
cargo test -p stratum-explorer
```
