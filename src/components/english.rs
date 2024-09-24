use yew::{function_component, html, Html};

#[function_component(English)]
pub fn english() -> Html {
    let bio_en = "Multitasking developer and eternal seeker of knowledge.";

    html! {
        <div class="body__profile__title">
            <h1>{ "Andrei Rozanov" }</h1>
            <h2>{ "Software Engineer" }</h2>
            <div class="body__profile__bio-en">
                { bio_en }
            </div>
        </div>
    }
}
