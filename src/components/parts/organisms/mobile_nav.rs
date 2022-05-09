use crate::components::parts::molecules::labeled_icon_link;
use yew::{function_component, html};
use yew_octicons::IconKind;

#[function_component(MobileNav)]
pub fn mobile_nav() -> Html {
    html! {
        <div class="backdrop-blur-sm flex justify-between fixed sm:hidden w-full py-1 px-3 bottom-0">
            <labeled_icon_link::LabeledIcon
                href="/"
                label="Home"
                icon={IconKind::Home}
            />
            // <labeled_icon_link::LabeledIcon
            //     href="/alerts"
            //     label="alerts"
            //     icon={IconKind::Bell}
            // />
            <labeled_icon_link::LabeledIcon
                href="/new"
                label="New"
                icon={IconKind::PlusCircle}
            />
            <labeled_icon_link::LabeledIcon
                href="/profile"
                label="Profile"
                icon={IconKind::Person}
            />
        </div>
    }
}
