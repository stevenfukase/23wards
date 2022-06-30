use crate::{
    constants::wards,
    contexts::app_context::{AppState, AppStateAction},
};
use web_sys::HtmlInputElement;
use yew::{
    classes, events::Event, function_component, html, use_context, Callback, Html, MouseEvent,
    Properties, TargetCast, UseReducerHandle,
};
use yew_octicons::{Icon, IconKind};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or(false)]
    pub is_open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(SideBar)]
pub fn side_bar(props: &SidebarProps) -> Html {
    let sidebar_class = classes!(
        String::from(
            "transform
            top-0
            left-0
            w-64
            bg-gray-100
            dark:bg-gray-800
            text-gray-700
            dark:text-white
            border-gray-200
            dark:border-gray-400
            fixed
            h-full
            overflow-auto
            ease-in-out
            transition-all
            duration-300
            px-6
            py-4
            border-r
            z-30",
        ),
        if props.is_open {
            "translate-x-0"
        } else {
            "-translate-x-full"
        }
    );

    let app_context = use_context::<UseReducerHandle<AppState>>().expect("no ctx found");
    let onchange_checkbox = Callback::from(|e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let id = input.value().parse().unwrap();
        match input.checked() {
            true => app_context.dispatch(AppStateAction::Disable(id)),
            false => app_context.dispatch(AppStateAction::Enable(id)),
        }
    });

    html! {
        <aside class={sidebar_class}>
            <button
                type="button"
                class="text-3xl focus:outline-none"
                onclick={&props.on_close}
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
                                    value={ward.id.to_string()}
                                    onchange={&onchange_checkbox}
                                    checked={app_context.disabled_wards.contains(&ward.id)}
                                />
                                {ward.ward}
                            </label>
                        </div>
                    }
                }).collect::<Html>()}
                <h3 class="mt-3 text-lg font-bold">{"Dark Mode"}</h3>
                <p class="text-gray-800 dark:text-gray-200">
                    {"Can be enabled from your system"}
                </p>
                <p class="text-gray-800 dark:text-gray-200">
                    {"(e.g., From Control Center on the Mac)"}
                </p>
                <h3 class="mt-3 text-lg font-bold">{"About"}</h3>
                <p class="text-xs text-gray-600 dark:text-gray-400">
                    { "Created by " }
                    <a
                        class="text-green-600 hover:text-green-500 dark:text-green-300 dark:hover:text-green-200"
                        href="https://stevenfukase.com"
                    >
                        { "STEVEN FUKASE" }
                    </a>
                    <br />
                    { "with Rust and Yew" }
                </p>
            </div>
        </aside>
    }
}
