use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    server::session::current_user,
    views::{colors::Color, components::button_link::ButtonLink},
};

#[component]
pub fn Header() -> impl IntoView {
    let user = current_user();
    view! {
        <div class="flex justify-between mt-5 mb-15">
            <A href="/" attr:class="flex gap-3 items-center">
                <Logo />
                <h2 class="text-3xl text-shadow-blue-3">"Pioche !"</h2>
            </A>
            {match user {
                Some(user) => view! { <p>"Bonjour "{user.name}</p> }.into_any(),
                None => {
                    view! {
                        <div class="flex gap-3 items-center">
                            <ButtonLink
                                link="/connexion"
                                label="Se connecter"
                                bg_color=Color::Surface
                                shadow_color=Color::Blue
                            />
                        </div>
                    }
                        .into_any()
                }
            }}
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
