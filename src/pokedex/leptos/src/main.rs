use leptos::*;

use pokedex::components::NavBar;

fn main() {
    mount_to_body(|| {
        view! {
            <div class="mx-auto my-5 max-w-6xl p-2">
                <NavBar />
            </div>
        }
    })
}
