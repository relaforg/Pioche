use leptos::prelude::*;

use crate::server::draws::parse_names;

#[component]
pub fn PasteDialog(
    dialog: NodeRef<leptos::html::Dialog>,
    participants: RwSignal<Vec<String>>,
) -> impl IntoView {
    let text = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);

    let close = move || {
        if let Some(d) = dialog.get() {
            d.close();
        }
    };

    let try_paste = move || -> Result<(), String> {
        let merged = participants
            .with(|current| {
                text.with(|t| {
                    let pasted = t.split(['\n', ',', ';']).filter(|s| !s.trim().is_empty());
                    parse_names(current.iter().map(String::as_str).chain(pasted))
                })
            })
            .map_err(|e| e.to_string())?;
        participants.set(merged);
        text.set(String::new());
        Ok(())
    };

    let submit = move |_| match try_paste() {
        Ok(()) => close(),
        Err(e) => error.set(Some(e)),
    };
    view! {
        <dialog
            node_ref=dialog
            class="m-auto w0full max-w-110 p-7 bg-surface text-fg border rounded-blob shadow-butter-5 backdrop:bg-ink/50"
            on:close=move |_| error.set(None)
        >
            <h3 class="font-bold text-2xl mb-5">"Coller une liste"</h3>
            <label for="paste-list">"Un prénom par ligne, ou séparés par des virgules"</label>
            <textarea
                id="paste-list"
                rows="8"
                placeholder="Léa\nTom\nAmir"
                class="w-full mt-2 p-3 bg-neutral-200 border rounded-blob font-bold resize-none"
                autofocus
                bind:value=text
                on:input=move |_| error.set(None)
            />
            {move || error.get().map(|e| view! { <p class="mt-2 text-raspberry-500">{e}</p> })}
            <div class="flex justify-end gap-3 mt-5">
                <button
                    type="button"
                    class="cursor-pointer border rounded-full px-5 py-2 bg-surface btn-press-blue-500"
                    on:click=move |_| close()
                >
                    "Annuler"
                </button>
                <button
                    type="button"
                    class="cursor-pointer border rounded-full px-5 py-2 bg-surface btn-press-raspberry-500"
                    on:click=submit
                >
                    "Ajouter à la liste"
                </button>
            </div>
        </dialog>
    }
}
