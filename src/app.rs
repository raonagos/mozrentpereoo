use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::*;

pub fn shell_app(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
                <HashedStylesheet id="leptos" options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

pub fn shell_admin_app(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
                <HashedStylesheet id="leptos" options/>
            </head>
            <body>
                <AdminApp/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Mo'orea Rent Pere'o'o"/>

        <Router>
            <main>
                <ul>
                    <li>
                        <a href="/">"Accueil"</a>
                    </li>
                    <li>
                        <a href="/vehicule">"Véhicules"</a>
                    </li>
                    <li>
                        <a href="/reservation">"Réserver"</a>
                    </li>
                </ul>
                <Routes fallback=|| "La page n'existe pas.".into_view()>
                    <Route path=StaticSegment("") view=IndexPage/>
                    <Route path=StaticSegment("accueil") view=IndexPage/>
                    <Route path=StaticSegment("vehicule") view=CarsPage/>
                    <Route path=StaticSegment("reservation") view=ReservationPage/>
                    <Route path=StaticSegment("termes-et-conditions") view=TermsPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn AdminApp() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Gestionnaire | Mo'orea Rent Pere'o'o"/>

        <Router>
            <main>
                <ul>
                    <li>
                        <a href="/">"Accueil"</a>
                    </li>
                    <li>
                        <a href="/vehicule">"Véhicules"</a>
                    </li>
                    <li>
                        <a href="/reservation">"Réserver"</a>
                    </li>
                    <li>
                        <a href="/se-connecter">"Se connecter"</a>
                    </li>
                </ul>
                <Routes fallback=|| "La page n'existe pas.".into_view()>
                    <Route path=StaticSegment("") view=IndexPage/>
                    <Route path=StaticSegment("vehicule") view=CarsPage/>
                    <Route path=StaticSegment("se-connecter") view=SessionPage/>
                    <Route path=StaticSegment("reservation") view=ReservationPage/>
                    <Route path=StaticSegment("termes-et-conditions") view=TermsPage/>
                </Routes>
            </main>
        </Router>
    }
}
