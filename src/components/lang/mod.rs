use std::sync::{Arc, OnceLock};

use crate::touch_handler::SwipeDirection;

pub mod english;
pub mod korean;
pub mod russian;
pub mod vietnam;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Korean,
    English,
    Russian,
    Vietnamese,
}

static LANGUAGES: OnceLock<[Arc<Language>; 4]> = OnceLock::new();

fn init_languages() -> &'static [Arc<Language>; 4] {
    LANGUAGES.get_or_init(|| {
        [
            Arc::new(Language::Korean),
            Arc::new(Language::English),
            Arc::new(Language::Russian),
            Arc::new(Language::Vietnamese),
        ]
    })
}

fn language_index(language: Language) -> usize {
    match language {
        Language::Korean => 0,
        Language::English => 1,
        Language::Russian => 2,
        Language::Vietnamese => 3,
    }
}

/// Returns the canonical shared [`Arc`] handle for the provided [`Language`].
///
/// # Examples
/// ```
/// use business_card::components::lang::{language_handle, Language};
///
/// let english = language_handle(Language::English);
/// assert_eq!(*english, Language::English);
/// ```
pub fn language_handle(language: Language) -> Arc<Language> {
    let languages = init_languages();
    Arc::clone(&languages[language_index(language)])
}

/// Finds the next language handle in the swipe direction relative to the current selection.
///
/// # Examples
/// ```
/// use business_card::components::lang::{get_next_language, language_handle, Language};
/// use business_card::touch_handler::SwipeDirection;
///
/// let english = language_handle(Language::English);
/// let next = get_next_language(&english, SwipeDirection::Right);
/// assert_eq!(*next, Language::Russian);
/// ```
pub fn get_next_language(current: &Arc<Language>, direction: SwipeDirection) -> Arc<Language> {
    let languages = init_languages();
    let current_index = language_index(**current);
    let next_index = get_next_language_index(current_index, languages.len(), direction);
    Arc::clone(&languages[next_index])
}

fn get_next_language_index(
    current_index: usize,
    total_languages: usize,
    direction: SwipeDirection,
) -> usize {
    if total_languages == 0 {
        return 0;
    }

    match direction {
        SwipeDirection::Right => (current_index + 1) % total_languages,
        SwipeDirection::Left => {
            if current_index == 0 {
                total_languages - 1
            } else {
                current_index - 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{state::State, touch_handler::SwipeDirection};

    #[test]
    fn default_state_uses_canonical_english_language() {
        let state = State::default();
        let english = language_handle(Language::English);

        assert!(Arc::ptr_eq(&state.language, &english));
    }

    #[test]
    fn swipes_from_english_visit_expected_neighbors() {
        let english = language_handle(Language::English);

        let left_neighbor = get_next_language(&english, SwipeDirection::Left);
        assert_eq!(*left_neighbor, Language::Korean);
        assert!(Arc::ptr_eq(
            &left_neighbor,
            &language_handle(Language::Korean)
        ));

        let right_neighbor = get_next_language(&english, SwipeDirection::Right);
        assert_eq!(*right_neighbor, Language::Russian);
        assert!(Arc::ptr_eq(
            &right_neighbor,
            &language_handle(Language::Russian)
        ));
    }
}
