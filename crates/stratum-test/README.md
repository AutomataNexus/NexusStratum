# stratum-test

## Purpose

Test utilities for verifying component rendering, ARIA compliance, keyboard interaction, and end-to-end behavior.

## Position in Pipeline

```
   stratum-leptos   stratum-dioxus
        |                |
        +-------+--------+
                |
          stratum-test
                |
          (test suites)
```

Depends on: `stratum-leptos`, `stratum-dioxus`
Used by: Test suites across the workspace

## Key Public API

| Item | Description |
|------|-------------|
| Render tests | Mount components in a virtual DOM and assert on output |
| ARIA tests | Assert correct ARIA roles, attributes, and live region announcements |
| Keyboard tests | Simulate keyboard events and verify focus management |
| E2E tests | Browser-based end-to-end test harness |

## Usage Example

```rust
use stratum_test::{render, screen, keyboard, aria};
use stratum_leptos::Button;

#[test]
fn button_has_correct_role() {
    render(|| view! { <Button>"Click"</Button> });

    let btn = screen::get_by_role("button");
    assert!(btn.is_some());
    aria::assert_role(&btn.unwrap(), "button");
}

#[test]
fn button_responds_to_enter_key() {
    let clicked = std::cell::Cell::new(false);
    render(|| view! { <Button on:click=|_| clicked.set(true)>"Go"</Button> });

    let btn = screen::get_by_role("button").unwrap();
    keyboard::press(&btn, Key::Enter);
    assert!(clicked.get());
}
```

## How to Run Tests

```bash
cargo test -p stratum-test
```
