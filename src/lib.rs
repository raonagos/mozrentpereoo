mod api;
pub mod app;
mod calendar;
mod interfaces;
mod pages;
mod services;

// exposes needed functions to binary
pub mod shared {
    pub use crate::interfaces::cli::*;
    #[cfg(feature = "server")]
    pub use crate::interfaces::web::*;
    pub use crate::services::*;
    pub use crate::shared::database::*;
}

pub type AppError = String;
pub type AppResult<T> = std::result::Result<T, AppError>;

#[cfg(feature = "web")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use crate::interfaces::cli::*;

    console_error_panic_hook::set_once();
    let AppCli { mode, .. } = AppCli::parse();

    match mode {
        Modes::Server => leptos::mount::hydrate_body(AdminApp),
        Modes::Client => leptos::mount::hydrate_body(App),
    };
}
