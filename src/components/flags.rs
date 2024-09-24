use std::rc::Rc;
use yew::{html, Callback, Html};

use super::lang::Language;

pub fn render_flag(
    lang: Rc<Language>,
    img_src: String,
    set_language: Callback<Rc<Language>>,
    classes: String,
) -> Html {
    html! {
        <a onclick={Callback::from(move |_| set_language.emit(Rc::clone(&lang)))}>
            <img class={format!("flag {}", classes)} src={img_src}/>
        </a>
    }
}
