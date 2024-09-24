use yew::{function_component, html, use_state, Callback, Html};

use crate::components::english::English;
use crate::components::korean::Korean;
use crate::components::photo::Photo;
use crate::components::social::Social;

#[function_component(Card)]
pub fn card() -> Html {
    let is_korean = use_state(|| true);

    let toggle_language = {
        let is_korean = is_korean.clone();
        Callback::from(move |_| {
            is_korean.set(!*is_korean);
        })
    };

    let github_url = "https://www.github.com/RAprogramm";
    let resume_url =
        "https://drive.google.com/file/d/1--CHUjt7L6gjNzVYx_3y_IL4iWWUPGum/view?usp=drive_link";

    html! {
        <div class="body__container">
            <div class="body__profile">
                <Photo korean={*is_korean} />
                <div class="body__profile__button-area">
                    if *is_korean {
                        <Korean />
                    } else {
                        <English />
                    }
                    <Social />
                    <button class="body__profile__button github" onclick={Callback::from(move |_| { web_sys::window().unwrap().location().set_href(github_url).unwrap(); })}>
                        { if *is_korean { "GitHub에서 포트폴리오 보기" } else { "View portfolio on GitHub" } }
                    </button>
                    <button class="body__profile__button cv" onclick={Callback::from(move |_| { web_sys::window().unwrap().location().set_href(resume_url).unwrap(); })}>
                        { if *is_korean { "이력서 다운로드" } else { "Download CV" } }
                    </button>
                </div>
                <div class="body__profile__translate">
                    <button onclick={toggle_language}>
                        { if *is_korean { "번역하기 (EN)" } else { "Translate (KR)" } }
                    </button>
                </div>
            </div>
        </div>
    }
}
