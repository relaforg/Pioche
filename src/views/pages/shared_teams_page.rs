use leptos::prelude::*;
use leptos::Params;
use leptos_router::{hooks::use_params, params::Params};

use crate::server::draws::{get_shared_teams, SharedTeamsDto};
use crate::server::error::AppError;
use crate::views::colors::Color;
use crate::views::components::draw_view::{header::DrawHeader, teams::TeamsResult};

#[derive(Params, PartialEq, Clone)]
struct ShareParams {
    token: Option<String>,
}

#[component]
pub fn SharedTeamsPage() -> impl IntoView {
    let params = use_params::<ShareParams>();
    let shared = Resource::new(
        move || params.read().as_ref().ok().and_then(|p| p.token.clone()),
        |token| async move {
            let token = token.ok_or_else(|| AppError::Invalid("Lien invalide".into()))?;
            get_shared_teams(token).await
        },
    );
    view! {
        <Suspense fallback=|| {
            "Chargement..."
        }>
            {move || Suspend::new(async move {
                match shared.await {
                    Ok(shared) => view! { <SharedTeams shared /> }.into_any(),
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
fn SharedTeams(shared: SharedTeamsDto) -> impl IntoView {
    let subtitle = format!(
        "{} participants · {} équipes",
        shared.participant_count,
        shared.teams.len()
    );
    view! {
        <div class="flex flex-col gap-8">
            <DrawHeader name=shared.name subtitle=subtitle.clone() color=Color::Blue>
                {()}
            </DrawHeader>
            <TeamsResult teams=shared.teams subtitle />
        </div>
    }
}
