use yew::{function_component, html, Html};

#[function_component(Vietnamese)]
pub fn vietnamese() -> Html {
    let bio_vn = "Nhà phát triển đa nhiệm và người tìm kiếm kiến thức vĩnh cửu.";

    html! {
        <div class="body__profile__title">
            <h1>{ "Andrei Rozanov" }</h1>
            <h2>{ "Kỹ sư phần mềm" }</h2>
            <div class="body__profile__bio-vn">
                { bio_vn }
            </div>
        </div>
    }
}
