use web_sys::console;
use yew::{html, Callback, Html};

pub fn render_github_button(github_url: String, text: String) -> Html {
    html! {
        <button class="body__profile__button github" onclick={Callback::from(move |_| {
            if let Some(window) = web_sys::window() {
                if let Err(err) = window.location().set_href(&github_url) {
                    console::error_1(&format!("Failed to navigate to GitHub: {:?}", err).into());
                }
            } else {
                console::error_1(&"Failed to get window object".into());
            }
        })}>
            { text }
        </button>
    }
}

pub fn render_resume_button(resume_url: String, text: String) -> Html {
    html! {
        <button class="body__profile__button cv" onclick={Callback::from(move |_| {
            if let Some(window) = web_sys::window() {
                if let Err(err) = window.location().set_href(&resume_url) {
                    console::error_1(&format!("Failed to navigate to resume: {:?}", err).into());
                }
            } else {
                console::error_1(&"Failed to get window object".into());
            }
        })}>
            { text }
        </button>
    }
}
