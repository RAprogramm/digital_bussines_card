use yew::{function_component, html, Html};

#[function_component(Korean)]
pub fn korean() -> Html {
    let bio_kr = "다재다능한 개발자이자 지식의 영원한 탐구자.";

    html! {
        <div class="body__profile__title">
            <h1>{ "안드레이 로자노프" }</h1>
            <h2>{ "소프트웨어 엔지니어" }</h2>
            <div class="body__profile__bio-pt">
                { bio_kr }
            </div>
        </div>
    }
}
