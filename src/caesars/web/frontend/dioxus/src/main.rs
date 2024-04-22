#![allow(non_snake_case)]

mod components {
    pub mod banner;
    pub mod dashboard;
    pub mod footer;
    pub mod icons;
}
pub mod rot;

use dioxus::prelude::*;
use log::LevelFilter;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        section { class: "bg-main pt-12 md:pt-20 pb-6 px-2 md:px-5 min-h-screen",
            header { class: "max-w-lg mx-auto", h1 { class: "font-bold text-white text-center", "Caesar" } }
            main { class: "bg-white max-w-5xl mx-auto p-8 md:p-12 my-10 rounded-lg shadow-2xl",
                components::banner::Banner {}
                components::dashboard::Dashboard {}
            }
            components::footer::Footer {}
        }
    }
}
