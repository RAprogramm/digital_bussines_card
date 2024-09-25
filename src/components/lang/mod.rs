use std::sync::{Arc, OnceLock};

use crate::touch_handler::SwipeDirection;

pub mod english;
pub mod korean;
pub mod russian;
pub mod vietnam;

#[derive(Debug, PartialEq)]
pub enum Language {
    Korean,
    English,
    Russian,
    Vietnamese,
}

static LANGUAGES: OnceLock<Vec<Arc<Language>>> = OnceLock::new();

fn init_languages() -> &'static Vec<Arc<Language>> {
    LANGUAGES.get_or_init(|| {
        vec![
            Arc::new(Language::Korean),
            Arc::new(Language::English),
            Arc::new(Language::Russian),
            Arc::new(Language::Vietnamese),
        ]
    })
}

pub fn get_next_language(current: &Arc<Language>, direction: SwipeDirection) -> Arc<Language> {
    let languages = init_languages();
    if let Some(current_index) = languages.iter().position(|lang| Arc::ptr_eq(lang, current)) {
        let next_index = get_next_language_index(current_index, direction);
        Arc::clone(&languages[next_index])
    } else {
        Arc::clone(&languages[0])
    }
}

fn get_next_language_index(current_index: usize, direction: SwipeDirection) -> usize {
    if let Some(languages) = LANGUAGES.get() {
        match direction {
            SwipeDirection::Right => (current_index + 1) % languages.len(),
            SwipeDirection::Left => {
                if current_index == 0 {
                    languages.len() - 1
                } else {
                    current_index - 1
                }
            }
        }
    } else {
        0
    }
}
