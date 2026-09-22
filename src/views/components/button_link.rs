use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::colors::Color;

#[component]
pub fn ButtonLink(
    link: &'static str,
    label: &'static str,
    btn_color: Color,
    shadow_color: Color,
) -> impl IntoView {
    view! {
        <A href=link
        attr:class=format!("border border-line rounded-full py-2 px-4 {}", shadow_color.shadow3())
        >{label}
        </A>
    }
}
