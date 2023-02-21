use crate::components::default_components::button::Button;
use crate::components::default_components::select::Select;
use strum::IntoEnumIterator;
use yew::{function_component, html, use_state, Html};
use yewdux::prelude::use_store;

use crate::components::default_components::button::ButtonColor;
use crate::components::store::algorithm_selector_state::AlgorithmSelectorState;
use crate::models::notification::{Notification, SeverityLevel};
use crate::{
    algorithms::{maze_generation::MGAlgorithms, path_finding::PFAlgorithms},
    components::custom_components::notification_component::NotificationComponent,
};

#[function_component(AlgorithmSelectorComponent)]
pub fn algorithm_selector_component() -> Html {
    let (select_state, select_dispatch) = use_store::<AlgorithmSelectorState>();
    let snackbar_value = use_state(Notification::default);

    let snackbar_value_clone = snackbar_value.clone();
    let on_find_click = select_dispatch.reduce_mut_callback(move |state| {
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
    let on_generate_maze_click = select_dispatch.reduce_mut_callback(move |state| {
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
        select_dispatch.reduce_mut_callback(|state| state.reset_board_clicked = true);

    let on_reset_visited_click =
        select_dispatch.reduce_mut_callback(|state| state.reset_visited_clicked = true);

    let mg_algorithms_options = MGAlgorithms::iter()
        .map(|algorithm| {
            let onclick = select_dispatch.reduce_mut_callback(move |state| {
                state.selected_mg_algorithm = algorithm;
            });

            return html! {
                <option
                    {onclick}
                    selected={select_state.selected_mg_algorithm == algorithm}
                    disabled={algorithm == MGAlgorithms::NotSelected}
                >
                    {algorithm.to_string()}
                </option>
            };
        })
        .collect::<Html>();

    let pf_algorithms_options = PFAlgorithms::iter()
        .map(|algorithm| {
            let onclick = select_dispatch.reduce_mut_callback(move |state| {
                state.selected_pf_algorithm = algorithm;
            });

            return html! {
                <option
                    {onclick}
                    selected={select_state.selected_pf_algorithm == algorithm}
                    disabled={algorithm == PFAlgorithms::NotSelected}
                >
                    {algorithm.to_string()}
                </option>
            };
        })
        .collect::<Html>();

    html! {
        <div>
            <Select options={mg_algorithms_options} />
            <Button label="Generate Maze" color={ButtonColor::Green} onclick={on_generate_maze_click} />
            <span class={"v-separator-m"} />

            <Select options={pf_algorithms_options} />
            <Button label="Find Path" color={ButtonColor::Green} onclick={on_find_click} />

            <span class={"v-separator-m"} />

            <Button label="Reset Visited Cells" color={ButtonColor::Orange} onclick={on_reset_visited_click} />
            <Button label="Reset Board" color={ButtonColor::Red} onclick={on_reset_board_click} />

            if !snackbar_value.msg.is_empty() {
                <NotificationComponent notification={(*snackbar_value).clone()} />
            }
        </div>
    }
}
