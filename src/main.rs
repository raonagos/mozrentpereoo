#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use mozrentpereoo::app::*;
    use mozrentpereoo::cli::*;

    let AppCli { mode } = AppCli::parse();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let app = match mode {
        Modes::Private => {
            log!("Run as private (manager)");
            let routes = generate_route_list(AdminApp);
            Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || shell_admin_app(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell_admin_app))
                .with_state(leptos_options)
        }
        Modes::Public => {
            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);
            Router::new()
                .leptos_routes(&leptos_options, routes, {
                    let leptos_options = leptos_options.clone();
                    move || shell_app(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler(shell_app))
                .with_state(leptos_options)
        }
    };

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "server"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
