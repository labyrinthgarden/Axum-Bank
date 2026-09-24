use yew::prelude::*;
use yew_icons::{Icon,IconData};

use crate::route::Route;

use super::nav::route_link;

#[derive(Properties, PartialEq)]
pub struct HomePageProps {
    pub on_navigate: Callback<Route>,
}

#[component]
pub fn HomePage(props: &HomePageProps) -> Html {
    html! {
        <section class="grid">
            <article class="card">
                <h2>{ "Welcome" }</h2>
                <p class="subtitle">
                    { "_____________________________________________________________________________________________" }
                </p>
                <h2>{ "Quick Actions" }</h2>
                    <div class="options-container">
                        <a >
                            <button class="option-button">
                                <Icon width={"9rem".to_owned()} height={"9rem".to_owned()} data={IconData::LUCIDE_ARROW_DOWN_CIRCLE}/>
                            </button>
                            <label>{ "esto" }</label>
                        </a>
                    </div>
            </article>
        </section>
    }
}
