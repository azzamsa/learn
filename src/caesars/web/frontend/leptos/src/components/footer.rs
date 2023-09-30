use leptos::*;

use crate::components::icons::Heart;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <section class="flex justify-center mx-auto max-w-lg font-medium text-white">
            <a
                class="duration-500 transform hover:underline hover:scale-125 hover:-translate-y-1"
                href="https://azzamsa.com/support/"
                target="_blank"
            >
                "Support Me"
                <i class="inline-block pt-0.5 mx-1 w-4 h-4">
                    <Heart/>
                </i>
            </a>
            <span class="mx-3">"•"</span>
            <a class="hover:underline" href="https://github.com/azzamsa/caesars" target="_blank">
                "Meta"
            </a>
        </section>
    }
}
