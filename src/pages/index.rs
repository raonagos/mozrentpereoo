use crate::calendar::CalendarPage;

use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn IndexPage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Bienvenue sur Mo'orea Rent Pere'o'o !"</h1>
        <button on:click=on_click>"Compteur : " {count}</button>
        <CalendarPage/>
    }
}
