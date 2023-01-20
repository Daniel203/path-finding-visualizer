use crate::components::app_component::App;

mod algorithms;
mod components;
mod models;

fn main() {
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
