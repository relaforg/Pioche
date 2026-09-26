use leptos::prelude::*;

use crate::{
    server::colors::Color,
    views::components::{button_link::ButtonLink, card_link::CardLink, pill::Pill},
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <LandingSection />
        <h3 class="text-3xl font-bold mt-10 mb-5 text-shadow-butter-3">
            "Deux tirages, zéro prise de tête"
        </h3>
        <div class="flex gap-5">
            <CardLink class="flex-1" link="/" bg_color=Color::Bg shadow_color=Color::Blue>
                <div class="flex gap-2">
                    <div class="size-10 rounded-chip bg-butter-500 border"></div>
                    <div class="size-10 rounded-chip bg-blue-500 border"></div>
                    <div class="size-10 rounded-chip bg-raspberry-500 border"></div>
                </div>
                <h4 class="text-2xl my-3">"Former des équipes"</h4>
                <p class="font-semibold my-3 font-sans">
                    "Choisis le nombre d'équipes ou la taille des groupes. On répartit, on nomme, on colorie."
                </p>
                <p class="font-display font-semibold">"C'est parti →"</p>
            </CardLink>
            <CardLink class="flex-1" link="/" bg_color=Color::Bg shadow_color=Color::Raspberry>
                <div class="flex gap-2">
                    <div class="size-10 rounded-full bg-butter-500 border"></div>
                    <div class="size-10 rounded-full bg-blue-500 border"></div>
                    <div class="size-10 rounded-full bg-raspberry-500 border"></div>
                </div>
                <h4 class="text-2xl my-3">"Secret Santa"</h4>
                <p class="font-semibold my-3 font-sans">
                    "Chacun reçoit son binôme en secret. Personne ne se tire soi-même, promis juré."
                </p>
                <p class="font-display font-semibold">"C'est parti →"</p>
            </CardLink>
        </div>
    }
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
