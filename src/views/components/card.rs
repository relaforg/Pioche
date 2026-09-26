use leptos::prelude::*;

use crate::server::colors::Color;

#[component]
pub fn CardLink(
    children: Children,
    #[prop(optional)] bg_color: Color,
    #[prop(optional)] shadow_color: Color,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!(
            "border rounded-blob {} {} {}",
            class,
            bg_color.bg(),
            shadow_color.btn_press(),
        )>
            <div class="p-7">{children()}</div>
        </div>
    }
}
