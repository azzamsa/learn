use leptos::*;

use crate::rot;

#[component]
pub fn Dashboard() -> impl IntoView {
    let (plain, set_plain) = create_signal("".to_string());
    let (secret, set_secret) = create_signal("".to_string());

    view! {
        <section class="flex flex-col mt-10">
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Plain"</label>
                <textarea
                    class="input"
                    placeholder="me@caesar.tld"
                    prop:value=move || plain.get()
                    on:input=move |ev| {
                        let plain_input = event_target_value(&ev);
                        log::debug!("plain text area: {}", & plain_input);
                        set_plain.set(plain_input.clone());
                        spawn_local(async move {
                            let secret_result = rot::encrypt(plain_input).await;
                            set_secret.set(secret_result);
                        });
                    }
                >
                </textarea>
            </div>
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Secret"</label>
                <textarea
                    class="input"
                    placeholder="zr@pnrfne.gyq"
                    prop:value=move || secret.get()
                    on:input=move |ev| {
                        let secret_input = event_target_value(&ev);
                        set_secret.set(secret_input.clone());
                        spawn_local(async move {
                            let plain_result = rot::decrypt(secret_input).await;
                            set_plain.set(plain_result);
                        });
                    }
                >
                </textarea>
            </div>
            <div class="flex justify-center"></div>
        </section>
    }
}
