use leptos::*;
use leptos_meta::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="Pokedex - About" />
        <p class="mt-4 text-center">"Browse your favourite pokemon 🐉"</p>
    }
}
