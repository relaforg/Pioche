use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::colors::Color;

#[component]
pub fn ButtonLink(
    link: &'static str,
    label: &'static str,
    #[prop(optional)] btn_color: Color,
    #[prop(optional)] shadow_color: Color,
) -> impl IntoView {
    view! {
        <A href=link
        ><button
        class=format!("border border-line rounded-full py-2 px-4 {} {}", btn_color.bg(), shadow_color.btn_press())
        >{label}</button>
        </A>
    }
}
