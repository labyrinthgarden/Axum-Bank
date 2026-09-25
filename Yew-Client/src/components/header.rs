use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
  html! {
    <header class="header">
        <div>
            <h1>{ "Axum Bank" }</h1>
            <p class="subtitle">{ "Simple, Safe" }</p>
        </div>
    </header>
  }
}
