use std::{cell::RefCell, rc::Rc, sync::Arc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, window, HtmlElement, MouseEvent};
use yew::{html, Callback, Event, Html, NodeRef, UseStateHandle};

use super::lang::Language;
use crate::state::State;

fn flag_button(
    flag_lang: Arc<Language>,
    img_src: String,
    set_language: Callback<Arc<Language>>,
    card_ref: NodeRef,
    classes: String,
) -> Html {
    let flag_lang_clone = Arc::clone(&flag_lang);
    let animation_in_progress = Rc::new(RefCell::new(false));

    let onclick = Callback::from({
        let card_ref = card_ref.clone();
        let flag_lang_clone = flag_lang_clone.clone();
        let animation_in_progress = animation_in_progress.clone();
        move |_: MouseEvent| {
            if let Some(target) = card_ref.cast::<HtmlElement>() {
                if *animation_in_progress.borrow() {
                    console::log_1(&"Animation already in progress. No action taken.".into());
                    return;
                }

                *animation_in_progress.borrow_mut() = true;

                let card_element = Rc::new(RefCell::new(target));

                if let Err(e) = card_element.borrow().class_list().add_1("rotating") {
                    console::error_1(&format!("Failed to add 'rotating' class: {:?}", e).into());
                    return;
                }

                let flag_lang_clone = Arc::clone(&flag_lang_clone);
                let set_language = set_language.clone();
                let animation_in_progress_clone = animation_in_progress.clone();
                let handle = Closure::wrap(Box::new(move || {
                    if *animation_in_progress_clone.borrow() {
                        set_language.emit(flag_lang_clone.clone());
                    }
                }) as Box<dyn FnMut()>);

                if let Some(window) = window() {
                    if let Err(e) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        handle.as_ref().unchecked_ref(),
                        250,
                    ) {
                        console::error_1(&format!("Failed to set timeout: {:?}", e).into());
                        return;
                    }
                } else {
                    console::error_1(&"Failed to get window object.".into());
                    return;
                }

                handle.forget();

                let card_element_clone = card_element.clone();
                let animation_in_progress_clone = animation_in_progress.clone();
                let on_animation_end = Closure::wrap(Box::new(move |_: Event| {
                    if let Err(e) = card_element_clone
                        .borrow()
                        .class_list()
                        .remove_1("rotating")
                    {
                        console::error_1(
                            &format!("Failed to remove rotating class: {:?}", e).into(),
                        );
                    }

                    *animation_in_progress_clone.borrow_mut() = false;
                }) as Box<dyn FnMut(_)>);

                card_element
                    .borrow()
                    .remove_event_listener_with_callback(
                        "animationend",
                        on_animation_end.as_ref().unchecked_ref(),
                    )
                    .ok();

                if let Err(e) = card_element.borrow().add_event_listener_with_callback(
                    "animationend",
                    on_animation_end.as_ref().unchecked_ref(),
                ) {
                    console::error_1(
                        &format!("Failed to add animationend listener: {:?}", e).into(),
                    );
                }

                on_animation_end.forget();
            } else {
                console::error_1(&"Failed to find card element.".into());
            }
        }
    });

    html! {
        <a onclick={onclick}>
            <img class={format!("flag {}", classes)} src={img_src}/>
        </a>
    }
}

pub fn render_flags(
    state: &UseStateHandle<State>,
    set_language: Callback<Arc<Language>>,
    card_ref: NodeRef,
) -> Html {
    html! {
        <>
            { flag_button(Arc::new(Language::Korean), "../images/flags/kr.svg".to_string(), set_language.clone(), card_ref.clone(), get_flag_class(&state.language, &Language::Korean)) }
            { flag_button(Arc::new(Language::English), "../images/flags/us.svg".to_string(), set_language.clone(), card_ref.clone(), get_flag_class(&state.language, &Language::English)) }
            { flag_button(Arc::new(Language::Russian), "../images/flags/ru.svg".to_string(), set_language.clone(), card_ref.clone(), get_flag_class(&state.language, &Language::Russian)) }
            { flag_button(Arc::new(Language::Vietnamese), "../images/flags/vn.svg".to_string(), set_language.clone(), card_ref.clone(), get_flag_class(&state.language, &Language::Vietnamese)) }
        </>
    }
}

fn get_flag_class(current_language: &Arc<Language>, language: &Language) -> String {
    if **current_language == *language {
        "active-flag".to_string()
    } else {
        "".to_string()
    }
}
