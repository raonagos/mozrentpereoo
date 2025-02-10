mod api;
pub mod app;
mod calendar;
mod cli;
mod pages;
mod services;

// exposes needed functions to binary
pub mod shared {
    pub use crate::cli::*;
    pub use crate::services::*;
}

pub type AppError = String;
pub type AppResult<T> = std::result::Result<T, AppError>;

#[cfg(feature = "web")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use crate::cli::*;

    console_error_panic_hook::set_once();
    let AppCli { mode } = AppCli::parse();

    match mode {
        Modes::Private => leptos::mount::hydrate_body(AdminApp),
        Modes::Public => leptos::mount::hydrate_body(App),
    };
}
