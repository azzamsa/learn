use leptos::*;

use crate::rot;

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    let (plain, set_plain) = create_signal(cx, "".to_string());
    let (secret, set_secret) = create_signal(cx, "".to_string());

    view! { cx,
        <section class="flex flex-col mt-10">
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Plain"</label>
                <textarea
                    class="input"
                    placeholder="me@caesar.tld"
                    prop:value=plain
                    on:input=move |ev| {
                        let plain = event_target_value(&ev);
                        set_plain(plain.clone());
                        spawn_local(async move {
                            let secret = rot::encrypt(plain).await;
                            set_secret(secret);
                        });
                    }
                ></textarea>
            </div>
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Secret"</label>
                <textarea
                    class="input"
                    placeholder="zr@pnrfne.gyq"
                    prop:value=secret
                    on:input=move |ev| {
                        let secret = event_target_value(&ev);
                        set_secret(secret.clone());
                        spawn_local(async move {
                            let plain = rot::decrypt(secret).await;
                            set_plain(plain);
                        });
                    }
                ></textarea>
            </div>
            <div class="flex justify-center"></div>
        </section>
    }
}
