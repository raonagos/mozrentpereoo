#[cfg(feature = "server")]
#[derive(Debug, Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: leptos::config::LeptosOptions,
    pub pool: std::sync::Arc<dyn mozrentpereoo::shared::database::Database>,
}

#[cfg(feature = "server")]
impl std::ops::Deref for AppState {
    type Target = dyn mozrentpereoo::shared::database::Database;
    fn deref(&self) -> &Self::Target {
        &*self.pool
    }
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use mozrentpereoo::app::*;
    use mozrentpereoo::shared::*;

    let AppCli { mode } = AppCli::parse();

    let conf = get_configuration(None).expect("couldn't get leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let app_state = AppState {
        leptos_options,
        pool: todo!(),
    };

    let app = match mode {
        Modes::Private => {
            log!("Run as private (manager)");
            let routes = generate_route_list(AdminApp);
            Router::new()
                .leptos_routes(&app_state, routes, {
                    let leptos_options = app_state.leptos_options.clone();
                    move || shell_admin_app(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler::<AppState, _>(
                    shell_admin_app,
                ))
                .with_state(app_state)
        }
        Modes::Public => {
            // Generate the list of routes in your Leptos App
            let routes = generate_route_list(App);
            Router::new()
                .leptos_routes(&app_state, routes, {
                    let leptos_options = app_state.leptos_options.clone();
                    move || shell_app(leptos_options.clone())
                })
                .fallback(leptos_axum::file_and_error_handler::<AppState, _>(
                    shell_app,
                ))
                .with_state(app_state)
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
