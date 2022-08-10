use crate::{
    constants::wards,
    contexts::app_context::{AppState, AppStateAction},
};
use yew::{function_component, html, use_context, Callback, UseReducerHandle};

#[function_component(IndexPage)]
pub fn index_page() -> Html {
    let context = use_context::<UseReducerHandle<AppState>>().expect("no ctx found");
    let current_ward = context.current_ward as usize;
    let on_click_generate = {
        Callback::from(move |_| context.dispatch(AppStateAction::Generate))
    };

    html! {
        <div class="h-screen bg-gray-100 dark:bg-gray-900">
            <div class="pt-14 h-full grid grid-rows-2 sm:grid-cols-2 sm:grid-rows-1">
                <div>
                    <iframe
                        title="map"
                        class="h-full w-full filter dark:invert"
                        src={format!("https://www.google.com/maps/embed/v1/place?key=AIzaSyDjd3XyCKvPTWNeIKtEWJpUCDW874-XBvM&q={}", wards::WARDS[current_ward].ward)}
                        allowFullScreen="true"
                    />
                </div>
                <div class="place-self-center flex flex-col items-center transform sm:-translate-y-12 sm:order-first">
                    <h1 class="dark:text-white">{"Welcome to Tokyo Ward Generator"}</h1>
                    <h1 class="text-6xl dark:text-white">{wards::WARDS[current_ward].ward}</h1>
                    <h2 class="text-4xl dark:text-white mt-3">{wards::WARDS[current_ward].japanese}</h2>
                    <button
                        type="button"
                        onclick={on_click_generate}
                        class="focus:outline-none text-gray-500 dark:text-gray-400 hover:text-green-500 dark:hover:text-green-200"
                    >
                        {"Generate Place"}
                    </button>
                </div>
            </div>
        </div>
    }
}
