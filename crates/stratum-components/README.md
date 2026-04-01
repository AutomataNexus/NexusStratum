# stratum-components

## Purpose

Styled, accessible component library (50+ components) built on primitives, theme tokens, and the styling subsystem.

## Position in Pipeline

```
  primitives  theme  tailwind  css  icons  motion  security
       |        |       |       |     |      |        |
       +--------+-------+-------+-----+------+--------+
                         |
                 stratum-components
                         |
              +----------+----------+
              |          |          |
         stratum-   stratum-   stratum-sdk
         leptos     dioxus
```

Depends on: `stratum-primitives`, `stratum-theme`, `stratum-tailwind`, `stratum-css`, `stratum-icons`, `stratum-motion`, optionally `stratum-security`
Used by: `stratum-leptos`, `stratum-dioxus`, `stratum-sdk`

## Key Public API

| Item | Description |
|------|-------------|
| `Button` | Button with variants (solid, outline, ghost, link) and sizes |
| `Input` | Text input with validation, prefix/suffix slots |
| `Dialog` | Modal dialog with focus trap and backdrop |
| `Select` | Dropdown select with search, multi-select support |
| `Tabs` | Tabbed interface with lazy and eager panel rendering |
| `Card`, `Badge`, `Avatar`, `Tooltip`, ... | 50+ additional components |

## Usage Example

```rust
use stratum_components::{Button, ButtonVariant, Input, Dialog};

// Button
let btn = Button::new()
    .variant(ButtonVariant::Solid)
    .size("md")
    .label("Save changes");

// Input
let input = Input::new()
    .placeholder("Enter email")
    .input_type("email")
    .required(true);

// Dialog
let dialog = Dialog::new()
    .title("Confirm")
    .description("Are you sure?")
    .body(content)
    .on_confirm(|_| save());
```

## How to Run Tests

```bash
cargo test -p stratum-components
```
