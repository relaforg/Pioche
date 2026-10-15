use leptos::prelude::*;

use crate::views::{colors::Color, components::button_link::ButtonLink};

#[component]
pub fn DrawHeader(
    name: String,
    subtitle: String,
    color: Color,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!(
                "flex flex-wrap justify-between items-center gap-5 p-7 border border-line rounded-blob shadow-ink-5 {}",
                color.bg(),
            )
            class=("text-surface", matches!(color, Color::Blue))
        >
            <div>
                <h2 class="text-4xl">{name}</h2>
                <p class="mt-1 font-bold">{subtitle}</p>
            </div>
            <div class="flex gap-3 text-fg">{children()}</div>
        </div>
    }
}

#[component]
pub fn BackButton() -> impl IntoView {
    view! {
        <ButtonLink
            link="/mes-tirages"
            label="← Mes tirages"
            bg_color=Color::Surface
            shadow_color=Color::Ink
        />
    }
}
