use dioxus::prelude::*;

use super::icons;

pub fn banner() -> Element {
    rsx!(
        section {
            p {
                class: "text-lg text-center text-gray-600 pt-0",
                 "Keep your secret safe",
                i { class: "inline-block mx-1 pt-1 w-6 h-6",
                    icons::padlock {}
                }
            }
        }
    )
}
