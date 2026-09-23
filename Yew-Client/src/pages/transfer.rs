use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    api::{api_list_accounts, api_transfer},
    models::{Account, TransferPayload},
    notice::{Notice, error_notice, render_notice, success_notice},
};

#[component]
pub fn TransferPage() -> Html {
    let accounts = use_state(Vec::<Account>::new);
    let from_account_id = use_state(String::new);
    let to_account_id = use_state(String::new);
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

    let on_from_account_change = {
        let from_account_id = from_account_id.clone();
        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            from_account_id.set(select.value());
        })
    };

    let on_to_account_change = {
        let to_account_id = to_account_id.clone();
        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            to_account_id.set(select.value());
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
        let from_account_id = from_account_id.clone();
        let to_account_id = to_account_id.clone();
        let amount = amount.clone();
        let description = description.clone();
        let notice = notice.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let from_account_id_value = from_account_id.trim().to_string();
            let to_account_id_value = to_account_id.trim().to_string();

            if from_account_id_value.is_empty() || to_account_id_value.is_empty() {
                notice.set(Some(error_notice(
                    "Select both source and destination accounts",
                )));
                return;
            }

            if from_account_id_value == to_account_id_value {
                notice.set(Some(error_notice(
                    "Source and destination must be different accounts",
                )));
                return;
            }

            let amount_value = amount.trim().to_string();
            if amount_value.is_empty() {
                notice.set(Some(error_notice("Transfer amount is required")));
                return;
            }

            let description_value = description.trim().to_string();
            let payload = TransferPayload {
                from_account_id: from_account_id_value,
                to_account_id: to_account_id_value,
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
                match api_transfer(payload).await {
                    Ok(_) => {
                        amount.set(String::new());
                        description.set(String::new());
                        notice.set(Some(success_notice("Transfer completed")));
                    }
                    Err(err) => notice.set(Some(error_notice(err))),
                }
            });
        })
    };

    html! {
        <section class="grid">
            <article class="card half">
                <h2>{ "Transfer" }</h2>
                { render_notice((*notice).clone()) }
                <form onsubmit={on_submit}>
                    <label>
                        { "From" }
                        <select onchange={on_from_account_change} value={(*from_account_id).clone()}>
                            <option value="">{ "Source account" }</option>
                            { for accounts.iter().map(|account| html! {
                                <option value={account.id.clone()}>{ account.owner_name.clone() }</option>
                            })}
                        </select>
                    </label>
                    <label>
                        { "To" }
                        <select onchange={on_to_account_change} value={(*to_account_id).clone()}>
                            <option value="">{ "Destination account" }</option>
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
                            placeholder="90.00"
                        />
                    </label>
                    <label>
                        { "Description" }
                        <input
                            value={(*description).clone()}
                            oninput={on_description_input}
                            placeholder="Savings transfer"
                        />
                    </label>
                    <button type="submit">{ "Transfer" }</button>
                </form>
            </article>
        </section>
    }
}
