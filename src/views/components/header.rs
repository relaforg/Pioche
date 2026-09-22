use leptos::prelude::*;
use leptos_router::components::A;

use crate::{server::colors::Color, views::components::button_link::ButtonLink};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="flex justify-between my-5">
            <A href="/" attr:class="flex gap-3 items-center">
                <Logo />
                <h2 class="text-3xl text-shadow-blue-3">"Pioche !"</h2>
            </A>
            <div class="flex gap-3 items-center">
                <ButtonLink link="/" label="Comment ça marche ?" btn_color=Color::Butter shadow_color=Color::Butter/>
                // <ButtonLink link="/" label="Se connecter" />
            </div>
        </div>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="size-13 bg-butter-500 rounded-logo border border-line shadow-ink-4 grid place-items-center">
            <div class="size-5 bg-blue-500 rounded-full border border-line"></div>
        </div>
    }
}
