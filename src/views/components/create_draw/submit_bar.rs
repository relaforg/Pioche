use leptos::prelude::*;

use crate::server::draws::AddDraw;

#[component]
pub fn SubmitBar(add_draw: ServerAction<AddDraw>, missing: Signal<usize>) -> impl IntoView {
    let server_error = move || add_draw.value().get().and_then(Result::err);

    view! {
        <div class="flex flex-col items-center gap-2 my-7">
            <input
                type="submit"
                value="Créer le tirage"
                class="cursor-pointer border rounded-full py-3 bg-raspberry-500 btn-press-ink font-display font-semibold w-full disabled:opacity-50 disabled:cursor-not-allowed"
                disabled=move || { missing.get() > 0 || add_draw.pending().get() }
            />
            <Show when=move || { missing.get() > 0 }>
                <p class="text-fg-soft">"Encore " {missing} " participant(s) minimum"</p>
            </Show>
        </div>
        {move || {
            server_error()
                .map(|e| view! { <p class="text-center text-red-600">{e.to_string()}</p> })
        }}
    }
}
