//! # stratum-primitives
//!
//! Headless, unstyled UI primitives for the NexusStratum component library.
//!
//! Each primitive provides framework-agnostic component logic implementing
//! the [`Component`](stratum_core::Component) trait from `stratum-core`.
//! Primitives handle ARIA attributes, keyboard interaction, and state
//! management per WAI-ARIA Authoring Practices Guide (APG) patterns.
//!
//! Framework adapters (stratum-leptos, stratum-dioxus) bridge these
//! primitives to each framework's rendering model.

pub mod alert_dialog;
pub mod checkbox;
pub mod dialog;
pub mod disclosure;
pub mod focus_scope;
pub mod popover;
pub mod portal;
pub mod pressable;
pub mod progress;
pub mod radio;
pub mod separator;
pub mod switch;
pub mod tabs;
pub mod text_area;
pub mod text_input;
pub mod toggle;
pub mod tooltip;
pub mod visually_hidden;
pub mod accordion;
pub mod collapsible;
pub mod form;
pub mod menu;
pub mod select;
pub mod toast;

pub use accordion::{Accordion, AccordionProps, AccordionState};
pub use alert_dialog::{AlertDialog, AlertDialogProps, AlertDialogState};
pub use checkbox::{Checkbox, CheckboxProps, CheckboxState};
pub use dialog::{Dialog, DialogProps, DialogState};
pub use disclosure::{Disclosure, DisclosureProps, DisclosureState};
pub use focus_scope::{FocusScope, FocusScopeProps, FocusScopeState};
pub use popover::{Popover, PopoverProps, PopoverState};
pub use portal::{Portal, PortalProps, PortalState};
pub use pressable::{Pressable, PressableProps, PressableState};
pub use progress::{Progress, ProgressProps, ProgressState};
pub use radio::{RadioGroup, RadioGroupProps, RadioGroupState};
pub use separator::{Separator, SeparatorProps, SeparatorState};
pub use switch::{Switch, SwitchProps, SwitchState};
pub use tabs::{Tabs, TabsProps, TabsState};
pub use text_area::{TextArea, TextAreaProps, TextAreaState};
pub use text_input::{TextInput, TextInputProps, TextInputState, TextInputType};
pub use toggle::{Toggle, ToggleProps, ToggleState};
pub use tooltip::{Tooltip, TooltipProps, TooltipState};
pub use collapsible::{Collapsible, CollapsibleProps, CollapsibleState};
pub use form::{Form, FormProps, FormState, FormField, FormFieldProps, FormFieldState};
pub use menu::{Menu, MenuProps, MenuState, MenuItemData};
pub use select::{Select, SelectProps, SelectState, SelectOption};
pub use toast::{Toast, ToastProps, ToastState, ToastVariant};
pub use visually_hidden::{VisuallyHidden, VisuallyHiddenProps, VisuallyHiddenState};
