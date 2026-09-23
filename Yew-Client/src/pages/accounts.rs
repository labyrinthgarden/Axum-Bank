use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::api_list_accounts,
    models::Account,
    notice::{Notice, error_notice, render_notice},
    utils::{format_money, human_time},
};

#[component]
pub fn AccountsPage() -> Html {
    let accounts = use_state(Vec::<Account>::new);
    let notice = use_state(|| None::<Notice>);
    let loading = use_state(|| false);

    {
        let accounts = accounts.clone();
        let notice = notice.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match api_list_accounts().await {
                    Ok(items) => accounts.set(items),
                    Err(err) => notice.set(Some(error_notice(err))),
                }
                loading.set(false);
            });

            || ()
        });
    }

    html! {
        <section class="grid">
            <article class="card">
                <h2>{ "Accounts" }</h2>
                if *loading {
                    <p class="empty">{ "Loading accounts..." }</p>
                } else {
                    { render_notice((*notice).clone()) }
                    {
                        if accounts.is_empty() {
                            html! { <p class="empty">{ "No accounts yet. Create one to get started." }</p> }
                        } else {
                            html! {
                                <table class="accounts">
                                    <thead>
                                        <tr>
                                            <th>{ "Owner" }</th>
                                            <th>{ "Balance" }</th>
                                            <th>{ "Account ID" }</th>
                                            <th>{ "Created" }</th>
                                            <th>{ "Last Updated" }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { for accounts.iter().map(view_account_row) }
                                    </tbody>
                                </table>
                            }
                        }
                    }
                }
            </article>
        </section>
    }
}

fn view_account_row(account: &Account) -> Html {
    html! {
        <tr>
            <td>{ &account.owner_name }</td>
            <td class="money positive">{ format_money(&account.balance) }</td>
            <td>{ &account.id }</td>
            <td>{ human_time(&account.created_at) }</td>
            <td>{ human_time(&account.updated_at) }</td>
        </tr>
    }
}
