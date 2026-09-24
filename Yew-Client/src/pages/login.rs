use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::notice::{Notice, error_notice, render_notice, success_notice};

#[derive(Properties, PartialEq)]
pub struct LoginPageProps {
    pub on_login: Callback<()>,
}

#[component]
pub fn LoginPage(props: &LoginPageProps) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let notice = use_state(|| None::<Notice>);

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
        let notice = notice.clone();
        let on_login = props.on_login.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            if username.trim().is_empty() || password.trim().is_empty() {
                notice.set(Some(error_notice("Username and password are required")));
                return;
            }

            notice.set(Some(success_notice("Login successful")));
            on_login.emit(());
        })
    };

    html! {
        <main class="container">
            <header class="header">
                <div>
                    <h1>{ "Axum Bank" }</h1>
                    <p class="subtitle">{ "Please sign in." }</p>
                </div>
            </header>

            { render_notice((*notice).clone()) }

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
                        <a href="/">{"Don't have an account? Sign up"}</a>
                        <button type="submit">{ "Sign in" }</button>
                    </form>
                </article>
            </section>
        </main>
    }
}
