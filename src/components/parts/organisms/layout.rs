use super::mobile_nav;
use super::navbar;
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
        <main>
          {props.children.clone()}
        </main>
        <mobile_nav::MobileNav />
      </>
    }
}
