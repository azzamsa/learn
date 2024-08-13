use leptos::*;
use leptos_router::*;

use pokedex::components::NavBar;
use pokedex::routes::{About, Index};

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <nav class="mx-auto my-5 max-w-6xl p-2">
                    <NavBar />
                </nav>
                <main>
                    <Routes>
                        <Route path="/" view=Index />
                        <Route path="/about" view=About />
                    </Routes>
                </main>
            </Router>
        }
    })
}
