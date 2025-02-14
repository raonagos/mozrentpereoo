use std::sync::Arc;

use crate::app::*;
use crate::shared::*;
use axum::Router;
use leptos::config::LeptosOptions;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::SqlitePool;

#[derive(Debug, Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: std::sync::Arc<dyn Database>,
}

impl std::ops::Deref for AppState {
    type Target = dyn Database;
    fn deref(&self) -> &Self::Target {
        &*self.pool
    }
}

pub async fn run_server() {
    let AppCli {
        mode,
        database_path,
    } = AppCli::parse();

    let conf = get_configuration(None).expect("couldn't initiate leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let pool = SqlitePool::connect(
        database_path
            .to_str()
            .expect("couldn't get the database path"),
    )
    .await
    .expect("couldn't initiate sqlite");

    let app_state = AppState {
        leptos_options,
        pool: Arc::new(pool),
    };

    let app = match mode {
        Modes::Server => {
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
        Modes::Client => {
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
