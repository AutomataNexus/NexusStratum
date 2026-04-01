# stratum-a11y

## Purpose

Accessibility utilities for keyboard navigation, live regions, and user preference detection.

## Position in Pipeline

```
   stratum-core
        |
   stratum-a11y
        |
   +----+--------+
   |              |
stratum-primitives  stratum-motion
   |
stratum-components
```

Depends on: `stratum-core`
Used by: `stratum-primitives`, `stratum-motion`

## Key Public API

| Item | Description |
|------|-------------|
| `KeyboardNav` | Manages keyboard navigation patterns (roving tabindex, arrow keys, etc.) |
| `LiveRegion` | ARIA live region announcements for screen readers |
| `prefers_reduced_motion()` | Detects user preference for reduced motion |
| `is_keyboard_user()` | Detects whether the user is navigating via keyboard |

## Usage Example

```rust
use stratum_a11y::{KeyboardNav, LiveRegion, prefers_reduced_motion};

// Set up roving tabindex keyboard navigation
let nav = KeyboardNav::roving_tabindex()
    .orientation(Orientation::Horizontal)
    .wrap(true);

// Announce a status change to screen readers
LiveRegion::polite("3 results found");

// Conditionally disable animations
if prefers_reduced_motion() {
    // skip animation
}
```

## How to Run Tests

```bash
cargo test -p stratum-a11y
```
