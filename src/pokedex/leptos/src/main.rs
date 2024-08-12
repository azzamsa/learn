use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <p class="text-red-900">"Hello, world!"</p>
            <button class="btn btn-primary">Button</button>
        }
    })
}
