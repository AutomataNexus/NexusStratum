# stratum-cli

## Purpose

Command-line tool for scaffolding projects, adding components, and managing themes.

## Position in Pipeline

```
   stratum-core   stratum-sdk
        |              |
        +------+-------+
               |
         stratum-cli
               |
        (developer terminal)
```

Depends on: `stratum-core`, `stratum-sdk`
Used by: Developers via the command line

## Key Public API

| Command | Description |
|---------|-------------|
| `stratum init` | Scaffold a new NexusStratum project (Leptos or Dioxus) |
| `stratum add` | Add a component to the current project |
| `stratum theme` | Generate or modify a theme file from presets or custom tokens |

## Usage Example

```bash
# Create a new Leptos project with NexusStratum
stratum init my-app --framework leptos

# Add specific components
stratum add button input dialog

# Generate a theme from a preset
stratum theme --preset "zinc" --output src/theme.rs

# Generate Tailwind config from the current theme
stratum theme --tailwind --output tailwind.config.js
```

## How to Run Tests

```bash
cargo test -p stratum-cli
```
