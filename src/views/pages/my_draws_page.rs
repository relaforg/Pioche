use leptos::prelude::*;
use leptos_router::components::{Redirect, A};

use crate::{
    server::{
        draws::{get_user_draw, Draw, DrawKind},
        session::require_user,
    },
    views::colors::Color,
};

#[component]
pub fn MyDrawsPage() -> impl IntoView {
    let Ok(user) = require_user() else {
        return view! { <Redirect path="/connexion" /> }.into_any();
    };

    let draws = Resource::new(|| (), |_| get_user_draw());

    view! {
        <div class="flex justify-between items-center">
            <h3 class="text-4xl font-bold text-shadow-butter-3 mb-5">"Mes tirages"</h3>
        </div>
        <div class="grid grid-cols-3 gap-5">
            <Suspense fallback=|| {
                "Chargement..."
            }>
                {move || Suspend::new(async move {
                    match draws.await {
                        Ok(draws) => {
                            view! {
                                {draws
                                    .into_iter()
                                    .map(|d| view! { <DrawView draw=d /> })
                                    .collect_view()}
                            }
                                .into_any()
                        }
                        Err(_) => view! { <p>"Error while fetching data"</p> }.into_any(),
                    }
                })}
            </Suspense>
            <A href="/">
                <div class="h-50 w-full border border-dashed rounded-blob bg-bg grid place-items-center hover:bg-surface cursor-pointer">
                    <div class="flex justify-center flex-col">
                        <span class="grid place-items-center font-display mx-auto font-semibold text-3xl bg-butter-500 border rounded-full size-15 my-5">
                            "+"
                        </span>
                        <p class="font-display font-semibold text-lg">"Créer un tirage"</p>
                    </div>
                </div>
            </A>
        </div>
    }
    .into_any()
}

#[component]
fn DrawView(draw: Draw) -> impl IntoView {
    let (color, label) = match draw.kind {
        DrawKind::SecretSanta => (Color::Raspberry, "Secret Santa"),
        DrawKind::Teams => (Color::Blue, "Équipes"),
    };

    view! {
        <A href="">
            <div class=format!(
                "h-50 w-full border border-line rounded-blob cursor-pointer bg-surface {} overflow-hidden flex flex-col",
                color.btn_press(),
            )>
                <div
                    class=format!(
                        "flex justify-between px-5 py-3 border-b border-line {}",
                        color.bg(),
                    )
                    class=("text-surface", move || matches!(color, Color::Blue))
                >
                    <h4>{label}</h4>
                    <h4>{draw.created_at.format_localized("%d %b", chrono::Locale::fr_FR).to_string()}</h4>
                </div>
                <div class="px-5 py-3 flex flex-col gap-2 flex-1">
                    <h3 class="text-2xl">{draw.name}</h3>
                    {match draw.drawn_at {
                        Some(_) => {
                            view! {
                                <span class="px-3 py-1 text-sm bg-blue-200 border rounded-full font-extrabold w-fit">
                                    "Tirage éffectué"
                                </span>
                            }
                                .into_any()
                        }
                        None => {
                            view! {
                                <span class="px-3 py-1 text-sm bg-butter-200 border rounded-full font-extrabold w-fit">
                                    "Brouillon"
                                </span>
                            }
                                .into_any()
                        }
                    }}
                </div>
                <div class="flex justify-between px-5 py-3 font-semibold">
                    <p>{draw.participants.len()} " participants"</p>
                    <p>"Ouvrir →"</p>
                </div>
            </div>
        </A>
    }
}
