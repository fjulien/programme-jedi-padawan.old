use leptos::*;
use crate::components::chronologie::Chronologie;
use crate::components::objectifs::Objectifs;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <img class="h-auto max-w-full" src="https://raw.githubusercontent.com/fjulien/programme-jedi-padawan/main/public/images/entrainement-padawan-2.jpg" alt="entrainement padawan"/>
        <Chronologie/>
        <Objectifs/>
    }
}
