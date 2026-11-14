use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};
use crate::routes::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
  html! {
    <section class="grid">
      <article class="card">
        <div style="display:flex; justify-content:space-between;">
          <h2>{ "Welcome" }</h2>

          <div style="display:flex; gap:1rem;">
            <div class="option-container">
              <Link<Route> to={Route::Profile}>
                <button class="option-button secondary">
                  <Icon width={"2rem".to_owned()} height={"2rem".to_owned()} data={IconData::FONT_AWESOME_SOLID_USER}/>
                </button>
              </Link<Route>>
              <label>{ "Profile" }</label>
            </div>

            <div class="option-container">
              <Link<Route> to={Route::Login}>
                <button class="option-button secondary">
                  <Icon width={"2rem".to_owned()} height={"2rem".to_owned()} data={IconData::FONT_AWESOME_SOLID_RIGHT_FROM_BRACKET}/>
                </button>
              </Link<Route>>
              <label>{ "Log out" }</label>
            </div>
          </div>
        </div>
      </article>

      <article class="card">
        <h2>{ "Quick Actions" }</h2>
        <div class="options-container">
          <div class="option-container">
            <Link<Route> to={Route::Deposit}>
              <button class="option-button">
                <Icon width={"5rem".to_owned()} height={"5rem".to_owned()} data={IconData::FONT_AWESOME_SOLID_PLUS}/>
              </button>
            </Link<Route>>
            <label>{ "Deposit" }</label>
          </div>

          <div class="option-container">
            <Link<Route> to={Route::Transfer}>
              <button class="option-button">
                <Icon width={"5rem".to_owned()} height={"5rem".to_owned()} data={IconData::FONT_AWESOME_SOLID_MONEY_BILL_TRANSFER}/>
              </button>
            </Link<Route>>
            <label>{ "Transfer" }</label>
          </div>

          <div class="option-container">
            <Link<Route> to={Route::Transactions}>
              <button class="option-button">
                <Icon width={"5rem".to_owned()} height={"5rem".to_owned()} data={IconData::FONT_AWESOME_SOLID_LIST_UL}/>
              </button>
            </Link<Route>>
            <label>{ "Transactions" }</label>
          </div>

          <div class="option-container">
            <Link<Route> to={Route::Insights}>
              <button class="option-button">
                <Icon width={"5rem".to_owned()} height={"5rem".to_owned()} data={IconData::FONT_AWESOME_SOLID_ARROW_UP_RIGHT_DOTS }/>
              </button>
            </Link<Route>>
            <label>{ "Account Insights" }</label>
          </div>
        </div>
      </article>
    </section>
  }
}
