use strum::IntoEnumIterator;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

use crate::algorithms::PFAlgorithms;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_find_path_clicked: Callback<PFAlgorithms>,
    pub on_reset_board_clicked: Callback<()>,
    pub on_reset_board_visited_clicked: Callback<()>,
}

#[function_component(AlgorithmSelectorComponent)]
pub fn algorithm_selector_component(props: &Props) -> Html {
    let selected_algorithm = use_state(|| PFAlgorithms::NotSelected);

    let on_find_click = {
        let selected_algorithm = selected_algorithm.clone();
        let props = props.clone();
        move |_| {
            props.on_find_path_clicked.emit(*selected_algorithm);
        }
    };

    let on_reset_click = {
        let props = props.clone();
        move |_| {
            props.on_reset_board_clicked.emit(());
        }
    };

    let on_reset_visited_click = {
        let props = props.clone();
        move |_| {
            props.on_reset_board_visited_clicked.emit(());
        }
    };

    html! {
        <div>
            <select class={classes!("selector")}>
            {
                PFAlgorithms::iter()
                    .map(|algorithm| {
                        let onclick = {
                            let selected_algorithm = selected_algorithm.clone();
                            move |_: MouseEvent | {
                                selected_algorithm.set(algorithm)
                            }
                        };

                        return html! {
                            <option {onclick} selected={*selected_algorithm == algorithm}>
                                    {format!("{algorithm}")}
                            </option>
                        };
                    })
                .collect::<Html>()
            }
            </select>

            <button class={classes!("button")} onclick={on_find_click}>{"Find"}</button>
            <button class={classes!("button")} onclick={on_reset_click}>{"Reset board"}</button>
            <button class={classes!("button")} onclick={on_reset_visited_click}>{"Reset path"}</button>
        </div>
    }
}