#![allow(clippy::needless_return)]

use crate::components::custom_components::app_component::App;

mod algorithms;
mod components;
mod constraints;
mod models;

fn main() {
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
