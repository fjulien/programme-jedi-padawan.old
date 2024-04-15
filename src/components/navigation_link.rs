use leptos::*;
use leptos_router::*;

#[component]
pub fn navigation_link(link: String) -> impl IntoView {
  let anchor_link: String = format!("/#{}", link.clone());
  view!{
    <li class="my-3 capitalize"><A href=anchor_link>{link}</A></li>
  }
}
