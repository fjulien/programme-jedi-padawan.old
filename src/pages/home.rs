use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
            <h1 class="text-2xl text-lime-500">"Programme Jedi Padawan"</h1>
            <img class="h-auto max-w-full" src="/images/entrainement-padawan-2.jpg" alt="entrainement padawan"/>
    }
}
