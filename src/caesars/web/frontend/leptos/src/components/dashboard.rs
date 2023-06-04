use leptos::*;

use crate::rot;

#[component]
pub fn Dashboard(cx: Scope) -> impl IntoView {
    let (plain, set_plain) = create_signal(cx, "".to_string());
    let (secret, set_secret) = create_signal(cx, "".to_string());

    let encrypted = create_resource(cx, plain, |value| async move { rot::encrypt(value).await });
    let encrypted = move || encrypted.read(cx).unwrap_or_else(|| "Loading...".into());
    let decrypted = create_resource(cx, secret, |value| async move { rot::decrypt(value).await });
    let decrypted = move || decrypted.read(cx).unwrap_or_else(|| "Loading...".into());

    view! { cx,
        <section class="flex flex-col mt-10">
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Plain"</label>
                <textarea
                    class="input"
                    placeholder="me@caesar.tld"
                    prop:value=decrypted
                    on:input=move |ev| {
                        set_plain(event_target_value(&ev));
                    }
                ></textarea>
            </div>
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Secret"</label>
                <textarea
                    class="input"
                    placeholder="zr@pnrfne.gyq"
                    prop:value=encrypted
                    on:input=move |ev| {
                        set_secret(event_target_value(&ev));
                    }
                ></textarea>
            </div>
            <div class="flex justify-center"></div>
        </section>
    }
}
