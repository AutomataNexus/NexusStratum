# NexusStratum

**The composable UI component library for Rust frontends.**

[![CI](https://img.shields.io/github/actions/workflow/status/AutomataNexus/NexusStratum/ci.yml?branch=main&label=CI)](https://github.com/AutomataNexus/NexusStratum/actions)
[![crates.io](https://img.shields.io/crates/v/nexus-stratum.svg)](https://crates.io/crates/nexus-stratum)
[![docs.rs](https://img.shields.io/docsrs/nexus-stratum)](https://docs.rs/nexus-stratum)
[![MSRV](https://img.shields.io/badge/MSRV-1.75-blue.svg)](https://github.com/AutomataNexus/NexusStratum)
[![License](https://img.shields.io/crates/l/nexus-stratum.svg)](https://github.com/AutomataNexus/NexusStratum#license)
[![Downloads](https://img.shields.io/crates/d/nexus-stratum.svg)](https://crates.io/crates/nexus-stratum)

---

## What is NexusStratum?

NexusStratum is the **shadcn/ui of the Rust ecosystem** -- a production-ready, framework-agnostic UI component library designed for Rust frontend development. It provides 50+ accessible, themeable, and fully composable components that target both Leptos and Dioxus with zero compromises. Components are built in layers: a headless primitives core handles behavior and accessibility, a styled layer applies design tokens and themes, and thin framework adapters expose idiomatic APIs for each supported framework. You own the code, you own the styling, and you never fight the library.

---

## Architecture

```
                         Application Code
                 --------------------------------
                |                                |
        stratum-leptos                   stratum-dioxus
          (Leptos adapter)                 (Dioxus adapter)
                |                                |
                 --------------------------------
                              |
                    stratum-components
                      (styled layer)
                              |
                    stratum-primitives
                     (headless layer)
                              |
                       stratum-core
                      (foundation)
                              |
          ------------------------------------------------
         |            |              |           |        |
   stratum-theme  stratum-a11y  stratum-css  stratum-tailwind  stratum-icons
    (design        (ARIA &       (CSS-in-     (Tailwind     (icon
     tokens &       focus         Rust         utility       sets)
     theming)       management)   engine)      classes)
                                                    |
                                              stratum-motion
                                               (transitions &
                                                animations)

   ---------------------------
   |        Tooling          |
   ---------------------------
   | stratum-cli       CLI for scaffolding, adding components, and codegen  |
   | stratum-explorer   Interactive component playground (WASM)             |
   | stratum-sdk        Plugin API for third-party component authors        |
   | stratum-test       Testing utilities and accessibility audits          |
   -------------------------------------------------------------------------
```

---

## Quick Start

Add NexusStratum to your project:

```toml
# Cargo.toml
[dependencies]
stratum-leptos = "0.1"
```

Render your first Button in five lines:

```rust
use stratum_leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! { <Button variant=ButtonVariant::Primary on_click=|_| log!("clicked")>"Get Started"</Button> }
}
```

That is all it takes. The component ships with sensible defaults, full keyboard support, and ARIA attributes out of the box.

---

## Components

NexusStratum ships 50+ components across nine categories. Each component is available as a headless primitive and as a styled, ready-to-use element.

| Category | Component | Phase |
|---|---|---|
| **Layout** | Box | Phase 1 |
| | Flex | Phase 1 |
| | Grid | Phase 1 |
| | Stack | Phase 1 |
| | Container | Phase 1 |
| | Center | Phase 1 |
| | Spacer | Phase 1 |
| | Divider | Phase 1 |
| | AspectRatio | Phase 2 |
| **Typography** | Heading | Phase 1 |
| | Text | Phase 1 |
| | Code | Phase 1 |
| | Kbd | Phase 2 |
| | Blockquote | Phase 2 |
| | Label | Phase 1 |
| **Forms** | Button | Phase 1 |
| | IconButton | Phase 1 |
| | Input | Phase 1 |
| | Textarea | Phase 1 |
| | Select | Phase 1 |
| | Checkbox | Phase 1 |
| | Radio | Phase 1 |
| | Switch | Phase 1 |
| | Slider | Phase 2 |
| | RangeSlider | Phase 2 |
| | NumberInput | Phase 2 |
| | PinInput | Phase 3 |
| | FileUpload | Phase 3 |
| | DatePicker | Phase 3 |
| | ColorPicker | Phase 3 |
| | FormControl | Phase 1 |
| **Overlay** | Modal | Phase 1 |
| | Dialog | Phase 1 |
| | Drawer | Phase 2 |
| | Popover | Phase 2 |
| | Tooltip | Phase 1 |
| | ContextMenu | Phase 3 |
| | AlertDialog | Phase 2 |
| **Navigation** | Tabs | Phase 1 |
| | Breadcrumb | Phase 1 |
| | Link | Phase 1 |
| | NavBar | Phase 2 |
| | Sidebar | Phase 2 |
| | Pagination | Phase 2 |
| | Stepper | Phase 3 |
| | CommandPalette | Phase 3 |
| **Data Display** | Table | Phase 1 |
| | Card | Phase 1 |
| | Badge | Phase 1 |
| | Avatar | Phase 1 |
| | Tag | Phase 2 |
| | List | Phase 1 |
| | DataTable | Phase 3 |
| | Timeline | Phase 3 |
| | Stat | Phase 2 |
| **Feedback** | Alert | Phase 1 |
| | Toast | Phase 2 |
| | Progress | Phase 1 |
| | Spinner | Phase 1 |
| | Skeleton | Phase 2 |
| | Notification | Phase 3 |
| **Disclosure** | Accordion | Phase 1 |
| | Collapsible | Phase 2 |
| | Tree | Phase 3 |
| **Utility** | Portal | Phase 1 |
| | VisuallyHidden | Phase 1 |
| | FocusTrap | Phase 2 |
| | ClickOutside | Phase 2 |
| | ScrollArea | Phase 2 |
| | ResizeObserver | Phase 3 |

**Phase 1** -- Core components, available now.
**Phase 2** -- Targeted for next minor release.
**Phase 3** -- Planned, contributions welcome.

---

## Framework Support

| Framework | Minimum Version | Adapter Crate | Status |
|---|---|---|---|
| Leptos | 0.7+ | `stratum-leptos` | Fully supported |
| Dioxus | 0.6+ | `stratum-dioxus` | Fully supported |

Both adapters expose identical component APIs. Code written against `stratum-primitives` is portable between frameworks without modification.

---

## Theme System

NexusStratum includes a token-based theme system that supports light mode, dark mode, and fully custom palettes.

```rust
use stratum_theme::{Theme, ThemeProvider, ColorScale, Radius, FontFamily};

// Use the built-in default theme
let theme = Theme::default();

// Or build a custom theme
let custom = Theme::builder()
    .primary(ColorScale::from_hex("#6366f1"))
    .secondary(ColorScale::from_hex("#8b5cf6"))
    .radius(Radius::Medium)
    .font_family(FontFamily::new("Inter, sans-serif"))
    .dark_mode(true)
    .build();

// Wrap your application with the provider
view! {
    <ThemeProvider theme=custom>
        <App />
    </ThemeProvider>
}
```

Themes propagate through context. Any component in the tree can read the active theme, and nested `ThemeProvider` nodes allow per-section overrides. The theme integrates with both `stratum-css` (CSS-in-Rust) and `stratum-tailwind` (utility classes) styling backends.

---

## CLI Usage

The `stratum-cli` tool handles project scaffolding, component installation, and code generation.

```bash
# Install the CLI
cargo install stratum-cli

# Initialize NexusStratum in an existing project (adds dependencies, creates config)
stratum init

# Add individual components to your project
stratum add button
stratum add modal accordion tabs

# Add an entire category
stratum add --category forms

# List available components
stratum list
```

`stratum init` detects your framework (Leptos or Dioxus), configures the appropriate adapter crate, and writes a `stratum.toml` configuration file. `stratum add` copies component source into your project so you can customize it freely -- the same "own your code" model that shadcn/ui pioneered.

---

## Explorer

The interactive component explorer lets you browse every component, toggle themes, adjust props in real time, and copy usage snippets.

```bash
# Clone the repository and launch the explorer
git clone https://github.com/AutomataNexus/NexusStratum.git
cd NexusStratum

# Run the explorer (serves on http://localhost:3000)
cargo run -p stratum-explorer
```

The explorer is a Trunk-based WASM application. It requires `trunk` (`cargo install trunk`) and a recent stable Rust toolchain.

---

## Contributing

Contributions are welcome. To add a new component:

1. **Primitive first.** Create the headless primitive in `crates/stratum-primitives/src/components/`. The primitive must handle all keyboard interactions, ARIA attributes, and state management with zero styling assumptions.

2. **Styled layer.** Add the styled wrapper in `crates/stratum-components/src/components/`. Apply design tokens from `stratum-theme` and ensure the component renders correctly in both light and dark modes.

3. **Framework adapters.** Expose the component through `crates/stratum-leptos/` and `crates/stratum-dioxus/` with idiomatic APIs for each framework.

4. **Tests.** Add unit tests in the component crate and accessibility tests using `stratum-test`. All components must pass `axe-core` equivalent checks.

5. **Documentation.** Add a page to `stratum-explorer` demonstrating the component with interactive controls.

6. **Open a PR.** Ensure `cargo fmt`, `cargo clippy`, and `cargo test --workspace` all pass before submitting.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide, coding conventions, and commit message format.

---

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Copyright (c) Andrew Jewell Sr. -- AutomataNexus LLC
