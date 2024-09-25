use std::sync::Arc;

use yew::{html, Callback, Html, UseStateHandle};

use crate::state::State;

use super::lang::Language;

fn flag_button(
    lang: Arc<Language>,
    img_src: String,
    set_language: Callback<Arc<Language>>,
    classes: String,
) -> Html {
    let lang_clone = Arc::clone(&lang);
    html! {
        <a onclick={Callback::from(move |_| set_language.emit(lang_clone.clone()))}>
            <img class={format!("flag {}", classes)} src={img_src}/>
        </a>
    }
}

pub fn render_flags(state: &UseStateHandle<State>, set_language: Callback<Arc<Language>>) -> Html {
    html! {
        <>
            { flag_button(Arc::clone(&state.language), "../images/flags/kr.svg".to_string(), set_language.clone(), get_flag_class(&state.language, &Language::Korean)) }
            { flag_button(Arc::clone(&state.language), "../images/flags/us.svg".to_string(), set_language.clone(), get_flag_class(&state.language, &Language::English)) }
            { flag_button(Arc::clone(&state.language), "../images/flags/ru.svg".to_string(), set_language.clone(), get_flag_class(&state.language, &Language::Russian)) }
            { flag_button(Arc::clone(&state.language), "../images/flags/vn.svg".to_string(), set_language.clone(), get_flag_class(&state.language, &Language::Vietnamese)) }
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
