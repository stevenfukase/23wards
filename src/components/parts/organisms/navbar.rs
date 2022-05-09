use crate::constants::site_name;
use yew::{function_component, html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class="backdrop-blur-sm sticky top-0 py-4 px-4 flex justify-between">
            <a href="/">
                <h1>{ site_name::SITE_NAME }</h1>
            </a>
            <nav class="hidden sm:flex gap-3">
                <a href="/">{ "Home" }</a>
                <a href="/profile">{ "Profile" }</a>
            </nav>
        </header>
    }
}
