# Changelog

All notable changes to NexusStratum will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace with 19 crates (16 library + 2 proc-macro + 1 binary)
- stratum-core: Component trait, Props, State, ComponentEvent, AriaAttributes (40+ fields), AriaRole (65+ roles), FocusManager, IdGenerator, Callback system, security module (XSS prevention, CSS injection validation)
- stratum-a11y: KeyboardNav (ARIA APG patterns), LiveRegion, media query helpers, FocusVisibleStrategy
- stratum-primitives: 23 headless primitives (Pressable, Checkbox, Radio, Switch, Disclosure, Dialog, AlertDialog, Popover, Tooltip, Tabs, Accordion, Menu, Select, TextInput, TextArea, Toast, Progress, Form, Toggle, Separator, Collapsible, Portal, FocusScope, VisuallyHidden) — all with ARIA compliance, keyboard navigation, controlled/uncontrolled modes
- stratum-theme: 7 built-in themes (default, slate, zinc, rose, blue, green, orange), CSS custom property generation including font weights/leading/tracking, Hsl with NaN/Infinity clamping, builder methods for primary/secondary/accent/destructive/spacing/shadows/fonts
- stratum-css: css! proc macro with FNV-1a 64-bit hashing, single-mutex StyleRegistry, StyleProps with padding/margin precedence logic
- stratum-tailwind: tw! proc macro with variant-aware class merging, 30+ utility groups, negative margin support, ClassBuilder, TailwindConfig generation
- stratum-icons: 45+ Lucide icons as inline SVG, XSS-safe rendering with color/class escaping, aria-label/aria-hidden mutual exclusivity
- stratum-motion: Transition presets with separate enter/exit timing, spring easing with overshoot, reduced-motion support, Serialize/Deserialize on AnimationStyle
- stratum-security: CspNonce and CsrfToken with CSPRNG (getrandom), constant-time comparison without length leakage, SRI hash generation (SHA-256), SecurityHeaders (Permissions-Policy, X-XSS-Protection: 0), improved HTML tag stripping
- stratum-components: 30+ styled components across 7 categories (Forms, Overlay, Navigation, Data Display, Layout, Typography, Utility) with Tailwind class generation and RenderOutput
- stratum-leptos: StratumAdapter (DOM event conversion), ThemeContext, ToasterContext, component re-exports
- stratum-cli: All commands implemented — init (creates stratum.toml/components dir), add (writes component source files), theme list/create/apply, diff (GitHub compare URL), docs (opens browser), explorer, list
- stratum-test: Test utility functions (assert_aria_role, assert_has_class, assert_has_attr, etc.)
- stratum (umbrella): Feature-gated re-exports for all subcrates
- Landing site at stratum-ui.com: 35 pages (landing, components catalog, 28 component detail pages, docs, installation, blocks)
- Logo and favicons
- Documentation: README.md, PROGRESS.md, ARCHITECTURE.md, ACCESSIBILITY.md, 19 per-crate READMEs

### Fixed
- All CRITICAL, HIGH, and MEDIUM audit findings resolved across 19 crates
- Controlled-mode bugs in Dialog, Popover, Tabs, Accordion, Tooltip, AlertDialog, Select, Menu, Collapsible
- Pressable no longer sets pressed=true on hover
- ARIA merge now covers all 40+ fields (13 previously missing)
- RenderOutput.with_class validates via is_safe_class_name, with_style via is_safe_css_value
- escape_attr now escapes / and = characters
- All unused dependencies removed across workspace
- Components use HTML id attr instead of data-*-id
- Textarea emits HTML required/readonly attributes
