use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod routes;
mod layouts;
mod components;

use layouts::MainLayout;
use pages::{LoginPage,RegisterPage,HomePage,ProfilePage,DepositPage,TransferPage,TransactionsPage,InsightsPage,NotFoundPage};
use routes::Route;

fn switch(routes:Route) -> Html {
  match routes {
    Route::Login => html! { <LoginPage /> },
    Route::Register => html! { <RegisterPage /> },
    Route::Home => html! { <HomePage /> },
    Route::Profile => html! { <ProfilePage /> },
    Route::Deposit => html! { <DepositPage /> },
    Route::Transfer => html! { <TransferPage/> },
    Route::Transactions => html! { <TransactionsPage /> },
    Route::Insights => html! { <InsightsPage /> },
    Route::NotFound => html! { <NotFoundPage /> }
  }
}

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <MainLayout>
        <Switch<Route> render={switch} />
      </MainLayout>
    </BrowserRouter>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
