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
    let state = use_card_state();

    let set_language = {
        let state = state.clone();
        Callback::from(move |new_lang: Language| {
            let state = state.clone();
            let new_lang = new_lang.clone();

            state.set(State {
                language: state.language.clone(),
                is_rotating: false,
                is_content_visible: false,
            });

            let rotate = {
                let state = state.clone();
                let new_lang = new_lang.clone();
                Closure::wrap(Box::new(move || {
                    state.set(State {
                        language: new_lang.clone(),
                        is_rotating: true,
                        is_content_visible: false,
                    });

                    let show_content = {
                        let state = state.clone();
                        let new_lang = new_lang.clone();
                        Closure::wrap(Box::new(move || {
                            state.set(State {
                                language: new_lang.clone(),
                                is_rotating: false,
                                is_content_visible: true,
                            });
                        }) as Box<dyn FnMut()>)
                    };

                    if let Some(window) = window() {
                        if let Err(err) = window
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                show_content.as_ref().unchecked_ref::<Function>(),
                                1000,
                            )
                        {
                            console::error_1(
                                &format!("Failed to set timeout for content: {:?}", err).into(),
                            );
                        }
                    } else {
                        console::error_1(&"Failed to get window object".into());
                    }

                    show_content.forget();
                }) as Box<dyn FnMut()>)
            };

            if let Some(window) = window() {
                if let Err(err) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    rotate.as_ref().unchecked_ref::<Function>(),
                    500,
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

    let github_text = match state.language {
        Language::Korean => "GitHub에서 포트폴리오 보기",
        Language::Russian => "Профиль на GitHub",
        Language::English => "View portfolio on GitHub",
        Language::Vietnamese => "Xem hồ sơ trên GitHub",
    };

    let resume_text = match state.language {
        Language::Korean => "이력서 다운로드",
        Language::Russian => "Скачать резюме",
        Language::English => "Download CV",
        Language::Vietnamese => "Tải xuống CV",
    };

    let container_classes = if state.is_rotating {
        Classes::from("body__container rotating")
    } else {
        Classes::from("body__container")
    };

    let content_classes = if state.is_content_visible {
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
                            { match state.language {
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
                    { render_flag(Language::Korean, "../images/flags/kr.svg".to_string(), set_language.clone()) }
                    { render_flag(Language::English, "../images/flags/us.svg".to_string(), set_language.clone()) }
                    { render_flag(Language::Russian, "../images/flags/ru.svg".to_string(), set_language.clone()) }
                    { render_flag(Language::Vietnamese, "../images/flags/vn.svg".to_string(), set_language.clone()) }
                </div>
                <p class="created">{"Created on "}<i class="fa-brands fa-rust"></i>{" by RAprogramm"}</p>
            </div>
        </div>
    }
}
