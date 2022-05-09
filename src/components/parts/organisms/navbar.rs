use crate::constants::site_name;
use yew::{classes, function_component, html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let nav_classes = String::from("flex fixed w-full items-center justify-between px-6 h-14 bg-gray-200 dark:bg-gray-800 text-gray-700 dark:text-white border-b border-gray-300 dark:border-gray-400");
    html! {
        <nav class={classes!(nav_classes)}>
            <button type="button" class="text-2xl focus:outline-none">
            // <FontAwesomeIcon icon={faBars} />
            </button>
            <div class="hidden sm:block sm:justify-between sm:bg-transparent">
                <p class="text-gray-500 dark:text-gray-400">{ site_name::SITE_NAME }</p>
            </div>
        </nav>
    }
}
