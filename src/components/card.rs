use std::sync::Arc;

use yew::{function_component, html, use_state, Callback, Classes, Html, NodeRef};

use crate::{
    components::{
        buttons::{render_github_button, render_resume_button},
        flags::render_flags,
        lang::{english::English, korean::Korean, russian::Russian, vietnam::Vietnamese, Language},
        photo::Photo,
        social::Social,
    },
    state::{use_card_state, State},
    touch_handler,
};

#[function_component(Card)]
pub fn card() -> Html {
    let state = use_card_state();
    let touch_start_x = use_state(|| 0.0);
    let touch_current_x = use_state(|| 0.0);
    let is_swiping = use_state(|| false);
    let swipe_threshold = 250.0;
    let card_ref = NodeRef::default();

    let set_language = {
        let state = state.clone();
        Callback::from(move |new_lang: Arc<Language>| {
            state.set(State {
                language: Arc::clone(&new_lang),
                is_rotating: false,
                is_content_visible: true,
            });
        })
    };

    let on_touch_start = touch_handler::on_touch_start(touch_start_x.clone(), is_swiping.clone());
    let on_touch_move = touch_handler::on_touch_move(
        touch_start_x.clone(),
        touch_current_x.clone(),
        is_swiping.clone(),
        card_ref.clone(),
    );
    let on_touch_end = touch_handler::on_touch_end(
        touch_start_x.clone(),
        touch_current_x.clone(),
        is_swiping.clone(),
        state.clone(),
        card_ref.clone(),
        swipe_threshold,
    );

    let github_url = "https://www.github.com/RAprogramm";
    let resume_url =
        "https://drive.google.com/file/d/1--CHUjt7L6gjNzVYx_3y_IL4iWWUPGum/view?usp=drive_link";

    let github_text = match &*state.language {
        Language::Korean => "GitHub에서 포트폴리오 보기",
        Language::Russian => "Профиль на GitHub",
        Language::English => "View portfolio on GitHub",
        Language::Vietnamese => "Xem hồ sơ trên GitHub",
    };

    let resume_text = match &*state.language {
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
        <div ref={card_ref.clone()} class={container_classes}
            ontouchstart={on_touch_start.clone()}
            ontouchmove={on_touch_move.clone()}
            ontouchend={on_touch_end.clone()}>
            <div class="body__profile">
                <Photo />
                <div class={content_classes}>
                    <div class="body__profile__button-area">
                        <div>
                            { match &*state.language {
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
                    { render_flags(&state, set_language.clone(), card_ref.clone()) }
                </div>
                <p class="created">{"Created on "}<i class="fa-brands fa-rust"></i>{" by RAprogramm"}</p>
            </div>
        </div>
    }
}
