use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    api::{api_list_accounts, api_withdraw},
    models::{Account, BalanceMutationPayload},
    notice::{Notice, error_notice, render_notice, success_notice},
};

#[component]
pub fn WithdrawPage() -> Html {
    let accounts = use_state(Vec::<Account>::new);
    let account_id = use_state(String::new);
    let amount = use_state(String::new);
    let description = use_state(String::new);
    let notice = use_state(|| None::<Notice>);

    {
        let accounts = accounts.clone();
        let notice = notice.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match api_list_accounts().await {
                    Ok(items) => accounts.set(items),
                    Err(err) => notice.set(Some(error_notice(err))),
                }
            });

            || ()
        });
    }

    let on_account_change = {
        let account_id = account_id.clone();
        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            account_id.set(select.value());
        })
    };

    let on_amount_input = {
        let amount = amount.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            amount.set(input.value());
        })
    };

    let on_description_input = {
        let description = description.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            description.set(input.value());
        })
    };

    let on_submit = {
        let account_id = account_id.clone();
        let amount = amount.clone();
        let description = description.clone();
        let notice = notice.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let account_id_value = account_id.trim().to_string();
            if account_id_value.is_empty() {
                notice.set(Some(error_notice("Select an account for withdrawal")));
                return;
            }

            let amount_value = amount.trim().to_string();
            if amount_value.is_empty() {
                notice.set(Some(error_notice("Withdrawal amount is required")));
                return;
            }

            let description_value = description.trim().to_string();
            let payload = BalanceMutationPayload {
                amount: amount_value,
                description: if description_value.is_empty() {
                    None
                } else {
                    Some(description_value)
                },
            };

            let amount = amount.clone();
            let description = description.clone();
            let notice = notice.clone();

            spawn_local(async move {
                match api_withdraw(&account_id_value, payload).await {
                    Ok(_) => {
                        amount.set(String::new());
                        description.set(String::new());
                        notice.set(Some(success_notice("Withdrawal completed")));
                    }
                    Err(err) => notice.set(Some(error_notice(err))),
                }
            });
        })
    };

    html! {
        <section class="grid">
            <article class="card half">
                <h2>{ "Withdraw" }</h2>
                { render_notice((*notice).clone()) }
                <form onsubmit={on_submit}>
                    <label>
                        { "Account" }
                        <select onchange={on_account_change} value={(*account_id).clone()}>
                            <option value="">{ "Select account" }</option>
                            { for accounts.iter().map(|account| html! {
                                <option value={account.id.clone()}>{ account.owner_name.clone() }</option>
                            })}
                        </select>
                    </label>
                    <label>
                        { "Amount" }
                        <input
                            value={(*amount).clone()}
                            oninput={on_amount_input}
                            placeholder="120.00"
                        />
                    </label>
                    <label>
                        { "Description" }
                        <input
                            value={(*description).clone()}
                            oninput={on_description_input}
                            placeholder="ATM cash"
                        />
                    </label>
                    <button class="secondary" type="submit">{ "Withdraw" }</button>
                </form>
            </article>
        </section>
    }
}
