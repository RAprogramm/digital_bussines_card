use web_sys::{
    js_sys::Function,
    wasm_bindgen::{prelude::Closure, JsCast},
    window,
};
use yew::{function_component, html, use_state, Callback, Classes, Html};

use crate::components::{
    english::English, korean::Korean, photo::Photo, russian::Russian, social::Social,
};

#[derive(Clone, PartialEq)]
enum Language {
    English,
    Korean,
    Russian,
}

#[function_component(Card)]
pub fn card() -> Html {
    let current_language = use_state(|| Language::English);
    let is_rotating = use_state(|| false);
    let is_content_visible = use_state(|| true); // Состояние для текста и кнопок

    let set_language = {
        let current_language = current_language.clone();
        let is_rotating = is_rotating.clone();
        let is_content_visible = is_content_visible.clone();
        Callback::from(move |new_lang: Language| {
            is_content_visible.set(false); // Сначала скрыть контент (текст + кнопки)
            let current_language = current_language.clone();
            let is_rotating = is_rotating.clone();
            let is_content_visible = is_content_visible.clone();

            // Установить задержку перед началом анимации вращения
            let lang_change = move || {
                let current_language = current_language.clone();
                let is_rotating = is_rotating.clone();
                let is_content_visible = is_content_visible.clone();

                current_language.set(new_lang.clone());
                is_rotating.set(true);

                let closure = Closure::wrap(Box::new(move || {
                    is_content_visible.set(true); // Показать новый контент после вращения
                    is_rotating.set(false);
                }) as Box<dyn FnMut()>);

                window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref::<Function>(),
                        1000, // Задержка в 1 секунду перед показом текста и кнопок
                    )
                    .unwrap();
                closure.forget(); // Не забываем "забыть" замыкание, чтобы избежать его удаления.
            };

            let closure = Closure::wrap(Box::new(lang_change) as Box<dyn FnMut()>);

            window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref::<Function>(),
                    500, // Задержка в 0.5 секунды перед вращением
                )
                .unwrap();
            closure.forget(); // "Забываем" это замыкание
        })
    };

    let github_url = "https://www.github.com/RAprogramm";
    let resume_url =
        "https://drive.google.com/file/d/1--CHUjt7L6gjNzVYx_3y_IL4iWWUPGum/view?usp=drive_link";

    // Тексты кнопок меняются в зависимости от языка
    let github_text = match &*current_language {
        Language::Korean => "GitHub에서 포트폴리오 보기",
        Language::Russian => "Профиль на GitHub",
        Language::English => "View portfolio on GitHub",
    };

    let resume_text = match &*current_language {
        Language::Korean => "이력서 다운로드",
        Language::Russian => "Скачать резюме",
        Language::English => "Download CV",
    };

    let container_classes = if *is_rotating {
        Classes::from("body__container rotating")
    } else {
        Classes::from("body__container")
    };

    // Общий класс для скрытия и текста, и кнопок
    let content_classes = if *is_content_visible {
        Classes::from("content-visible")
    } else {
        Classes::from("content-hidden")
    };

    html! {
        <div class={container_classes}>
            <div class="body__profile">
                <Photo />
                <div class={content_classes}>
                    <div class="body__profile__button-area">
                        <div>
                            { match &*current_language {
                                Language::Korean => html! { <Korean /> },
                                Language::English => html! { <English /> },
                                Language::Russian => html! { <Russian /> },
                            }}
                        </div>
                        <Social />
                        <button class="body__profile__button github" onclick={Callback::from(move |_| { web_sys::window().unwrap().location().set_href(github_url).unwrap(); })}>
                            { github_text }
                        </button>
                        <button class="body__profile__button cv" onclick={Callback::from(move |_| { web_sys::window().unwrap().location().set_href(resume_url).unwrap(); })}>
                            { resume_text }
                        </button>
                    </div>
                </div>
                <div class="body__profile__translate">
                    <a onclick={Callback::from({
                        let set_language = set_language.clone();
                        move |_| set_language.emit(Language::Korean)
                    })}>
                        <img class="flag" src="../images/flags/kr.svg"/>
                    </a>
                    <a onclick={Callback::from({
                        let set_language = set_language.clone();
                        move |_| set_language.emit(Language::English)
                    })}>
                        <img class="flag" src="../images/flags/us.svg"/>
                    </a>
                    <a onclick={Callback::from({
                        let set_language = set_language.clone();
                        move |_| set_language.emit(Language::Russian)
                    })}>
                        <img class="flag" src="../images/flags/ru.svg"/>
                    </a>
                </div>
                <p class="created">{"Created on "} <i class="fa-brands fa-rust"></i></p>
            </div>
        </div>
    }
}
