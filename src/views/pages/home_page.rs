use leptos::prelude::*;

use crate::{
    server::colors::Color,
    views::components::{button_link::ButtonLink, pill::Pill},
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! { <LandingSection /> }
}

#[component]
fn LandingSection() -> impl IntoView {
    view! {
        <section class="flex justify-between">
            <div class="flex-1">
                <Pill
                    class="my-5"
                    label="100% hasard, 0% embrouille"
                    bg_color=Color::Butter
                    shadow_color=Color::Ink
                />
                <h1 class="text-7xl text-shadow-blue-7 my-5">
                    "Le tirage au sort qui met tout le monde d'accord"
                </h1>
                <p class="text-xl font-semibold my-5">
                    "Des équipes équilibrées en trois secondes, un Secret Santa sans spoiler. Tu colles ta liste, on mélange."
                </p>
                <p class="font-bold">
                    "Aucun compte pour participer : un lien suffit. Le compte sert seulement à retrouver et modifier tes propres tirages."
                </p>
                <div class="flex gap-5 my-5">
                    <ButtonLink
                        link=""
                        label="Former des équipes"
                        bg_color=Color::Bg
                        shadow_color=Color::Butter
                    />
                    <ButtonLink
                        link=""
                        label="Lancer un secret Santa"
                        bg_color=Color::Bg
                        shadow_color=Color::Raspberry
                    />
                </div>
            </div>
            <div class="flex-1 flex justify-end items-center">
                <div class="absolute size-90 dots-butter rounded-full">
                    <div class="relative top-20 -left-15 size-60 bg-raspberry-500 mix-blend-multiply rounded-blob"></div>
                    <div class="relative -top-44 -left-19 size-60 bg-butter-500 border rounded-blob animate-bob">
                        <div class="flex justify-center items-center h-full">
                            <div class="grid grid-cols-2 gap-4 place-items-center">
                                <div class="size-17 border rounded-full bg-raspberry-500"></div>
                                <div class="size-17 border rounded-full"></div>
                                <div class="size-17 border rounded-full bg-blue-500"></div>
                                <div class="size-17 border rounded-full bg-bg"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
