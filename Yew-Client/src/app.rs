use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

use crate::{
    pages::{
        AccountsPage, CreateAccountPage, DashboardPage, DepositPage, LoginPage, NotFoundPage,
        TransactionsPage, TransferPage, WithdrawPage, nav::route_link,
    },
    route::Route,
};

const AUTH_STORAGE_KEY: &str = "axum_bank_logged_in";

#[component]
pub fn App() -> Html {
    let logged_in = use_state(read_login_state);
    let route = use_state(read_hash_route);

    {
        let route = route.clone();
        use_effect(move || {
            let listener = web_sys::window().map(|window| {
                let route = route.clone();
                EventListener::new(&window, "hashchange", move |_| {
                    route.set(read_hash_route());
                })
            });

            move || drop(listener)
        });
    }

    {
        let route = route.clone();
        let logged_in = *logged_in;

        use_effect_with(
            ((*route).clone(), logged_in),
            move |(current_route, is_logged_in)| {
                let guarded = guard_route(current_route, *is_logged_in);
                if &guarded != current_route {
                    write_hash_route(&guarded);
                    route.set(guarded);
                }

                || ()
            },
        );
    }

    let on_navigate = {
        let route = route.clone();
        Callback::from(move |target: Route| {
            write_hash_route(&target);
            route.set(target);
        })
    };

    let on_login = {
        let logged_in = logged_in.clone();
        let on_navigate = on_navigate.clone();

        Callback::from(move |_| {
            let _ = LocalStorage::set(AUTH_STORAGE_KEY, true);
            logged_in.set(true);
            on_navigate.emit(Route::Dashboard);
        })
    };

    let on_logout = {
        let logged_in = logged_in.clone();
        let on_navigate = on_navigate.clone();

        Callback::from(move |_| {
            LocalStorage::delete(AUTH_STORAGE_KEY);
            logged_in.set(false);
            on_navigate.emit(Route::Login);
        })
    };

    let guarded_route = guard_route(&route, *logged_in);

    match guarded_route {
        Route::Login => html! { <LoginPage on_login={on_login} /> },
        Route::Dashboard
        | Route::Accounts
        | Route::CreateAccount
        | Route::Transactions
        | Route::Deposit
        | Route::Withdraw
        | Route::Transfer
        | Route::NotFound => {
            let content = match guarded_route {
                Route::Dashboard => html! { <DashboardPage on_navigate={on_navigate.clone()} /> },
                Route::Accounts => html! { <AccountsPage /> },
                Route::CreateAccount => html! { <CreateAccountPage /> },
                Route::Transactions => html! { <TransactionsPage /> },
                Route::Deposit => html! { <DepositPage /> },
                Route::Withdraw => html! { <WithdrawPage /> },
                Route::Transfer => html! { <TransferPage /> },
                Route::NotFound => html! { <NotFoundPage on_navigate={on_navigate.clone()} /> },
                Route::Root | Route::Login => html! {},
            };

            app_shell(on_logout, on_navigate, content)
        }
        Route::Root => html! {},
    }
}

fn app_shell(on_logout: Callback<()>, on_navigate: Callback<Route>, content: Html) -> Html {
    let on_logout_click = Callback::from(move |_| on_logout.emit(()));

    html! {
        <main class="container">
            <header class="header">
                <nav class="top-nav">
                    <div>
                        <h1>{ "Axum Bank" }</h1>
                        <p class="subtitle">{ "Simple, Safety." }</p>
                    </div>
                    <div class="top-nav-links">
                        { route_link("Transfer", Route::Transfer, "top-nav-link", on_navigate.clone()) }
                        { route_link("Deposit", Route::Deposit, "top-nav-link", on_navigate.clone()) }
                        { route_link("Withdraw", Route::Withdraw, "top-nav-link", on_navigate.clone()) }
                        { route_link("Transactions", Route::Transactions, "top-nav-link", on_navigate.clone()) }
                    </div>
                    <button class="secondary" type="button" onclick={on_logout_click}>{ "Logout" }</button>
                </nav>
            </header>


            { content }
        </main>
    }
}

fn guard_route(route: &Route, logged_in: bool) -> Route {
    if !logged_in {
        if route.requires_auth() {
            return Route::Login;
        }

        return match route {
            Route::Root => Route::Login,
            _ => route.clone(),
        };
    }

    match route {
        Route::Root | Route::Login => Route::Dashboard,
        _ => route.clone(),
    }
}

fn read_login_state() -> bool {
    LocalStorage::get::<bool>(AUTH_STORAGE_KEY).unwrap_or(false)
}

fn read_hash_route() -> Route {
    let hash = web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .unwrap_or_default();

    Route::from_hash(&hash)
}

fn write_hash_route(route: &Route) {
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_hash(route.fragment());
    }
}
