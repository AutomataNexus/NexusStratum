# NexusStratum Accessibility Policy

NexusStratum is committed to delivering UI components that are usable by everyone, including people who rely on assistive technologies. This document defines our accessibility standards, implementation patterns, and testing requirements.

## Compliance Target

All NexusStratum components must conform to **WCAG 2.2 Level AA**. No component may be promoted to stable status without passing automated and manual accessibility audits against this standard.

## WCAG Criteria Coverage

The following WCAG 2.2 AA success criteria are directly relevant to NexusStratum components and must be satisfied by every interactive primitive and composite widget.

| Criterion | Name | Requirement |
|-----------|------|-------------|
| 1.4.3 | Contrast (Minimum) | Text must meet a 4.5:1 contrast ratio against its background. Large text (18pt or 14pt bold) must meet 3:1. |
| 1.4.11 | Non-text Contrast | UI components and graphical objects must meet a 3:1 contrast ratio against adjacent colors. This includes borders, focus indicators, icons, and interactive element boundaries. |
| 2.1.1 | Keyboard | All functionality must be operable through a keyboard interface without requiring specific timings for individual keystrokes. |
| 2.1.2 | No Keyboard Trap | If keyboard focus can be moved to a component, focus can also be moved away using only the keyboard. Where non-standard exit methods are required, the user is advised of the method. |
| 2.4.3 | Focus Order | Focusable components receive focus in an order that preserves meaning and operability. |
| 2.4.7 | Focus Visible | Any keyboard-operable UI has a visible focus indicator. NexusStratum enforces a minimum 2px focus ring on all interactive elements. |
| 4.1.2 | Name, Role, Value | All UI components expose their name, role, and current value (or state) to assistive technologies via ARIA attributes and semantic markup. |

## Keyboard Navigation Patterns

