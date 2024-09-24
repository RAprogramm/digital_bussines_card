use yew::{function_component, html, Html};

#[function_component(Photo)]
pub fn photo() -> Html {
    html! {
        <div>
            <div class="body__profile__photo2 bounce-enter-active">
                <img src="../images/face2.jpg" alt="in Korea" />
            </div>
        </div>
    }
}
