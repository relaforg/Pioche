use leptos::prelude::*;

#[component]
pub fn HowItWorksPage() -> impl IntoView {
    view! {
        <div class="max-w-100">
            <h2 class="text-5xl text-shadow-butter-3 mb-5">"Comment ça marche"</h2>
            <p class="text-lg font-bold">
                "Deux rôles, deux parcours. L'organisateur a un compte, les participants n'ont qu'un lien."
            </p>
        </div>
    }
}
