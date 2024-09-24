use yew::{function_component, html, Html};

#[function_component(Russian)]
pub fn russian() -> Html {
    let bio_ru = "Многозадачный разработчик и вечный искатель знаний.";

    html! {
        <div class="body__profile__title">
            <h1>{ "Андрей Розанов" }</h1>
            <h2>{ "Инженер-программист" }</h2>
            <div class="body__profile__bio-ru">
                { bio_ru }
            </div>
        </div>
    }
}
