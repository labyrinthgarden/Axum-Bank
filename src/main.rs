use yew::prelude::*;

#[component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1>{ "Hola desde Yew!" }</h1>
            <p>{ "Contador: " }{ *counter }</p>
            <button {onclick}>{ "+1" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
