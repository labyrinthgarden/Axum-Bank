use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::routes::Route;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
  let navigator = use_navigator().unwrap();
  let username = use_state(String::new);
  let password = use_state(String::new);
  let error = use_state(|| None::<String>);

  let on_username_input = {
    let username = username.clone();
    Callback::from(move |event: InputEvent| {
      let input: HtmlInputElement = event.target_unchecked_into();
      username.set(input.value());
    })
  };

  let on_password_input = {
    let password = password.clone();
    Callback::from(move |event: InputEvent| {
      let input: HtmlInputElement = event.target_unchecked_into();
      password.set(input.value());
    })
  };

  let on_submit = {
    let username = username.clone();
    let password = password.clone();
    let error = error.clone();
    let navigator = navigator.clone();

    Callback::from(move |event: SubmitEvent| {
      event.prevent_default();

      if username.trim().is_empty() || password.trim().is_empty() {
        error.set(Some("Username and password are required".into()));
        return;
      }

      error.set(None);
      navigator.push(&Route::Home);
    })
  };

  html! {
    <main class="container">
      if let Some(msg) = (*error).clone() {
        <p class="error">{ msg }</p>
      }

      <section class="grid">
        <article class="card auth-card">
          <h2>{ "Login" }</h2>
          <form onsubmit={on_submit}>
            <label>
              { "Username" }
              <input
                autocomplete="username"
                value={(*username).clone()}
                oninput={on_username_input}
                placeholder="jane.doe"
              />
            </label>
            <label>
              { "Password" }
              <input
                type="password"
                autocomplete="current-password"
                value={(*password).clone()}
                oninput={on_password_input}
                placeholder="••••••••"
              />
            </label>
            <Link<Route> to={Route::Register}>{ "Don't have and account?. Sign Up" }</Link<Route>>
            <button type="submit">{ "Sign in" }</button>
          </form>
        </article>
      </section>
    </main>
  }
}
