use leptos::*;
use leptos_image::*;

#[component]
pub fn home_picture() -> impl IntoView {
    view! {
      <section class="flex">
        <div></div>
      <img src="programme-jedi-padawan/public/images/entrainement-padawan-2.jpg"
      blur=true
      width=750
      height=500
      quality=85
            alt="Entrainement d'un jadi et d'un padawan"/>
      </section> 
    }
}
