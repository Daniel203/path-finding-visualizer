use yew::{function_component, html, Html};

use crate::components::custom_components::algorithm_selector_component::AlgorithmSelectorComponent;
use crate::components::custom_components::matrix_component::MatrixComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <AlgorithmSelectorComponent />
            <MatrixComponent />
        </div>
    }
}
