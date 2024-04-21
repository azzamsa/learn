use dioxus::{events::*, prelude::*};

use crate::rot;

pub fn dashboard() -> Element {
    let mut plain = use_signal(|| "".to_string());
    let mut secret = use_signal(|| "".to_string());

    let on_input_plain = move |e: FormEvent| {
        plain.set(e.value());
        spawn({
            // You will enter the ownership hell
            // without the magic of `to_owned()`
            let secret = secret.to_owned();

            async move {
                let resp = rot::encrypt(e.value()).await;
                secret.clone().set(resp);
            }
        });
    };
    let on_input_secret = move |e: FormEvent| {
        secret.set(e.value());
        spawn({
            let plain = plain.to_owned();

            async move {
                let resp = rot::decrypt(e.value()).await;
                plain.clone().set(resp);
            }
        });
    };

    rsx!(
        section { class: "flex flex-col mt-10 ",
                  div { class: "mb-6 pt-3 rounded bg-gray-200",
                        label { class: "input-label",
                                "Plain",
                        },
                        // plain textarea
                        textarea { class: "input",
                                   placeholder: "me@casar.tld",
                                   value: "{plain}",
                                   oninput: on_input_plain
                        }
                  }
                  div { class: "flex justify-center" }
                  div { class: "mb-6 pt-3 rounded bg-gray-200",
                        label { class: "input-label",
                                "Plain",
                        },
                        // secret textarea
                        textarea { class: "input",
                                   placeholder: "me@casar.tld",
                                   value: "{secret}",
                                   oninput: on_input_secret
                        }
                  }
        }
    )
}
