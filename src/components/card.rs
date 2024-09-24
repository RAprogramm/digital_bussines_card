use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{HtmlElement, TouchEvent};
use yew::{function_component, html, Callback, Classes, Html, NodeRef};

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
    let touch_start_x = Rc::new(RefCell::new(0.0));
    let touch_current_x = Rc::new(RefCell::new(0.0));
    let is_swiping = Rc::new(RefCell::new(false)); // Добавляем флаг для отслеживания свайпа
    let swipe_threshold = 50.0;
    let card_ref = NodeRef::default();

    // Обработчик смены языка
    let set_language = {
        let state = Rc::clone(&state);
        Callback::from(move |new_lang: Rc<Language>| {
            let state_ref = state.borrow_mut();
            state_ref.set(State {
                language: Rc::clone(&new_lang),
                is_rotating: false,
                is_content_visible: true,
            });
        })
    };

    // Начало касания
    let on_touch_start = {
        let touch_start_x = Rc::clone(&touch_start_x);
        let is_swiping = Rc::clone(&is_swiping); // Сбрасываем флаг свайпа
        Callback::from(move |event: TouchEvent| {
            if let Some(touch) = event.touches().get(0) {
                *touch_start_x.borrow_mut() = touch.client_x() as f64;
                *is_swiping.borrow_mut() = false; // Флаг свайпа сбрасывается
            }
        })
    };

    // Движение пальцем по экрану
    let on_touch_move = {
        let touch_start_x = Rc::clone(&touch_start_x);
        let touch_current_x = Rc::clone(&touch_current_x);
        let is_swiping = Rc::clone(&is_swiping); // Устанавливаем флаг свайпа
        let card_ref = card_ref.clone();
        Callback::from(move |event: TouchEvent| {
            if let Some(touch) = event.changed_touches().get(0) {
                let current_x = touch.client_x() as f64;
                *touch_current_x.borrow_mut() = current_x;

                let delta_x = current_x - *touch_start_x.borrow();
                let rotation_angle = delta_x / 3.0;

                if delta_x.abs() > 10.0 {
                    // Если движение значительное, считаем это свайпом
                    *is_swiping.borrow_mut() = true;
                }

                // Обновляем вращение карточки
                if let Some(card) = card_ref.cast::<HtmlElement>() {
                    card.style()
                        .set_property("transform", &format!("rotateY({}deg)", rotation_angle))
                        .unwrap();
                }
            }
        })
    };

    // Завершение касания (свайпа)
    let on_touch_end = {
        let touch_start_x = Rc::clone(&touch_start_x);
        let touch_current_x = Rc::clone(&touch_current_x);
        let is_swiping = Rc::clone(&is_swiping); // Проверяем флаг свайпа
        let state = Rc::clone(&state);
        let card_ref = card_ref.clone();
        Callback::from(move |_| {
            let delta_x = *touch_current_x.borrow() - *touch_start_x.borrow();

            if *is_swiping.borrow() {
                // Только если это был свайп
                // Меняем язык в зависимости от направления свайпа
                if delta_x.abs() > swipe_threshold {
                    let new_language = if delta_x > 0.0 {
                        // Свайп вправо
                        match &*state.borrow().language {
                            Language::English => Rc::new(Language::Korean),
                            Language::Korean => Rc::new(Language::Vietnamese),
                            Language::Russian => Rc::new(Language::English),
                            Language::Vietnamese => Rc::new(Language::Russian),
                        }
                    } else {
                        // Свайп влево
                        match &*state.borrow().language {
                            Language::English => Rc::new(Language::Russian),
                            Language::Korean => Rc::new(Language::English),
                            Language::Russian => Rc::new(Language::Vietnamese),
                            Language::Vietnamese => Rc::new(Language::Korean),
                        }
                    };

                    // Обновляем состояние с новым языком
                    let state_ref = state.borrow_mut();
                    state_ref.set(State {
                        language: Rc::clone(&new_language),
                        is_rotating: false,
                        is_content_visible: true,
                    });

                    // Возвращаем карточку в исходное положение
                    if let Some(card) = card_ref.cast::<HtmlElement>() {
                        card.style()
                            .set_property("transform", "rotateY(0deg)")
                            .unwrap();
                    }
                } else {
                    // Если движение не было достаточно большим для свайпа, возвращаем карточку в исходное положение
                    if let Some(card) = card_ref.cast::<HtmlElement>() {
                        card.style()
                            .set_property("transform", "rotateY(0deg)")
                            .unwrap();
                    }
                }
            }
        })
    };

    // Вывод HTML-кода
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

    // Определяем активный класс для каждого флага
    let korean_flag_classes = if let Language::Korean = *state_ref.language {
        "active-flag"
    } else {
        ""
    };

    let english_flag_classes = if let Language::English = *state_ref.language {
        "active-flag"
    } else {
        ""
    };

    let russian_flag_classes = if let Language::Russian = *state_ref.language {
        "active-flag"
    } else {
        ""
    };

    let vietnamese_flag_classes = if let Language::Vietnamese = *state_ref.language {
        "active-flag"
    } else {
        ""
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
                    { render_flag(Rc::new(Language::Korean), "../images/flags/kr.svg".to_string(), set_language.clone(), korean_flag_classes.to_string()) }
                    { render_flag(Rc::new(Language::English), "../images/flags/us.svg".to_string(), set_language.clone(), english_flag_classes.to_string()) }
                    { render_flag(Rc::new(Language::Russian), "../images/flags/ru.svg".to_string(), set_language.clone(), russian_flag_classes.to_string()) }
                    { render_flag(Rc::new(Language::Vietnamese), "../images/flags/vn.svg".to_string(), set_language.clone(), vietnamese_flag_classes.to_string()) }
                </div>
                <p class="created">{"Created on "}<i class="fa-brands fa-rust"></i>{" by RAprogramm"}</p>
            </div>
        </div>
    }
}
