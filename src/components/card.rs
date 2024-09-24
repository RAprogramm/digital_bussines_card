use crate::components::english::English;
use crate::components::korean::Korean;
use crate::components::photo::Photo;
use crate::components::russian::Russian;
use crate::components::social::Social;
use yew::{function_component, html, use_state, Callback, Html};

#[function_component(Card)]
pub fn card() -> Html {
    let current_language = use_state(|| "en".to_string());

    let set_language = {
        let current_language = current_language.clone();
        Callback::from(move |new_lang: String| {
            current_language.set(new_lang);
        })
    };

    let github_url = "https://www.github.com/RAprogramm";
    let resume_url =
        "https://drive.google.com/file/d/1--CHUjt7L6gjNzVYx_3y_IL4iWWUPGum/view?usp=drive_link";

    html! {
        <div class="body__container">
            <div class="body__profile">
                <Photo />
                <div class="body__profile__button-area">
                    if *current_language == "kr" {
                        <Korean />
                    } else if *current_language == "en" {
                        <English />
                    } else if *current_language == "ru" {
                        <Russian />
                    }
                    <Social />
                    <button class="body__profile__button github" onclick={Callback::from(move |_| { web_sys::window().unwrap().location().set_href(github_url).unwrap(); })}>
                        { if *current_language == "kr" { "GitHub에서 포트폴리오 보기" } else if *current_language == "ru" { "Профиль на GitHub" } else { "View portfolio on GitHub" } }
                    </button>
                    <button class="body__profile__button cv" onclick={Callback::from(move |_| { web_sys::window().unwrap().location().set_href(resume_url).unwrap(); })}>
                        { if *current_language == "kr" { "이력서 다운로드" } else if *current_language == "ru" { "Скачать резюме" } else { "Download CV" } }
                    </button>
                </div>
                <div class="body__profile__translate">
                    <a onclick={Callback::from({
                        let set_language = set_language.clone();
                        move |_| set_language.emit("kr".to_string())
                    })}>
                        <img class="flag" src="../images/flags/kr.svg"/>
                    </a>
                    <a onclick={Callback::from({
                        let set_language = set_language.clone();
                        move |_| set_language.emit("en".to_string())
                    })}>
                        <img class="flag" src="../images/flags/us.svg"/>
                    </a>
                    <a onclick={Callback::from({
                        let set_language = set_language.clone();
                        move |_| set_language.emit("ru".to_string())
                    })}>
                        <img class="flag" src="../images/flags/ru.svg"/>
                    </a>
                </div>

                <p class="created">{"Created on "} <i class="fa-brands fa-rust"></i></p>
            </div>
        </div>
    }
}
