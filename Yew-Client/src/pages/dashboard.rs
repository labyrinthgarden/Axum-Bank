use yew::prelude::*;

use crate::route::Route;

use super::nav::route_link;

#[derive(Properties, PartialEq)]
pub struct DashboardPageProps {
    pub on_navigate: Callback<Route>,
}

#[component]
pub fn DashboardPage(props: &DashboardPageProps) -> Html {
    html! {
        <section class="grid">
            <article class="card half">
                <h2>{ "Welcome" }</h2>
                <p class="subtitle">
                    { "Use the navigation below to manage accounts, transactions, and money operations." }
                </p>
            </article>

            <article class="card half">
                <h2>{ "Quick Actions" }</h2>
                <div class="nav-grid">
                    { route_link("View Accounts", Route::Accounts, "action-link", props.on_navigate.clone()) }
                    { route_link("Create Account", Route::CreateAccount, "action-link", props.on_navigate.clone()) }
                    { route_link("View Transactions", Route::Transactions, "action-link", props.on_navigate.clone()) }
                    { route_link("Deposit", Route::Deposit, "action-link", props.on_navigate.clone()) }
                    { route_link("Withdraw", Route::Withdraw, "action-link", props.on_navigate.clone()) }
                    { route_link("Transfer", Route::Transfer, "action-link", props.on_navigate.clone()) }
                </div>
            </article>
        </section>
    }
}
