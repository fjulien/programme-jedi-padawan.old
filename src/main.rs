extern crate console_error_panic_hook;

use leptos::*;
use programme_jedi_padawan::App;


// Modules
pub mod components;


fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
