mod components;
mod constants;
mod pages;

use std::rc::Rc;

use components::parts::organisms::layout::Layout;
use rand::{thread_rng, Rng};
use yew::{
    function_component, html, use_reducer, use_reducer_eq, Callback, ContextProvider, Html,
    MouseEvent, Reducible,
};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
enum Routes {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Routes) -> Html {
    match routes {
        Routes::Home => html! { <pages::index::IndexPage /> },
        Routes::NotFound => html! {<pages::not_found::NotFoundPage />},
    }
}

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

#[function_component(App)]
fn app() -> Html {
    let app_state = use_reducer(AppState::default);
    // let generate = {
    //     let mut rng = thread_rng();
    //     let id = rng.gen_range(0..=22);
    //     Callback::from(move |_| current_ward.set(id))
    // };

    html! {
      <ContextProvider<AppState> context={app_state}>
        <Layout>
          <BrowserRouter>
            <Switch<Routes> render={Switch::render(switch)} />
          </BrowserRouter>
        </Layout>
      </ContextProvider<AppState>>
    }
}

fn main() {
    yew::start_app::<App>();
}
