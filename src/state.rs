use std::sync::Arc;

use yew::{use_state, Hook, UseStateHandle};

use crate::components::lang::Language;

pub struct State {
    pub language: Arc<Language>,
    pub is_rotating: bool,
    pub is_content_visible: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            language: Arc::new(Language::English),
            is_rotating: false,
            is_content_visible: true,
        }
    }
}

pub fn use_card_state() -> impl Hook<Output = UseStateHandle<State>> {
    use_state(State::default)
}
