use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn AuthPage() -> impl IntoView {
    view! {
        <p>"AuthPage"</p>
        <Outlet />
    }
}
