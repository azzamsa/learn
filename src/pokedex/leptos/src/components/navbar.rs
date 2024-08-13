use leptos::*;

use crate::components::icons;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar rounded-box mb-2 bg-primary shadow-lg">
            <div class="md:hidden">
                <button class="btn-ghost btn-square btn">
                    <icons::Hamburger />
                </button>
            </div>

            <div class="navbar-start mx-2 px-2">
                <span class="text-lg font-bold">
                    <a href="/">Pokedex</a>
                </span>
            </div>

            <div class="navbar-center mx-2 px-2">
                <div class="space-x-2 flex hidden items-stretch md:flex">
                    <a class="nav-btn" href="/">
                        Home
                    </a>
                    <a class="nav-btn" href="/about">
                        About
                    </a>
                </div>
            </div>

            <div class="navbar-end">
                <div class="form-control text-neutral">
                    <input type="text" placeholder="bulbasur" class="input-bordered input" />
                </div>
                <button class="btn-ghost btn-square btn">
                    <icons::Search />
                </button>
            </div>
        </div>
    }
}
