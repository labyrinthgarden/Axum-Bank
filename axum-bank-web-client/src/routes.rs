use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/login")]
  Login,
  #[at("/register")]
  Register,
  #[at("/")]
  Home,
  #[at("/profile")]
  Profile,
  #[at("/deposit")]
  Deposit,
  #[at("/transfer")]
  Transfer,
  #[at("/transactions")]
  Transactions,
  #[at("/insights")]
  Insights,
  #[not_found]
  #[at("/404")]
  NotFound,
}
