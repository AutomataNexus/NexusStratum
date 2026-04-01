# stratum

## Purpose

Meta-crate that re-exports all NexusStratum subcrates under a single dependency, controlled by feature flags.

## Position in Pipeline

```
   core  a11y  primitives  theme  css  tailwind  icons  motion
     |    |       |          |     |      |        |      |
     +----+-------+----------+-----+------+--------+------+
                             |
   components  leptos  dioxus  sdk  test  explorer  cli  security
        |         |       |     |    |       |       |      |
        +---------+-------+-----+----+-------+------+------+
                             |
                          stratum
                             |
                        applications
```

Depends on: All other stratum crates (gated by feature flags)
Used by: End-user applications

## Feature Flags

| Flag | Enables |
|------|---------|
| `leptos` | `stratum-leptos` adapter |
| `dioxus` | `stratum-dioxus` adapter |
| `tailwind` | `stratum-tailwind` styling |
| `css` | `stratum-css` styling |
| `icons` | `stratum-icons` icon library |
| `motion` | `stratum-motion` animations |
| `security` | `stratum-security` hardening |
| `full` | All of the above |

## Key Public API

All types are re-exported from their respective subcrates. With the `full` feature:

- `stratum::Button`, `stratum::Input`, etc. (components)
- `stratum::Theme`, `stratum::ThemeProvider` (theming)
- `stratum::tw!`, `stratum::css!` (styling)
- `stratum::Transition` (motion)

## Usage Example

```toml
# Cargo.toml
[dependencies]
stratum = { version = "0.1", features = ["leptos", "tailwind", "icons"] }
```

```rust
use stratum::*;

#[component]
fn App() -> impl IntoView {
    let theme = Theme::default();

    view! {
        <ThemeProvider theme=theme>
            <Button variant="solid">
                <Icon icon=lucide::Check />
                "Confirm"
            </Button>
        </ThemeProvider>
    }
}
```

## How to Run Tests

```bash
# Test with all features
cargo test -p stratum --features full

# Test with specific features
cargo test -p stratum --features leptos,tailwind
```
