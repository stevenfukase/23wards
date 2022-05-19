mod components;
mod constants;
mod pages;

use components::parts::organisms::layout::Layout;
use yew::prelude::*;
// use yew::{function_component, html, ContextProvider, Html};
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
struct CurrentWard {
    ward_id: u8,
}

#[function_component(App)]
fn app() -> Html {
    let context = use_memo(|_| CurrentWard { ward_id: 0 }, ());

    html! {
      <ContextProvider<CurrentWard> context={context}>
        <Layout>
          <BrowserRouter>
            <Switch<Routes> render={Switch::render(switch)} />
          </BrowserRouter>
        </Layout>
      </ContextProvider<CurrentWard>>
    }
}

fn main() {
    yew::start_app::<App>();
}
