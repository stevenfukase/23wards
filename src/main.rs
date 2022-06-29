mod components;
mod constants;
mod contexts;
mod pages;

use components::parts::organisms::layout::Layout;
use contexts::app_context;
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
      <app_context::AppContext>
        <Layout>
          <BrowserRouter>
            <Switch<Routes> render={Switch::render(switch)} />
          </BrowserRouter>
        </Layout>
      </app_context::AppContext>
    }
}

fn main() {
    yew::start_app::<App>();
}
