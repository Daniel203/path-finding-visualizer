use strum::IntoEnumIterator;
use yew::{classes, function_component, html, use_state, Html};
use yewdux::prelude::use_store;

use crate::components::store::algorithm_selector_state::AlgorithmSelectorState;
use crate::models::notification::Notification;
use crate::models::notification::SeverityLevel;
use crate::{
    algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms},
    components::notification_component::NotificationComponent,
};

#[function_component(AlgorithmSelectorComponent)]
pub fn algorithm_selector_component() -> Html {
    let (algorithm_selector_state, algorithm_selector_dispatch) =
        use_store::<AlgorithmSelectorState>();
    let snackbar_value = use_state(Notification::default);

    let snackbar_value_clone = snackbar_value.clone();
    let on_find_click = algorithm_selector_dispatch.reduce_mut_callback(move |state| {
        if state.selected_pf_algorithm != PFAlgorithms::NotSelected {
            state.find_path_clicked = true;
        } else {
            snackbar_value_clone.set(Notification::new(
                String::from("Please select a path finding algorithm"),
                SeverityLevel::Warning,
            ));
        }
    });

    let snackbar_value_clone = snackbar_value.clone();
    let on_generate_maze_click = algorithm_selector_dispatch.reduce_mut_callback(move |state| {
        if state.selected_mg_algorithm != MGAlgorithms::NotSelected {
            state.generate_maze_clicked = true;
        } else {
            snackbar_value_clone.set(Notification::new(
                String::from("Please select a maze generation algorithm"),
                SeverityLevel::Warning,
            ));
        }
    });

    let on_reset_board_click =
        algorithm_selector_dispatch.reduce_mut_callback(|state| state.reset_board_clicked = true);

    let on_reset_visited_click =
        algorithm_selector_dispatch.reduce_mut_callback(|state| state.reset_visited_clicked = true);

    html! {
        <div>
            <select class={classes!("btn")}>
            {
                MGAlgorithms::iter()
                    .map(|algorithm| {
                        let onclick = algorithm_selector_dispatch.reduce_mut_callback(move |state| {
                            state.selected_mg_algorithm = algorithm;
                        });

                        return html! {
                            <option
                                {onclick}
                                selected={algorithm_selector_state.selected_mg_algorithm == algorithm}
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
                        let onclick = algorithm_selector_dispatch.reduce_mut_callback(move |state| {
                            state.selected_pf_algorithm = algorithm;
                        });

                        return html! {
                            <option
                                {onclick}
                                selected={algorithm_selector_state.selected_pf_algorithm == algorithm}
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
            <button class={classes!("btn", "red")} onclick={on_reset_board_click}>{"Reset Board"}</button>

            if !snackbar_value.msg.is_empty() {
                <NotificationComponent notification={(*snackbar_value).clone()} />
            }
        </div>
    }
}
