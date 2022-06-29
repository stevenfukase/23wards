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

fn generate_rand_int() -> u8 {
    let mut rng = thread_rng();
    let id = rng.gen_range(0..=22);
    log::info!("Generated Ward Id: {}", id);
    id
}

impl Default for AppState {
    fn default() -> Self {
        let id = generate_rand_int();
        Self { current_ward: id }
    }
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::Generate => {
                let id = generate_rand_int();
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
