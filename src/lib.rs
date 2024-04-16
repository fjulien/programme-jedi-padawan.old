use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::components::navigation::Navigation;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
            <Html lang="fr" dir="ltr" attr:data-theme="dark"/>
            <Title text="Programme Jedi Padawan"/>
            // injects metadata in the <head> of the page
            <Meta charset="UTF-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            <Router>
            <Navigation/>
                <main class="mt-[74px]">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/*" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        }
}
