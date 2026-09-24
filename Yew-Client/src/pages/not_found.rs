use yew::prelude::*;

use crate::route::Route;

use super::nav::route_link;

#[derive(Properties, PartialEq)]
pub struct NotFoundPageProps {
    pub on_navigate: Callback<Route>,
}

#[component]
pub fn NotFoundPage(props: &NotFoundPageProps) -> Html {
    html! {
        <section class="grid">
            <article class="card half">
                <h2>{ "Page not found" }</h2>
                <p class="empty">{ "The page you requested does not exist." }</p>
                { route_link("Go back to Home Page", Route::Home, "action-link", props.on_navigate.clone()) }
            </article>
        </section>
    }
}
