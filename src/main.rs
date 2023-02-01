#![allow(clippy::needless_return)]

use crate::components::app_component::App;

mod algorithms;
mod components;
mod models;
mod constraints;

fn main() {
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
