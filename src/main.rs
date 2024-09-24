#![allow(non_snake_case)]

use wasm_logger::{init, Config};
use yew::Renderer;

use app::App;

// mod api;
mod app;
mod components;
// mod layouts;
// mod pages;
// mod router;
// mod store;

fn main() {
    init(Config::default());
    Renderer::<App>::new().render();
}
