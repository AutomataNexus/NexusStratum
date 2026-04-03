//! Table components for Leptos.

use leptos::prelude::*;

#[component]
pub fn Table(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("w-full caption-bottom text-sm {}", class);
    view! { <div class="relative w-full overflow-auto"><table class=classes>{children()}</table></div> }
}

#[component]
pub fn TableHeader(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("[&_tr]:border-b {}", class);
    view! { <thead class=classes>{children()}</thead> }
}

#[component]
pub fn TableBody(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("[&_tr:last-child]:border-0 {}", class);
    view! { <tbody class=classes>{children()}</tbody> }
}

#[component]
pub fn TableFooter(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0 {}", class);
    view! { <tfoot class=classes>{children()}</tfoot> }
}

#[component]
pub fn TableRow(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted {}", class);
    view! { <tr class=classes>{children()}</tr> }
}

#[component]
pub fn TableHead(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}", class);
    view! { <th class=classes>{children()}</th> }
}

#[component]
pub fn TableCell(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {}", class);
    view! { <td class=classes>{children()}</td> }
}

#[component]
pub fn TableCaption(#[prop(optional, default = String::new())] class: String, children: Children) -> impl IntoView {
    let classes = format!("mt-4 text-sm text-muted-foreground {}", class);
    view! { <caption class=classes>{children()}</caption> }
}
