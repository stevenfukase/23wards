use super::{navbar, sidebar};
use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
      <>
        <navbar::Navbar />
        <sidebar::SideBar />
        <main>
          {props.children.clone()}
        </main>
      </>
    }
}
