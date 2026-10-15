use leptos::prelude::*;
use leptos::Params;
use leptos_router::{components::Redirect, hooks::use_params, params::Params};

use crate::server::draws::get_draw_by_id;
use crate::server::draws::DrawDetailDto;
use crate::server::error::AppError;
use crate::server::session::require_user;

#[derive(Params, PartialEq, Clone)]
struct DrawParams {
    id: Option<i32>,
}

#[component]
pub fn DrawViewPage() -> impl IntoView {
    let params = use_params::<DrawParams>();
    let draw = Resource::new(
        move || params.read().as_ref().ok().and_then(|p| p.id),
        |id| async move {
            let id = id.ok_or_else(|| AppError::Invalid("Id tirage invalide".into()))?;
            get_draw_by_id(id).await
        },
    );
    view! {
        <Suspense fallback=|| {
            "Chargement..."
        }>
            {move || Suspend::new(async move {
                match draw.await {
                    Ok(draw) => view! { <DrawView draw /> }.into_any(),
                    Err(_) => view! { <Redirect path="/mes-tirages" /> }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn DrawView(draw: DrawDetailDto) -> impl IntoView {
    let Ok(user) = require_user() else {
        return view! { <Redirect path="/mes-tirages" /> }.into_any();
    };
    if user.id != draw.owner_id {
        return view! { <Redirect path="/mes-tirages" /> }.into_any();
    }
    view! { <p>{draw.name}</p> }.into_any()
}
