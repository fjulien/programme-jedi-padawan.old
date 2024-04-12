use crate::components::chronologie::Chronologie;
use crate::components::objectifs::Objectifs;
use crate::components::home_picture::HomePicture;
use leptos::*;

/// Default Home Page
#[component]
pub fn home() -> impl IntoView {
    view! {
        <HomePicture/>
        <Chronologie/>
        <Objectifs/>
    }
}
