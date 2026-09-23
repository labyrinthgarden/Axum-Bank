use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::{
    api::{api_list_accounts, api_list_transactions},
    models::{Account, Transaction},
    notice::{Notice, error_notice, render_notice},
    utils::{format_money, human_time, prettify_kind},
};

#[component]
pub fn TransactionsPage() -> Html {
    let accounts = use_state(Vec::<Account>::new);
    let selected_account_id = use_state(String::new);
    let transactions = use_state(Vec::<Transaction>::new);
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

    {
        let selected_account_id = selected_account_id.clone();
        let transactions = transactions.clone();
        let notice = notice.clone();

        use_effect_with((*selected_account_id).clone(), move |account_id| {
            if account_id.is_empty() {
                transactions.set(vec![]);
            } else {
                let account_id = account_id.clone();
                spawn_local(async move {
                    match api_list_transactions(&account_id).await {
                        Ok(items) => transactions.set(items),
                        Err(err) => notice.set(Some(error_notice(err))),
                    }
                });
            }

            || ()
        });
    }

    let on_selected_account_change = {
        let selected_account_id = selected_account_id.clone();
        Callback::from(move |event: Event| {
            let select: HtmlSelectElement = event.target_unchecked_into();
            selected_account_id.set(select.value());
        })
    };

    html! {
        <section class="grid">
            <article class="card">
                <h2>{ "Transaction Viewer" }</h2>
                { render_notice((*notice).clone()) }

                <label>
                    { "Choose Account" }
                    <select onchange={on_selected_account_change} value={(*selected_account_id).clone()}>
                        <option value="">{ "Select account" }</option>
                        { for accounts.iter().map(|account| html! {
                            <option value={account.id.clone()}>{ format!("{} ({})", account.owner_name, account.id) }</option>
                        })}
                    </select>
                </label>

                {
                    if selected_account_id.is_empty() {
                        html! { <p class="empty">{ "Pick an account to load its latest transactions." }</p> }
                    } else if transactions.is_empty() {
                        html! { <p class="empty">{ "No transactions yet for this account." }</p> }
                    } else {
                        html! {
                            <ul class="transactions">
                                { for transactions.iter().map(view_transaction) }
                            </ul>
                        }
                    }
                }
            </article>
        </section>
    }
}

fn view_transaction(transaction: &Transaction) -> Html {
    let is_credit = matches!(
        transaction.kind.as_str(),
        "deposit" | "transfer_in" | "initial_deposit"
    );
    let class = if is_credit {
        "money positive"
    } else {
        "money negative"
    };
    let direction = if is_credit { "+" } else { "-" };

    html! {
        <li>
            <div>
                <strong>{ prettify_kind(&transaction.kind) }</strong>
                <br />
                <small>{ format!("Account: {}", transaction.account_id) }</small>
                <br />
                {
                    if let Some(counterparty) = &transaction.counterparty_account_id {
                        html! {
                            <>
                                <small>{ format!("Counterparty: {counterparty}") }</small>
                                <br />
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
                <small>{ transaction.description.clone().unwrap_or_else(|| "No description".to_string()) }</small>
                <br />
                <small>{ format!("Ref: {}", transaction.id) }</small>
                <br />
                <small>{ human_time(&transaction.created_at) }</small>
            </div>
            <div class={class}>{ format!("{}{}", direction, format_money(&transaction.amount)) }</div>
        </li>
    }
}
