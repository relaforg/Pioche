use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-15 flex justify-between items-center bg-ink text-bg p-7 gap-5">
            <h5 class="text-nowrap font-semibold font-display text-xl">"Pioche !"</h5>
            <p class="font-bold">
                "Tirage équitable, résultats vérifiables. Fait avec beaucoup de hasard."
            </p>
        </footer>
    }
}
