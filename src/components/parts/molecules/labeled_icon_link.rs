use yew::{function_component, html, Properties};
use yew_octicons::{Icon, IconKind};

#[derive(Properties, PartialEq)]
pub struct LabeledIconProps {
    pub href: String,
    pub label: String,
    pub icon: IconKind,
}

#[function_component(LabeledIcon)]
pub fn labeled_icon(props: &LabeledIconProps) -> Html {
    html! {
        <a href={props.href.to_owned()}>
            <div class="flex flex-col p-2 w-min">
                <div class="flex justify-center">{Icon::new(props.icon)}</div>
                <div class="text-xs text-center">{props.label.to_owned()}</div>
            </div>
        </a>
    }
}
