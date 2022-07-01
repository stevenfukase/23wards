use rand::{thread_rng, Rng};
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Properties, Reducible,
    UseReducerHandle,
};

use crate::constants::wards::WARDS;

pub enum AppStateAction {
    Generate,
    Disable(u8),
    Enable(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub current_ward: u8,
    pub disabled_wards: Vec<u8>,
}

fn generate_rand_int(range: Option<u8>) -> u8 {
    let range = range.unwrap_or(22);
    let mut thread = thread_rng();
    let id = thread.gen_range(0..=range);
    log::info!("Generated Ward Id: {}", id);
    id
}

impl Default for AppState {
    fn default() -> Self {
        let id = generate_rand_int(None);
        Self {
            current_ward: id,
            disabled_wards: vec![],
        }
    }
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_state = Self::clone(&self);
        match action {
            AppStateAction::Generate => {
                let rand_int = generate_rand_int(Some(self.disabled_wards.len() as u8));
                let enabled_wards = &WARDS
                    .into_iter()
                    .filter(|val| self.disabled_wards.contains(&val.id))
                    .collect::<u8>();
                new_state.current_ward = enabled_wards[rand_int];
            }
            AppStateAction::Disable(id) => {
                new_state.disabled_wards.push(id);
            }
            AppStateAction::Enable(id) => {
                new_state.disabled_wards.retain(|val| val != &id);
            }
        };

        log::info!("App State: {:#?}", new_state);
        new_state.into()
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
