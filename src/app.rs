use crate::views::{
    components::{footer::Footer, header::Header},
    pages::{home_page::HomePage, how_it_works_page::HowItWorksPage},
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/pioche.css" />
        <Title text="Welcome to Leptos" />

        <div class="mx-auto max-w-shell px-4">
            <Router>
                <div class="my-10">
                    <Header />
                </div>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/comment-ca-marche") view=HowItWorksPage />
                    </Routes>
                </main>
            </Router>
        </div>
        <Footer />
    }
}
