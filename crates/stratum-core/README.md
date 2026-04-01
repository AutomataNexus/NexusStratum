# stratum-core

## Purpose

Foundation crate providing the core traits, types, and utilities that all other NexusStratum crates depend on.

## Position in Pipeline

```
                        stratum-core
                             |
        +----------+---------+----------+---------+----------+
        |          |         |          |         |          |
   stratum-a11y  theme     css      tailwind   icons    security
        |          |         |          |         |
   primitives  components  css     tailwind    icons
        |          |                    |         |
   components  leptos/dioxus       components  leptos/dioxus
```

No dependencies on other stratum crates. Used by every other crate in the workspace.

## Key Public API

| Item | Description |
|------|-------------|
| `Component` trait | Base trait all components implement |
| `Props` trait | Trait for component property structs |
| `ComponentEvent` | Unified event type for component interactions |
| `AriaAttributes` | Struct for WAI-ARIA attribute management |
| `AriaRole` | Enum of ARIA roles |
| `FocusManager` | Utility for managing focus state across components |
| `IdGenerator` | Deterministic unique ID generation for SSR compatibility |
| `RenderOutput` | Framework-agnostic render output type |

## Usage Example

```rust
use stratum_core::{Component, Props, AriaRole, IdGenerator};

#[derive(Props)]
struct MyButtonProps {
    label: String,
    disabled: bool,
}

struct MyButton;

impl Component for MyButton {
    type Props = MyButtonProps;

    fn render(props: &Self::Props) -> RenderOutput {
        let id = IdGenerator::next("my-button");
        RenderOutput::new("button")
            .aria_role(AriaRole::Button)
            .attr("id", &id)
            .text(&props.label)
    }
}
```

## How to Run Tests

```bash
cargo test -p stratum-core
```
