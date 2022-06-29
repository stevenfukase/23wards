use rand::{thread_rng, Rng};
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Properties, Reducible,
    UseReducerHandle,
};

pub enum AppStateAction {
    Generate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub current_ward: u8,
}

impl Default for AppState {
    fn default() -> Self {
        Self { current_ward: 0 }
    }
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::Generate => {
                let mut rng = thread_rng();
                let id = rng.gen_range(0..=22);
                log::info!("Generated Ward Id: {}", id);
                AppState { current_ward: id }.into()
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AppContextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppContext)]
pub fn app_context(props: &AppContextProps) -> Html {
    let app_reducer = use_reducer(|| AppState::default());

    html! {
      <ContextProvider<UseReducerHandle<AppState>> context={app_reducer}>
        {props.children.clone()}
      </ContextProvider<UseReducerHandle<AppState>>>
    }
}
