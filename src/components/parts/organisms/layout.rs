use super::{navbar, sidebar};
use yew::{function_component, html, use_state, Callback, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let is_sidebar_open = use_state(|| false);
    let on_sidebar_close = {
        let is_sidebar_open = is_sidebar_open.clone();
        Callback::from(move |_| is_sidebar_open.set(!*is_sidebar_open))
    };

    html! {
      <>
        <navbar::Navbar />
        <sidebar::SideBar is_open={(*is_sidebar_open).clone()} on_close={on_sidebar_close} />
        <main>
          {props.children.clone()}
        </main>
      </>
    }
}
