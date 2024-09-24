use yew::{html, Callback, Html};

use super::lang::Language;

pub fn render_flag(lang: Language, img_src: String, set_language: Callback<Language>) -> Html {
    html! {
        <a onclick={Callback::from(move |_| set_language.emit(lang.clone()))}>
            <img class="flag" src={img_src}/>
        </a>
    }
}
