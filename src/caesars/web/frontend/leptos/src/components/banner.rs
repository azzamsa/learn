use leptos::*;

use crate::components::icons::Padlock;

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <section>
            <p class="pt-0 text-lg text-center text-gray-600">
                "Keep your secret safe" <i class="inline-block pt-1 mx-1 w-6 h-6">
                    <Padlock/>
                </i>
            </p>
        </section>
    }
}
