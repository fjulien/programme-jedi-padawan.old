use leptos::*;

#[component]
pub fn navigation_link(link: String) -> impl IntoView {
  let anchor_link: String = format!("#{}", link.clone());
  view!{
    <li class="my-3 capitalize"><a href=anchor_link>{link}</a></li>
  }
}
