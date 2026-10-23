#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use pioche::{app::*, server::ssr::session::session_middleware};

    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL given");
    let db = pioche::db::connect(&db_url)
        .await
        .expect("Cannot connect to the database");
    let mailer = pioche::server::ssr::mail::Mailer::from_env().expect("Invalid SMTP configuration");
    let public_url =
        pioche::server::ssr::public_url::PublicUrl::from_env().expect("Invalid PUBLIC_URL");
    let rate_limits = pioche::server::ssr::rate_limit::RateLimits::from_env();
    rate_limits.spawn_cleanup();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                use leptos::context::provide_context;

                let db = db.clone();
                move || {
                    provide_context(db.clone());
                    provide_context(mailer.clone());
                    provide_context(public_url.clone());
                    provide_context(rate_limits.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .route_layer(axum::middleware::from_fn_with_state(
            db.clone(),
            session_middleware,
        ))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // `ConnectInfo` donne l'IP du client aux limites de débit.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
