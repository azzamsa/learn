use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::banner::Banner;
use crate::components::dashboard::Dashboard;
use crate::components::footer::Footer;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route
                    path=""
                    view=move |cx| {
                        view! { cx, <Home/> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="px-2 pt-12 pb-6 min-h-screen md:px-5 md:pt-20 bg-main">
            <header class="mx-auto max-w-lg">
                <h1 class="font-bold text-center text-white"></h1>
            </header>
            <main class="p-8 my-10 mx-auto max-w-5xl bg-white rounded-lg shadow-2xl md:p-12">
                <Banner/>
                <Dashboard/>
            </main>
            <Footer/>
        </div>
    }
}
