//! # stratum-components
//!
//! Production-quality styled UI components for NexusStratum.
//!
//! Each component wraps a primitive from `stratum-primitives`, adding:
//! - Variant enums (e.g., `ButtonVariant`)
//! - Size enums (`Size`: Xs, Sm, Md, Lg, Xl)
//! - Default Tailwind classes per variant/size
//! - Icon slot support
//! - Loading state support (where applicable)
//!
//! Components produce `RenderOutput` from `stratum-core`, which framework
//! adapters translate into actual DOM elements.

pub mod common;

pub mod data_display;
pub mod forms;
pub mod layout;
pub mod navigation;
pub mod overlay;
pub mod typography;
pub mod utility;

// ── Re-exports for convenience ──────────────────────────────────────────

// Common
pub use common::Size;

// Forms
pub use forms::button::{Button, ButtonProps, ButtonVariant};
pub use forms::checkbox::{Checkbox, CheckboxGroup, CheckboxGroupProps, CheckboxProps};
pub use forms::form::{Form, FormError, FormErrorProps, FormField, FormFieldProps, FormHelperText, FormHelperTextProps, FormLabel, FormLabelProps, FormProps};
pub use forms::input::{Input, InputGroup, InputProps, InputVariant};
pub use forms::radio::{Radio, RadioGroup, RadioGroupProps, RadioProps};
pub use forms::select::{Select, SelectProps};
pub use forms::switch::{Switch, SwitchProps};
pub use forms::textarea::{Textarea, TextareaProps, ResizeMode};

// Overlay
pub use overlay::alert_dialog::{AlertDialog, AlertDialogProps};
pub use overlay::dialog::{Dialog, DialogOverlay, DialogOverlayProps, DialogProps};
pub use overlay::popover::{Popover, PopoverAlign, PopoverProps, PopoverSide};
pub use overlay::toast::{Toast, ToastProps, ToastVariant, Toaster, ToasterProps};
pub use overlay::tooltip::{Tooltip, TooltipProps, TooltipSide};

// Navigation
pub use navigation::accordion::{
    Accordion, AccordionContent, AccordionContentProps, AccordionItem, AccordionItemProps,
    AccordionProps, AccordionTrigger, AccordionTriggerProps,
};
pub use navigation::tabs::{Tab, TabList, TabListProps, TabPanel, TabPanelProps, TabProps};

// Data Display
pub use data_display::badge::{Badge, BadgeProps, BadgeVariant};
pub use data_display::card::{
    Card, CardContent, CardContentProps, CardDescription, CardDescriptionProps, CardFooter,
    CardFooterProps, CardHeader, CardHeaderProps, CardProps, CardTitle, CardTitleProps,
};
pub use data_display::skeleton::{Skeleton, SkeletonProps};
pub use data_display::spinner::{Spinner, SpinnerProps};

// Layout
pub use layout::divider::{Divider, DividerProps};
pub use layout::stack::{
    HStack, Stack, StackAlign, StackDirection, StackJustify, StackProps, StackSpacing, VStack,
};

// Typography
pub use typography::heading::{Heading, HeadingLevel, HeadingProps};
pub use typography::link::{Link, LinkProps, LinkVariant};
pub use typography::text::{FontWeight, Text, TextColor, TextProps};

// Utility
pub use utility::focus_scope::{FocusScope, FocusScopeProps};
pub use utility::portal::{Portal, PortalProps};
pub use utility::separator::{Separator, SeparatorProps};
pub use utility::visually_hidden::{VisuallyHidden, VisuallyHiddenProps};
