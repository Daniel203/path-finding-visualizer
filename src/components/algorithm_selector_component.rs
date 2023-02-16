use strum::IntoEnumIterator;
use web_sys::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

use crate::models::notification::Notification;
use crate::models::notification::SeverityLevel;
use crate::{
    algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms},
    components::notification_component::NotificationComponent,
};

#[derive(Properties, PartialEq, Clone)]
pub struct AlgorithmSelectorProps {
    pub on_generate_maze_clicked: Callback<MGAlgorithms>,
    pub on_find_path_clicked: Callback<PFAlgorithms>,
    pub on_reset_board_clicked: Callback<()>,
    pub on_reset_visited_clicked: Callback<()>,
}

#[function_component(AlgorithmSelectorComponent)]
pub fn algorithm_selector_component(props: &AlgorithmSelectorProps) -> Html {
    let selected_pf_algorithm = use_state(|| PFAlgorithms::NotSelected);
    let selected_mg_algorithm = use_state(|| MGAlgorithms::NotSelected);

    let snackbar_value = use_state(Notification::default);

    let on_find_click = {
        let selected_algorithm = selected_pf_algorithm.clone();
        let snackbar_value = snackbar_value.clone();
        let props = props.clone();

        move |_| {
            if *selected_algorithm != PFAlgorithms::NotSelected {
                props.on_find_path_clicked.emit(*selected_algorithm);
            } else {
                snackbar_value.set(Notification::new(
                    String::from("Please select a path finding algorithm"),
                    SeverityLevel::Warning,
                ));
            }
        }
    };

    let on_generate_maze_click = {
        let selected_algorithm = selected_mg_algorithm.clone();
        let snackbar_value = snackbar_value.clone();
        let props = props.clone();

        move |_| {
            if *selected_algorithm != MGAlgorithms::NotSelected {
                props.on_generate_maze_clicked.emit(*selected_algorithm);
            } else {
                snackbar_value.set(Notification::new(
                    String::from("Please select a maze generation algorithm"),
                    SeverityLevel::Warning,
                ));
            }
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
            props.on_reset_visited_clicked.emit(());
        }
    };

    html! {
        <div>
            <select class={classes!("btn")}>
            {
                MGAlgorithms::iter()
                    .map(|algorithm| {
                        let onclick = {
                            let selected_algorithm = selected_mg_algorithm.clone();
                            move |_: MouseEvent | {
                                selected_algorithm.set(algorithm)
                            }
                        };

                        return html! {
                            <option
                                {onclick}
                                selected={*selected_mg_algorithm == algorithm}
                                disabled={algorithm == MGAlgorithms::NotSelected}
                            >
                                {format!("{algorithm}")}
                            </option>
                        };
                    })
                .collect::<Html>()
            }
            </select>
            <button class={classes!("btn", "green")} onclick={on_generate_maze_click}>{"Generate Maze"}</button>

            <span class={classes!("v-separator-m")} />

            <select class={classes!("btn")}>
            {
                PFAlgorithms::iter()
                    .map(|algorithm| {
                        let onclick = {
                            let selected_algorithm = selected_pf_algorithm.clone();
                            move |_: MouseEvent | {
                                selected_algorithm.set(algorithm)
                            }
                        };

                        return html! {
                            <option
                                {onclick}
                                selected={*selected_pf_algorithm == algorithm}
                                disabled={algorithm == PFAlgorithms::NotSelected}
                            >
                                {format!("{algorithm}")}
                            </option>
                        };
                    })
                .collect::<Html>()
            }
            </select>
            <button class={classes!("btn", "green")} onclick={on_find_click}>{"Find path"}</button>

            <span class={classes!("v-separator-m")} />

            <button class={classes!("btn", "orange")} onclick={on_reset_visited_click}>{"Reset Visited Cells"}</button>
            <button class={classes!("btn", "red")} onclick={on_reset_click}>{"Reset Board"}</button>

            if !snackbar_value.msg.is_empty() {
                <NotificationComponent notification={(*snackbar_value).clone()} />
            }
        </div>
    }
}
