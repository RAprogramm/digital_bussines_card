use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{
    console,
    js_sys::Function,
    wasm_bindgen::{prelude::Closure, JsCast},
    window,
};
use yew::{function_component, html, Callback, Classes, Html};

use crate::{
    components::{
        buttons::{render_github_button, render_resume_button},
        flags::render_flag,
        lang::{english::English, korean::Korean, russian::Russian, vietnam::Vietnamese, Language},
        photo::Photo,
        social::Social,
    },
    state::{use_card_state, State},
};

#[function_component(Card)]
pub fn card() -> Html {
    let state = Rc::new(RefCell::new(use_card_state()));

    let set_language = {
        let state = Rc::clone(&state);
        Callback::from(move |new_lang: Rc<Language>| {
            let state = Rc::clone(&state);

            let state_ref = state.borrow_mut();
            state_ref.set(State {
                language: Rc::clone(&state_ref.language), // Оставляем текущий язык
                is_rotating: true,                        // Сначала запускаем вращение
                is_content_visible: false,
            });

            let rotate = {
                let state = Rc::clone(&state);
                let new_lang = Rc::clone(&new_lang);
                Closure::wrap(Box::new(move || {
                    let state_ref = state.borrow_mut();
                    state_ref.set(State {
                        language: Rc::clone(&new_lang), // После завершения вращения меняем язык
                        is_rotating: false,
                        is_content_visible: true, // Показываем контент с новым языком
                    });
                }) as Box<dyn FnMut()>)
            };

            if let Some(window) = window() {
                if let Err(err) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    rotate.as_ref().unchecked_ref::<Function>(),
                    1000, // Время вращения
                ) {
                    console::error_1(
                        &format!("Failed to set timeout for rotation: {:?}", err).into(),
                    );
                }
            } else {
                console::error_1(&"Failed to get window object".into());
            }

            rotate.forget();
        })
    };

    let github_url = "https://www.github.com/RAprogramm";
    let resume_url =
        "https://drive.google.com/file/d/1--CHUjt7L6gjNzVYx_3y_IL4iWWUPGum/view?usp=drive_link";

    let state_ref = state.borrow();
    let github_text = match &*state_ref.language {
        Language::Korean => "GitHub에서 포트폴리오 보기",
        Language::Russian => "Профиль на GitHub",
        Language::English => "View portfolio on GitHub",
        Language::Vietnamese => "Xem hồ sơ trên GitHub",
    };

    let resume_text = match &*state_ref.language {
        Language::Korean => "이력서 다운로드",
        Language::Russian => "Скачать резюме",
        Language::English => "Download CV",
        Language::Vietnamese => "Tải xuống CV",
    };

    let container_classes = if state_ref.is_rotating {
        Classes::from("body__container rotating")
    } else {
        Classes::from("body__container")
    };

    let content_classes = if state_ref.is_content_visible {
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
                            { match &*state_ref.language {
                                Language::Korean => html! { <Korean /> },
                                Language::English => html! { <English /> },
                                Language::Russian => html! { <Russian /> },
                                Language::Vietnamese => html! { <Vietnamese /> },
                            }}
                        </div>
                        <Social />
                        { render_github_button(github_url.to_string(), github_text.to_string()) }
                        { render_resume_button(resume_url.to_string(), resume_text.to_string()) }
                    </div>
                </div>
                <div class="body__profile__translate">
                    { render_flag(Rc::new(Language::Korean), "../images/flags/kr.svg".to_string(), set_language.clone()) }
                    { render_flag(Rc::new(Language::English), "../images/flags/us.svg".to_string(), set_language.clone()) }
                    { render_flag(Rc::new(Language::Russian), "../images/flags/ru.svg".to_string(), set_language.clone()) }
                    { render_flag(Rc::new(Language::Vietnamese), "../images/flags/vn.svg".to_string(), set_language.clone()) }
                </div>
                <p class="created">{"Created on "}<i class="fa-brands fa-rust"></i>{" by RAprogramm"}</p>
            </div>
        </div>
    }
}
