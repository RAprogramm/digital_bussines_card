use std::sync::Arc;

use web_sys::{console, HtmlElement, TouchEvent};
use yew::{Callback, NodeRef, UseStateHandle};

use crate::{components::lang::get_next_language, state::State};

pub enum SwipeDirection {
    Left,
    Right,
}

pub fn on_touch_start(
    touch_start_x: UseStateHandle<f64>,
    is_swiping: UseStateHandle<bool>,
) -> Callback<TouchEvent> {
    Callback::from(move |event: TouchEvent| {
        if let Some(touch) = event.touches().get(0) {
            touch_start_x.set(touch.client_x() as f64);
            is_swiping.set(false);
        }
    })
}

pub fn on_touch_move(
    touch_start_x: UseStateHandle<f64>,
    touch_current_x: UseStateHandle<f64>,
    is_swiping: UseStateHandle<bool>,
    card_ref: NodeRef,
) -> Callback<TouchEvent> {
    Callback::from(move |event: TouchEvent| {
        if let Some(touch) = event.changed_touches().get(0) {
            let current_x = touch.client_x() as f64;
            touch_current_x.set(current_x);

            let delta_x = current_x - *touch_start_x;
            let rotation_angle = delta_x / 3.0;

            if delta_x.abs() > 10.0 {
                is_swiping.set(true);
            }

            if let Some(card) = card_ref.cast::<HtmlElement>() {
                if let Err(err) = card
                    .style()
                    .set_property("transform", &format!("rotateY({}deg)", rotation_angle))
                {
                    web_sys::console::error_1(
                        &format!("Failed to set transform property: {:?}", err).into(),
                    );
                }
            }
        }
    })
}

pub fn on_touch_end(
    touch_start_x: UseStateHandle<f64>,
    touch_current_x: UseStateHandle<f64>,
    is_swiping: UseStateHandle<bool>,
    state: UseStateHandle<State>,
    card_ref: NodeRef,
    swipe_threshold: f64,
) -> Callback<TouchEvent> {
    Callback::from(move |_| {
        let delta_x = *touch_current_x - *touch_start_x;

        if *is_swiping {
            if delta_x.abs() > swipe_threshold {
                let new_language = if delta_x > 0.0 {
                    get_next_language(&state.language, SwipeDirection::Left)
                } else {
                    get_next_language(&state.language, SwipeDirection::Right)
                };

                state.set(State {
                    language: Arc::clone(&new_language),
                    is_rotating: false,
                    is_content_visible: true,
                });

                if let Some(card) = card_ref.cast::<HtmlElement>() {
                    if let Err(err) = card.style().set_property("transform", "rotateY(0deg)") {
                        console::error_1(
                            &format!("Failed to reset transform property: {:?}", err).into(),
                        );
                    }
                }
            } else if let Some(card) = card_ref.cast::<HtmlElement>() {
                if let Err(err) = card.style().set_property("transform", "rotateY(0deg)") {
                    console::error_1(
                        &format!("Failed to reset transform property: {:?}", err).into(),
                    );
                }
            }
        }
    })
}
