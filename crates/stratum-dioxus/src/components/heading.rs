//! Heading component for Dioxus.

use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[component]
pub fn Heading(
    #[props(default = HeadingLevel::H2)] level: HeadingLevel,
    #[props(default = String::new())] class: String,
    children: Element,
) -> Element {
    let size_cls = match level {
        HeadingLevel::H1 => "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        HeadingLevel::H2 => "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        HeadingLevel::H3 => "scroll-m-20 text-2xl font-semibold tracking-tight",
        HeadingLevel::H4 => "scroll-m-20 text-xl font-semibold tracking-tight",
        HeadingLevel::H5 => "scroll-m-20 text-lg font-semibold tracking-tight",
        HeadingLevel::H6 => "scroll-m-20 text-base font-semibold tracking-tight",
    };
    let classes = format!("{size_cls} {class}");

    match level {
        HeadingLevel::H1 => rsx! { h1 { class: "{classes}", {children} } },
        HeadingLevel::H2 => rsx! { h2 { class: "{classes}", {children} } },
        HeadingLevel::H3 => rsx! { h3 { class: "{classes}", {children} } },
        HeadingLevel::H4 => rsx! { h4 { class: "{classes}", {children} } },
        HeadingLevel::H5 => rsx! { h5 { class: "{classes}", {children} } },
        HeadingLevel::H6 => rsx! { h6 { class: "{classes}", {children} } },
    }
}
