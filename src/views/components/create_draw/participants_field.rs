use leptos::prelude::*;

use crate::server::participants::{parse_name, MAX_PARTICIPANTS};

#[component]
pub fn ParticipantsField(
    participants: RwSignal<Vec<String>>,
    dialog: NodeRef<leptos::html::Dialog>,
) -> impl IntoView {
    let draft = RwSignal::new(String::new());
    let local_error = RwSignal::new(None::<String>);

    let count = move || participants.with(Vec::len);

    let add_participant = move || local_error.set(try_add_participant(participants, draft).err());

    view! {
        <div class="flex items-baseline gap-2">
            <label for="participant-draft">"Participants"</label>
            <span class="font-bold text-fg-soft">{count} " dans la liste"</span>
        </div>
        <ParticipantList participants />
        <div class="flex gap-3">
            <input
                id="participant-draft"
                type="text"
                placeholder="Prénom puis Entrée"
                class="flex-1"
                bind:value=draft
                on:input=move |_| local_error.set(None)
                on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                    if ev.key() == "Enter" {
                        ev.prevent_default();
                        add_participant();
                    }
                }
            />
            <button
                type="button"
                class="cursor-pointer border rounded-full px-5 bg-raspberry-500 btn-press-ink"
                on:click=move |_| add_participant()
            >
                "Ajouter"
            </button>
            <button
                type="button"
                class="cursor-pointer border rounded-full px-5 bg-surface btn-press-blue-500"
                on:click=move |_| {
                    if let Some(d) = dialog.get() {
                        let _ = d.show_modal();
                    }
                }
            >
                "Coller une liste"
            </button>
        </div>
        {move || {
            local_error.get().map(|e| view! { <p class="mt-2 text-raspberry-500">{e}</p> })
        }}
    }
}

/// Valide `draft`, l'ajoute à `participants` puis vide le brouillon.
pub fn try_add_participant(
    participants: RwSignal<Vec<String>>,
    draft: RwSignal<String>,
) -> Result<(), String> {
    let name = draft.with(|d| parse_name(d)).map_err(|e| e.to_string())?;
    let key = name.to_lowercase();

    if participants.with(|list| list.iter().any(|p| p.to_lowercase() == key)) {
        return Err(format!("'{name}' est déjà dans la liste"));
    }
    if participants.with(Vec::len) >= MAX_PARTICIPANTS {
        return Err(format!("{MAX_PARTICIPANTS} participants maximum"));
    }
    participants.update(|list| list.push(name));
    draft.set(String::new());
    Ok(())
}

#[component]
pub fn ParticipantList(participants: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <ul class="flex flex-wrap content-start gap-2 min-h-32 mt-2 mb-3 p-3 bg-neutral-200 border rounded-blob">
            {move || {
                participants
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, name)| {
                        view! {
                            <li class="flex items-center gap-2 h-fit px-3 py-1 bg-surface border rounded-full font-bold">
                                {name.clone()}
                                <input
                                    type="hidden"
                                    name=format!("participants[{i}]")
                                    value=name.clone()
                                />
                                <button
                                    type="button"
                                    class="cursor-pointer text-raspberry-500"
                                    aria-label=format!("Retirer {name}")
                                    on:click=move |_| {
                                        participants
                                            .update(|list| {
                                                list.remove(i);
                                            })
                                    }
                                >
                                    "x"
                                </button>
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
