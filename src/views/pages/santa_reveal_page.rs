use leptos::prelude::*;
use leptos::Params;
use leptos_router::{hooks::use_params, params::Params};

use crate::server::error::AppError;
use crate::server::santa_reveal::{get_santa_reveal, SantaRevealDto};
use crate::server::secret_santa_option::{format_day, format_euros};
use crate::views::colors::Color;
use crate::views::components::{card::Card, draw_view::header::DrawHeader};

#[derive(Params, PartialEq, Clone)]
struct RevealParams {
    token: Option<String>,
}

#[component]
pub fn SantaRevealPage() -> impl IntoView {
    let params = use_params::<RevealParams>();
    let reveal = Resource::new(
        move || params.read().as_ref().ok().and_then(|p| p.token.clone()),
        |token| async move {
            let token = token.ok_or_else(|| AppError::Invalid("Lien invalide".into()))?;
            get_santa_reveal(token).await
        },
    );
    view! {
        <Suspense fallback=|| {
            "Chargement..."
        }>
            {move || Suspend::new(async move {
                match reveal.await {
                    Ok(reveal) => view! { <SantaReveal reveal /> }.into_any(),
                    Err(e) => {
                        view! {
                            <div class="p-7 border border-dashed rounded-blob text-center font-bold text-fg-soft">
                                {e.to_string()}
                            </div>
                        }
                            .into_any()
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
fn SantaReveal(reveal: SantaRevealDto) -> impl IntoView {
    let subtitle = [
        Some("Secret Santa".to_string()),
        reveal
            .options
            .budget_cents
            .map(|c| format!("budget {}", format_euros(c))),
        reveal
            .options
            .exchange_date
            .map(|d| format!("échange le {}", format_day(d))),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" · ");

    view! {
        <div class="flex flex-col gap-8 w-full max-w-2xl mx-auto">
            <DrawHeader name=reveal.draw_name subtitle color=Color::Raspberry>
                {()}
            </DrawHeader>
            <Card bg_color=Color::Surface shadow_color=Color::Butter class="text-center">
                <p class="font-bold text-fg-soft">
                    "Salut " {reveal.participant_name} ", cette année tu offres un cadeau à…"
                </p>
                <h2 class="mt-4 text-5xl text-shadow-butter-3 break-words">
                    {reveal.receiver_name}
                </h2>
                <p class="mt-6 text-sm text-fg-soft">
                    "Chut ! Ce lien est personnel : personne d'autre ne sait qui tu as tiré."
                </p>
            </Card>
        </div>
    }
}
