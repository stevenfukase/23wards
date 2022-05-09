use yew::{function_component, html};

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
      <>
        { "404" }
      </>
    }
}
