use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    api::api_create_account,
    models::CreateAccountPayload,
    notice::{Notice, error_notice, render_notice, success_notice},
};

#[component]
pub fn CreateAccountPage() -> Html {
    let owner_name = use_state(String::new);
    let initial_balance = use_state(String::new);
    let notice = use_state(|| None::<Notice>);

    let on_owner_name_input = {
        let owner_name = owner_name.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            owner_name.set(input.value());
        })
    };

    let on_initial_balance_input = {
        let initial_balance = initial_balance.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            initial_balance.set(input.value());
        })
    };

    let on_submit = {
        let owner_name = owner_name.clone();
        let initial_balance = initial_balance.clone();
        let notice = notice.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let owner_name_value = owner_name.trim().to_string();
            if owner_name_value.is_empty() {
                notice.set(Some(error_notice("Owner name is required")));
                return;
            }

            let initial_balance_value = initial_balance.trim().to_string();
            let payload = CreateAccountPayload {
                owner_name: owner_name_value,
                initial_balance: if initial_balance_value.is_empty() {
                    None
                } else {
                    Some(initial_balance_value)
                },
            };

            let owner_name = owner_name.clone();
            let initial_balance = initial_balance.clone();
            let notice = notice.clone();

            spawn_local(async move {
                match api_create_account(payload).await {
                    Ok(_) => {
                        owner_name.set(String::new());
                        initial_balance.set(String::new());
                        notice.set(Some(success_notice("Account created successfully")));
                    }
                    Err(err) => notice.set(Some(error_notice(err))),
                }
            });
        })
    };

    html! {
        <section class="grid">
            <article class="card half">
                <h2>{ "Create Account" }</h2>
                { render_notice((*notice).clone()) }
                <form onsubmit={on_submit}>
                    <label>
                        { "Owner Name" }
                        <input
                            value={(*owner_name).clone()}
                            oninput={on_owner_name_input}
                            placeholder="Jane Doe"
                        />
                    </label>
                    <label>
                        { "Initial Balance (optional)" }
                        <input
                            value={(*initial_balance).clone()}
                            oninput={on_initial_balance_input}
                            placeholder="1000.00"
                        />
                    </label>
                    <button type="submit">{ "Create" }</button>
                </form>
            </article>
        </section>
    }
}
