use crate::constants::wards;
use yew::{function_component, html, Callback, Html};
use yew_octicons::{Icon, IconKind};

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html! {
      <aside
        class="transform top-0 left-0 w-64 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-white border-gray-200 dark:border-gray-400 fixed h-full overflow-auto ease-in-out transition-all duration-300 px-6 py-4 border-r z-30 { if isOpenSidebar ? 'translate-x-0' : '-translate-x-full'}"
      >
        <button
          type="button"
          class="text-3xl focus:outline-none"
          onclick={Callback::from(|_| (panic!("test")))}
        >
          {Icon::new(IconKind::X)}
        </button>
        <div>
          <h2 class="mt-3 text-2xl font-bold">{ "Settings" }</h2>
          <h3 class="mt-3 text-lg font-bold">{ "Exclude:" }</h3>
          {wards::WARDS.into_iter().map(|ward| {
            html! {
              <div>
                <label
                  htmlFor={ward.ward}
                  class="inline-flex items-center text-gray-800 dark:text-gray-200"
                >
                  <input
                    type="checkbox"
                    class="mr-1 h-4 w-4"
                    id={ward.ward}
                    value={ward.id}
                    // onChange={handleCheckboxChange}
                  />
                  {ward.ward}
                </label>
              </div>}
          }).collect::<Html>() }
          <h3 class="mt-3 text-lg font-bold">{"Dark Mode"}</h3>
          <p class="text-gray-800 dark:text-gray-200">
            {"Can be enabled from your system"}
          </p>
          <p class="text-gray-800 dark:text-gray-200">
            {"(e.g., From Control Center on the Mac)"}
          </p>
          <h3 class="mt-3 text-lg font-bold">{"About"}</h3>
          <p class="text-xs text-gray-600 dark:text-gray-400">
            {"Created by "}
            <a
              class="text-green-600 hover:text-green-500 dark:text-green-300 dark:hover:text-green-200"
              href="https://stevenfukase.com"
            >
              { "STEVEN FUKASE" }
            </a>
            <br />
            { "with the Rust and Yew" }
          </p>
        </div>
      </aside>
    }
}
