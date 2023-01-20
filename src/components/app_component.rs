use yew::{function_component, html, Html};

use crate::components::matrix_component::MatrixComponent;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <MatrixComponent />
        </div>
    }
}
