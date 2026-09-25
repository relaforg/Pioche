use leptos::prelude::*;

use crate::server::colors::Color;

#[component]
pub fn Pill(
    label: &'static str,
    #[prop(optional)] bg_color: Color,
    #[prop(optional)] shadow_color: Color,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <p class=format!(
            "w-fit font-semibold font-display border border-line rounded-full py-2 px-4 {} {} {}",
            class,
            bg_color.bg(),
            shadow_color.shadow5(),
        )>{label}</p>
    }
}
