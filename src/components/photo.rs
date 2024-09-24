use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PhotoProps {
    pub korean: bool,
}

#[function_component(Photo)]
pub fn photo(props: &PhotoProps) -> Html {
    html! {
        <div>
            if props.korean {
                <div class="body__profile__photo bounce-enter-active">
                    <img src="../images/face.jpg" alt="Face" />
                </div>
            } else {
                <div class="body__profile__photo2 bounce-enter-active">
                    <img src="../images/face2.jpg" alt="in Korea" />
                </div>
            }
        </div>
    }
}
