mod components {
    pub mod banner;
    pub mod dashboard;
    pub mod footer;
    pub mod icons;
}
pub mod rot;

use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        section { class: "bg-main pt-12 md:pt-20 pb-6 px-2 md:px-5 min-h-screen",
            header { class: "max-w-lg mx-auto",
                h1 { class: "font-bold text-white text-center",
                    "Caesar",
                }
            }
            main { class: "bg-white max-w-5xl mx-auto p-8 md:p-12 my-10 rounded-lg shadow-2xl",
            components::banner::banner()
            components::dashboard::dashboard()
            }
            components::footer::footer()
        }
    ))
}
