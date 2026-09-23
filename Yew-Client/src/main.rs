mod api;
mod app;
mod models;
mod notice;
mod pages;
mod route;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
