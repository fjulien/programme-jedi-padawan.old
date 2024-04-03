use leptos::*;
use programme_jedi_padawan::App;
use crate::components::navigation::Navigation;

// Modules
pub mod components;


fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        
        view! {
            <Navigation/>
            <section style="margin-top:74px" class="container mx-auto">
            <App  />
            </section>
        }
    })
}
