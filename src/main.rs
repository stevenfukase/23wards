mod components;
mod constants;
mod pages;

use components::parts::organisms::layout::Layout;
use yew::{function_component, html, Html};
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

#[function_component(App)]
fn app() -> Html {
    html! {
      <Layout>
        <BrowserRouter>
          <Switch<Routes> render={Switch::render(switch)} />
        </BrowserRouter>
      </Layout>
    }
}

fn main() {
    yew::start_app::<App>();
}
