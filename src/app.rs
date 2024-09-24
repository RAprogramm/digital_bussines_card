use yew::{function_component, html, Html};

use crate::components::card::Card;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="app">
            <div class="container">
                <Card />
            </div>
        </div>
    }
}