Keyboard interactions follow the [ARIA Authoring Practices Guide (APG)](https://www.w3.org/WAI/ARIA/apg/). Each component type implements the key bindings listed below.

| Component | Required Keys | Behavior |
|-----------|---------------|----------|
| Button | `Enter`, `Space` | Activates the button. |
| Checkbox | `Space` | Toggles checked state. |
| Radio | `Arrow Up/Down`, `Arrow Left/Right`, `Space` | Arrows move selection within the radio group. `Space` selects the focused radio if not already selected. |
| Select / Listbox | `Arrow Up/Down`, `Enter`, Type-ahead | Arrows navigate options. `Enter` confirms selection. Type-ahead jumps to matching option by first character(s). |
| Menu | `Arrow Up/Down`, `Arrow Left/Right`, `Enter`/`Space`, `Escape`, Type-ahead | Arrows navigate items and submenus. `Enter`/`Space` activates. `Escape` closes the menu. Type-ahead jumps to matching item. |
| Dialog | `Escape`, `Tab`/`Shift+Tab` (cycle) | `Escape` closes the dialog. `Tab` and `Shift+Tab` cycle focus within the dialog boundary (focus trap). |
| Tabs | `Arrow Left/Right`, `Arrow Up/Down`, `Enter`/`Space` | Arrows move between tabs. `Enter`/`Space` activates the focused tab (in manual activation mode). |
| Accordion | `Enter`/`Space`, `Arrow Up/Down` | `Enter`/`Space` expands or collapses a section. Arrows move focus between accordion headers. |
| Tree | `Arrow Up/Down`, `Arrow Left/Right`, `Enter` | Up/Down navigate visible nodes. Left collapses or moves to parent. Right expands or moves to first child. `Enter` activates. |
| Slider | `Arrow Left/Right`, `Arrow Up/Down`, `Page Up/Down`, `Home`, `End` | Arrows adjust by step. `Page Up/Down` adjusts by large step. `Home`/`End` sets to minimum/maximum. |
| Date Picker | `Arrow Up/Down/Left/Right`, `Page Up/Down`, `Enter` | Arrows navigate days. `Page Up/Down` navigates months. `Enter` selects the focused date. |

## Screen Reader Testing

NexusStratum targets the following screen reader and browser combinations for manual testing.

| Screen Reader | Browser | Platform | Status |
|---------------|---------|----------|--------|
| NVDA | Chrome | Windows | Testing pending (Phase 1) |
| JAWS | Chrome | Windows | Testing pending (Phase 1) |
| VoiceOver | Safari | macOS / iOS | Testing pending (Phase 1) |
| TalkBack | Chrome | Android | Testing pending (Phase 1) |

Manual screen reader testing will be conducted for every Tier 1 component before it reaches stable status. Testing scripts and expected announcements will be documented per component.

## ARIA Implementation

Every interactive component in NexusStratum receives correct ARIA semantics through the **primitive layer**. Developers building on NexusStratum primitives get accessible components by default, without needing to manually wire ARIA attributes.

### AriaAttributes Struct

The `AriaAttributes` struct is the central mechanism for passing ARIA properties through the component tree. It provides typed fields for all commonly used ARIA attributes:

```rust
pub struct AriaAttributes {
    pub role: Option<AriaRole>,
    pub label: Option<String>,
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub checked: Option<AriaChecked>,
    pub disabled: Option<bool>,
    pub hidden: Option<bool>,
    pub live: Option<AriaLive>,
    pub controls: Option<String>,
    pub owns: Option<String>,
    pub has_popup: Option<AriaHasPopup>,
    pub active_descendant: Option<String>,
    pub value_now: Option<f64>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
    pub value_text: Option<String>,
}
```

### Automatic ID Generation

Components that require cross-referencing (e.g., a label pointing to an input via `aria-labelledby`, or a description via `aria-describedby`) use automatic ID generation. The primitive layer generates stable, unique IDs so that ARIA relationships are correctly established without manual coordination. IDs follow the pattern `ns-{component}-{instance}` to avoid collisions.

## Focus Management

NexusStratum provides a `FocusManager` that implements three focus strategies, each suited to different interaction patterns.

| Strategy | Behavior | Used By |
|----------|----------|---------|
| **Trap** | Focus is constrained within a container. `Tab` and `Shift+Tab` cycle through focusable children without escaping. | Dialog, Modal, Drawer |
| **Restore** | When a component unmounts or closes, focus returns to the element that triggered it. | Dialog, Menu, Popover, Tooltip (interactive) |
| **Initial** | When a component mounts, focus is programmatically moved to a designated initial element. | Dialog (first focusable element or designated target), Date Picker (current date cell) |

Strategies can be composed. For example, a Dialog uses all three: **Initial** to set focus on open, **Trap** to contain focus while open, and **Restore** to return focus on close.

## Reduced Motion

All animations in NexusStratum respect the `prefers-reduced-motion` media query. The `stratum-motion` module checks this preference and, when reduced motion is preferred, makes all animations instant (duration set to `0ms`). This applies to:

- Transitions on component state changes (e.g., accordion expand/collapse)
- Loading spinners and progress indicators (switched to non-animated alternatives)
- Page and view transitions
- Hover and focus animations

Components must never rely on animation to convey information. Motion is decorative and supplementary only.

## High Contrast

NexusStratum supports the `prefers-contrast: more` media query. When high contrast is preferred:

- Border widths on interactive elements are increased to a minimum of 2px.
- Focus indicators use a higher-contrast color and increased thickness.
- Subtle background color distinctions are replaced with visible borders or heavier visual weight.
- All theme tokens include high-contrast overrides that are applied automatically.

Components should not hard-code colors. All color values must come from theme tokens so that high-contrast overrides are picked up without per-component logic.

## Testing Strategy

### Automated Testing with axe-core

NexusStratum integrates [axe-core](https://github.com/dequelabs/axe-core) into its Playwright end-to-end test suite. Every component has a dedicated accessibility test file that:

1. Renders the component in all supported states (default, hover, focus, active, disabled, error).
2. Runs `axe-core` against the rendered DOM.
3. Asserts **zero violations** at the WCAG 2.2 AA level.

**Zero violations is a hard requirement.** A component with any axe-core violation at AA level cannot pass CI.

### Running Accessibility Tests

```bash
# Run all accessibility tests
cargo make test-a11y

# Run accessibility tests for a specific component
cargo make test-a11y -- --component button

# Run accessibility tests and generate an HTML report
cargo make test-a11y -- --report
```

### What the Tests Cover

- ARIA role and attribute correctness
- Label and name computation
- Color contrast ratios
- Focus order and keyboard operability
- Landmark and heading structure

### What Requires Manual Testing

- Screen reader announcement accuracy (see Screen Reader Testing above)
- Cognitive accessibility and plain language
- Touch target sizing on mobile
- Real-world assistive technology compatibility

## Known Issues

No known accessibility issues at this time. Issues will be logged here as they are discovered, with references to tracking tickets and target resolution dates.

## Component Compliance Status

The table below tracks the accessibility compliance status of all Tier 1 components.

| Component | ARIA | Keyboard | Contrast | Screen Reader | Status |
|-----------|------|----------|----------|---------------|--------|
| Button/Pressable | Done | Done | Pending | Pending | Implemented |
| Checkbox | Done | Done | Pending | Pending | Implemented |
| Radio | Done | Done | Pending | Pending | Implemented |
| Select | Done | Done | Pending | Pending | Implemented |
| Input | Done | Done | Pending | Pending | Implemented |
| Textarea | Done | Done | Pending | Pending | Implemented |
| Switch | Done | Done | Pending | Pending | Implemented |
| Dialog | Done | Done | Pending | Pending | Implemented |
| AlertDialog | Done | Done | Pending | Pending | Implemented |
| Menu | Done | Done | Pending | Pending | Implemented |
| Tabs | Done | Done | Pending | Pending | Implemented |
| Accordion | Done | Done | Pending | Pending | Implemented |
| Tooltip | Done | Done | Pending | Pending | Implemented |
| Popover | Done | Done | Pending | Pending | Implemented |
| Toast | Done | N/A | Pending | Pending | Implemented |
| Progress | Done | N/A | Pending | Pending | Implemented |
| Form | Done | Done | Pending | Pending | Implemented |
| Toggle | Done | Done | Pending | Pending | Implemented |
| Separator | Done | N/A | Pending | Pending | Implemented |
| Collapsible | Done | Done | Pending | Pending | Implemented |
| Disclosure | Done | Done | Pending | Pending | Implemented |
| FocusScope | Done | Done | Pending | Pending | Implemented |
| VisuallyHidden | Done | N/A | N/A | Pending | Implemented |

**Note:** ARIA and Keyboard are implemented and unit-tested in stratum-primitives. Contrast checking requires visual testing against rendered output. Screen reader testing requires manual verification with NVDA/JAWS/VoiceOver.
