//! Styled Toast and Toaster container components.

use crate::common::merge_classes;
use stratum_core::aria::{AriaAttributes, AriaLive, AriaRole};
use stratum_core::render::{AttrValue, RenderOutput};

/// Toast variant determines the visual style.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
}

/// Properties for a single Toast notification.
#[derive(Debug, Clone, PartialEq)]
pub struct ToastProps {
    pub variant: ToastVariant,
    pub open: bool,
    pub class: Option<String>,
    pub aria_label: Option<String>,
    pub id: Option<String>,
}

impl Default for ToastProps {
    fn default() -> Self {
        Self {
            variant: ToastVariant::default(),
            open: true,
            class: None,
            aria_label: None,
            id: None,
        }
    }
}

pub struct Toast;

impl Toast {
    const BASE: &'static str = "group pointer-events-auto relative flex w-full items-center justify-between space-x-2 overflow-hidden rounded-md border p-4 pr-6 shadow-lg transition-all data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-80 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-right-full data-[state=open]:slide-in-from-top-full";

    pub fn classes(props: &ToastProps) -> String {
        let variant_cls = match props.variant {
            ToastVariant::Default => "border bg-background text-foreground",
            ToastVariant::Success => {
                "border-green-500 bg-green-50 text-green-900 dark:bg-green-950 dark:text-green-100"
            }
            ToastVariant::Error => "border-destructive bg-destructive text-destructive-foreground",
            ToastVariant::Warning => {
                "border-yellow-500 bg-yellow-50 text-yellow-900 dark:bg-yellow-950 dark:text-yellow-100"
            }
            ToastVariant::Info => {
                "border-blue-500 bg-blue-50 text-blue-900 dark:bg-blue-950 dark:text-blue-100"
            }
        };

        let computed = format!("{} {}", Self::BASE, variant_cls);
        merge_classes(&computed, &props.class)
    }

    pub fn render(props: &ToastProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new().with_role(AriaRole::Alert);
        aria.live = Some(AriaLive::Assertive);
        aria.atomic = Some(true);

        if let Some(ref label) = props.aria_label {
            aria = aria.with_label(label.clone());
        }

        let mut output = RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
            .with_data("state", if props.open { "open" } else { "closed" });

        if let Some(ref id) = props.id {
            output = output.with_attr("id", AttrValue::String(id.clone()));
        }

        output
    }
}

/// Properties for the Toaster container.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToasterProps {
    pub class: Option<String>,
}

pub struct Toaster;

impl Toaster {
    const BASE: &'static str = "fixed top-0 z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:bottom-0 sm:right-0 sm:top-auto sm:flex-col md:max-w-[420px]";

    pub fn classes(props: &ToasterProps) -> String {
        merge_classes(Self::BASE, &props.class)
    }

    pub fn render(props: &ToasterProps) -> RenderOutput {
        let classes = Self::classes(props);
        let mut aria = AriaAttributes::new();
        aria.live = Some(AriaLive::Polite);

        RenderOutput::new()
            .with_tag("div")
            .with_class(classes)
            .with_aria(aria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_default_classes() {
        let props = ToastProps::default();
        let classes = Toast::classes(&props);
        assert!(classes.contains("rounded-md"));
        assert!(classes.contains("bg-background"));
    }

    #[test]
    fn toast_error_variant() {
        let props = ToastProps {
            variant: ToastVariant::Error,
            ..Default::default()
        };
        let classes = Toast::classes(&props);
        assert!(classes.contains("bg-destructive"));
    }

    #[test]
    fn toast_render_role() {
        let props = ToastProps::default();
        let output = Toast::render(&props);
        assert_eq!(output.aria.role, Some(AriaRole::Alert));
        assert_eq!(output.aria.live, Some(AriaLive::Assertive));
    }

    #[test]
    fn toaster_render() {
        let props = ToasterProps::default();
        let output = Toaster::render(&props);
        assert!(output.class_string().contains("fixed"));
        assert_eq!(output.aria.live, Some(AriaLive::Polite));
    }
}
