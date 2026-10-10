use leptos::prelude::*;

#[component]
pub fn SecretSantaOption() -> impl IntoView {
    view! {
        <div class="flex gap-5">
            <div class="flex-1">
                <label for="budget">"Budget Cadeau"</label>
                <input class="mt-2" id="budget" name="budget" type="text" required />
            </div>
            <div class="flex-1">
                <label for="date">"Date de l'échange"</label>
                <input class="mt-2" id="date" name="date" type="date" required />
            </div>
        </div>
    }
}
