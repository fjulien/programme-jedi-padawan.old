use leptos::*;

#[component]
pub fn home_picture() -> impl IntoView {
  view!{
    <section class="bg-cover bg-center h-[calc(100vh-74px)]"
        style="background-image: url(\"https://raw.githubusercontent.com/fjulien/programme-jedi-padawan/main/public/images/entrainement-padawan-2.jpg\")">
    </section>
  }
}