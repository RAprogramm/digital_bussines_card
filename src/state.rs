use yew::use_state;

use crate::components::lang::Language;

#[derive(Clone)]
pub struct State {
    pub language: Language,
    pub is_rotating: bool,
    pub is_content_visible: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            language: Language::English,
            is_rotating: false,
            is_content_visible: true,
        }
    }
}

pub fn use_card_state() -> impl yew::Hook<Output = yew::UseStateHandle<State>> {
    use_state(State::default)
}
