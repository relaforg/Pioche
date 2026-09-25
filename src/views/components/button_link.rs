use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::colors::Color;

#[component]
pub fn ButtonLink(
    link: &'static str,
    label: &'static str,
    #[prop(optional)] bg_color: Color,
    #[prop(optional)] shadow_color: Color,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    view! {
        <A href=link>
            <button class=format!(
                "cursor-pointer border border-line rounded-full py-2 px-4 {} {} {}",
                class,
                bg_color.bg(),
                shadow_color.btn_press(),
            )>{label}</button>
        </A>
    }
}
