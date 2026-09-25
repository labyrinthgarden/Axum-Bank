use yew::prelude::*;

use crate::components::{Header};

#[derive(Properties, PartialEq)]
pub struct MainLayoutProps {
  #[prop_or_default]
  pub children: Children,
}

#[function_component(MainLayout)]
pub fn main_layout(props: &MainLayoutProps) -> Html {
  html! {
    <div class="app-container">
      <Header/>
      <main class="app-content">
        { props.children.clone() }
      </main>
    </div>
  }
}
