mod components;
mod constants;
mod pages;

use components::parts::organisms::layout::Layout;
use rand::{thread_rng, Rng};
use yew::{function_component, html, Callback, ContextProvider, Html, MouseEvent, use_reducer_eq};
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

#[derive(Clone, Debug, PartialEq)]
struct AppContext {
    current_ward: u8,
    generate: Callback<MouseEvent>,
}

#[function_component(App)]
fn app() -> Html {
    let current_ward = use_reducer_eq(|| 0);
    let generate = {
        let mut rng = thread_rng();
        let id = rng.gen_range(0..=22);
        Callback::from(move |_| current_ward.set(id))
    };

    html! {
      <ContextProvider<AppContext> context={context.clone()}>
        <Layout>
          <BrowserRouter>
            <Switch<Routes> render={Switch::render(switch)} />
          </BrowserRouter>
        </Layout>
      </ContextProvider<AppContext>>
    }
}

fn main() {
    yew::start_app::<App>();
}
