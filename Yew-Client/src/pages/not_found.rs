use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::routes::Route;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
      <article class="card">
        <h2>{ "Page not found" }</h2>
        <p class="empty">{ "The page you requested does not exist." }</p>

        <a href={"/"}>
          {"Back to homepage"}
        </a>
      </article>
    }
}
