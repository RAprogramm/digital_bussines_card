use web_sys::{console, window};
use yew::{function_component, html, Callback, Html};

#[function_component(Social)]
pub fn social() -> Html {
    let navigate_to = |url: &'static str| {
        let url = url.to_string();
        Callback::from(move |_| {
            if let Some(window) = window() {
                if let Err(err) = window.location().set_href(&url) {
                    console::error_1(&format!("Failed to navigate to {}: {:?}", url, err).into());
                }
            } else {
                console::error_1(&"Failed to get window object".into());
            }
        })
    };

    html! {
        <div class="body__profile__icons">
            <a class="linkedin" onclick={navigate_to("https://www.linkedin.com/in/andrei-rozanov-4271a425b/")}>
                <i class="fab fa-linkedin"></i>
            </a>
            <a class="telegram" onclick={navigate_to("https://t.me/Magistr_RA")}>
                <i class="fab fa-telegram"></i>
            </a>
            <a class="google" onclick={navigate_to("mailto:andrey.rozanov.vl@gmail.com")}>
                <i class="fab fa-google"></i>
            </a>
        </div>
    }
}
