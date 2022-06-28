use crate::{constants::wards, CurrentWard};
use yew::{function_component, html, use_context};

#[function_component(IndexPage)]
pub fn index_page() -> Html {
    let current_ward_id = use_context::<CurrentWard>().expect("no ctx found").id as usize;

    html! {
      <div class="h-screen bg-gray-100 dark:bg-gray-900">
      <div class="pt-14 h-full grid grid-rows-2 sm:grid-cols-2 sm:grid-rows-1">
        <div>
        </div>
        <div class="place-self-center flex flex-col items-center transform sm:-translate-y-12 sm:order-first">
          <h1 class="dark:text-white">{"Welcome to Tokyo Ward Generator"}</h1>
                <h1 class="text-6xl dark:text-white">{wards::WARDS[current_ward_id].ward}</h1>
                <h2 class="text-4xl dark:text-white mt-3">{wards::WARDS[current_ward_id].japanese}</h2>
            <button
              type="button"
              class="focus:outline-none text-gray-500 dark:text-gray-400 hover:text-green-500 dark:hover:text-green-200"
            >
              {"Generate Place"}
            </button>
        </div>
      </div>
      </div>
    }
}
