use rand::{thread_rng, Rng};
use std::rc::Rc;
use yew::{
    function_component, html, use_reducer, Children, ContextProvider, Properties, Reducible,
    UseReducerHandle,
};

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
    thread_rng().gen_range(0..=range.unwrap_or(22))
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_ward: generate_rand_int(None),
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
                if self.disabled_wards.len() == 23 {
                    return self;
                }
                let enabled_ward_ids = (0..=22)
                    .into_iter()
                    .filter(|val| !self.disabled_wards.contains(val))
                    .collect::<Vec<u8>>();
                let rand_int = generate_rand_int(Some(enabled_ward_ids.len() as u8 - 1));
                new_state.current_ward = enabled_ward_ids[rand_int as usize];
            }
            AppStateAction::Disable(id) => {
                new_state.disabled_wards.push(id);
            }
            AppStateAction::Enable(id) => {
                new_state.disabled_wards.retain(|val| val != &id);
            }
        };
        log::debug!("App State: {:#?}", new_state);
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
