use wasm_logger::{init, Config};
use yew::Renderer;

use app::App;

mod app;
mod components;
mod state;
mod touch_handler;

fn main() {
    init(Config::default());
    Renderer::<App>::new().render();
}
