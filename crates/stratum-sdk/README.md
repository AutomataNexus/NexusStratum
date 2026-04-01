# stratum-sdk

## Purpose

Editor tooling SDK providing component metadata extraction, JSON schema generation, and the `#[stratum_component]` attribute macro.

## Position in Pipeline

```
        stratum-components
                |
           stratum-sdk
                |
        +-------+-------+
        |               |
   stratum-explorer  stratum-cli
```

Depends on: `stratum-components`
Used by: `stratum-explorer`, `stratum-cli`

## Key Public API

| Item | Description |
|------|-------------|
| `#[stratum_component]` macro | Annotates a component to emit metadata for tooling |
| JSON schema generation | Produces JSON Schema for component props (for editor autocomplete) |
| Component metadata | Runtime-queryable info about props, variants, slots, and events |

## Usage Example

```rust
use stratum_sdk::stratum_component;

#[stratum_component]
struct MyCard {
    /// The card title
    title: String,
    /// Visual variant
    #[variant("elevated", "outlined", "flat")]
    variant: String,
}

// Generate JSON schema for editor integration
let schema = stratum_sdk::schema_for::<MyCard>();
std::fs::write("my-card.schema.json", schema.to_json()).unwrap();
```

## How to Run Tests

```bash
cargo test -p stratum-sdk
```
