use gloo_console::log;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use yew::{function_component, html, use_state, Callback, Event, Html, Properties, use_node_ref, use_effect_with_deps};
use web_sys::HtmlSelectElement;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_find_path_clicked: Callback<PFAlgorithms>,
}

#[derive(Debug, Copy, Clone, EnumIter, Eq, PartialEq)]
pub enum PFAlgorithms {
    BFS,
    DFS,
}

#[function_component(AlgorithmSelectorComponent)]
pub fn algorithm_selector_component(props: &Props) -> Html {
    let selected_algorithm = use_state(|| PFAlgorithms::BFS);

    let onclick = {
        let selected_algorithm = selected_algorithm.clone();
        let props = props.clone();

        move |_| {
            props
                .on_find_path_clicked
                .emit((*selected_algorithm).clone());
        }
    };

    //let onchange = {
        //move |e: Event| {
            //log!(format!("{:?}", e.value_of()));
        //}
    //};

     let select_node_ref = use_node_ref();

    {
        let select_node_ref = select_node_ref.clone();

        use_effect_with_deps(
            move |selection| {
                log!(format!("{selection:?}"));
                if let Some(element) = select_node_ref.cast::<HtmlSelectElement>() {
                    element.set_value(&format!("{selection:?}"));
                }
                || {}
            },
            *selected_algorithm,
        )
    };

    html! {
        <div>
            <div>
                <label>{"Choose an algorithm: "}</label>
                <select ref={select_node_ref}>
                {
                    PFAlgorithms::iter()
                        .map(|algorithm| {
                            return html! {
                                //<option selected={algorithm == (*selected_algorithm).clone()}>
                                <option>
                                    {format!("{:?}", algorithm)}
                                </option>
                            }
                        })
                    .collect::<Html>()
                }
                </select>
            </div>

            <button {onclick}>{"Find"}</button>
        </div>
    }
}
