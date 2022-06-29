use std::rc::Rc;

use rand::{thread_rng, Rng};
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Properties, Reducible,
};

enum CounterAction {
    Double,
    Square,
}

#[derive(Clone, Debug, PartialEq)]
struct AppState {
    current_ward: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self { current_ward: 0 }
    }
}

impl Reducible for AppState {
    type Action = u8;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        AppState {
            current_ward: action,
        }
        .into()
    }
}

#[derive(Properties, PartialEq)]
struct AppContextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppContext)]
fn app_context(props: &AppContextProps) -> Html {
    let app_reducer = use_reducer(AppState::default);
    // let generate = {
    //     let mut rng = thread_rng();
    //     let id = rng.gen_range(0..=22);
    //     Callback::from(move |_| current_ward.set(id))
    // };

    html! {
      <ContextProvider<AppState> context={app_reducer}>
        {props.children.clone()}
      </ContextProvider<AppState>>
    }
}
