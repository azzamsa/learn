mod app;
mod rot;
mod components {
    pub mod banner;
    pub mod dashboard;
    pub mod footer;
    pub mod icons;
}

use app::*;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");
    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}
