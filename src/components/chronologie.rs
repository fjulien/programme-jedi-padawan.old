use leptos::*;
use crate::components::chronologie_item::ChronologieItem;
use crate::components::chronologie_item::ChronologieElement;
use chrono::{NaiveDate, Utc};

#[component]
pub fn chronologie() -> impl IntoView {
let items = [
  ChronologieElement {title: "String", date: NaiveDate::from_ymd(2019, 3, 17), description: "description", link: "String"}
  ];

  view!{
    <h2 id="chronologie" class="text-center">"chronologie"</h2>
      <ol class="flex flex-col items-center">
      {items.into_iter()
        .map(|item| view! { <ChronologieItem item=item/>})
        .collect::<Vec<_>>()} 
    </ol>
  }
}