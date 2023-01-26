use gloo_console::log;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use web_sys::{Event, HtmlSelectElement, MouseEvent};
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, use_state, Callback, Html,
    Properties,
};

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

    html! {
        <div>
            <div>
                <label>{"Choose an algorithm: "}</label>
                <select>
                {
                    PFAlgorithms::iter()
                        .map(|algorithm| {
                            let onclick = {
                                let selected_algorithm = selected_algorithm.clone();
                                move |_: MouseEvent | {
                                    selected_algorithm.set(algorithm.clone())
                                }
                            };

                            return html! {
                                <option {onclick} selected={(*selected_algorithm) == algorithm}>
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
