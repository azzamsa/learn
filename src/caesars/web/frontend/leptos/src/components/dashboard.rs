use leptos::*;

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
                    prop:value=secret
                    on:input=move |ev| {
                        set_plain(event_target_value(&ev));
                    }
                ></textarea>
            </div>
            <div class="pt-3 mb-6 bg-gray-200 rounded">
                <label class="input-label">"Secret"</label>
                <textarea
                    class="input"
                    placeholder="me@caesar.tld"
                    prop:value=plain
                    on:input=move |ev| {
                        set_secret(event_target_value(&ev));
                    }
                ></textarea>
            </div>
            <div class="flex justify-center"></div>
        </section>
    }
}
