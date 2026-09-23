use yew::prelude::*;

use crate::route::Route;

pub fn route_link(
    label: &str,
    route: Route,
    class_name: &str,
    on_navigate: Callback<Route>,
) -> Html {
    let href = route.href();
    let label = label.to_string();
    let class_name = class_name.to_string();
    let onclick = Callback::from(move |event: MouseEvent| {
        event.prevent_default();
        on_navigate.emit(route.clone());
    });

    html! {
        <a href={href} class={class_name} {onclick}>{ label }</a>
    }
}
