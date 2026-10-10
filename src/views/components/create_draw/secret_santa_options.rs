use leptos::prelude::*;
use strum::IntoEnumIterator;

use crate::server::question_kind::QuestionKindDto;

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
        <fieldset class="mt-5">
            <legend class="font-display font-semibold text-lg">
                "Questions posées aux participants"
            </legend>
            <div class="flex flex-wrap gap-2 mt-2">
                {QuestionKindDto::iter()
                    .map(|q| {
                        view! {
                            <label class="cursor-pointer px-4 py-2 border rounded-full text-sm bg-surface btn-press-blue-500 has-checked:bg-butter-500 has-checked:btn-press-ink has-checked:text-ink has-focus-visible:outline-2 has-focus-visible:outline-ink">
                                <input
                                    class="sr-only"
                                    name="question_kind"
                                    type="checkbox"
                                    value=q.form_value()
                                />
                                <p>{q.label()}</p>
                            </label>
                        }
                    })
                    .collect_view()}
            </div>
            <p class="mt-2 font-semibold text-fg-soft">
                "Ces réponses ne seront visibles que par la personne qui offre le cadeau."
            </p>
        </fieldset>
    }
}
