# stratum-motion

## Purpose

Animation and transition system that respects accessibility preferences and provides declarative motion primitives.

## Position in Pipeline

```
   stratum-core    stratum-a11y
        |               |
        +-------+-------+
                |
        stratum-motion
                |
        stratum-components
```

Depends on: `stratum-core`, `stratum-a11y`
Used by: `stratum-components`

## Key Public API

| Item | Description |
|------|-------------|
| `Transition` | Declarative enter/exit transition wrapper |
| `TransitionConfig` | Configuration for duration, delay, and easing |
| `Easing` | Built-in easing functions (ease-in, ease-out, spring, etc.) |
| `AnimationStyle` | CSS keyframe animation definition |

## Usage Example

```rust
use stratum_motion::{Transition, TransitionConfig, Easing};

let config = TransitionConfig::new()
    .duration_ms(200)
    .easing(Easing::EaseOut)
    .reduced_motion(TransitionConfig::crossfade());

let transition = Transition::new(config)
    .enter(css! { opacity: "0"; transform: "translateY(-4px)"; })
    .enter_to(css! { opacity: "1"; transform: "translateY(0)"; })
    .leave(css! { opacity: "1"; })
    .leave_to(css! { opacity: "0"; });
```

## How to Run Tests

```bash
cargo test -p stratum-motion
```
