use gloo_timers::callback::Timeout;
use yew::{classes, function_component, html, use_effect, Html, Properties};

use crate::models::notification::Notification;

#[derive(Properties, PartialEq, Clone)]
pub struct NotificationProps {
    pub notification: Notification,
}

#[function_component(NotificationComponent)]
pub fn notification(props: &NotificationProps) -> Html {
    let notification = props.notification.clone();

    use_effect(move || {
        let snackbar = gloo_utils::document()
            .get_element_by_id("snackbar")
            .unwrap();

        let curr_class_name = snackbar.class_name();

        snackbar.set_class_name(&[&curr_class_name, "show"].join(" "));

        Timeout::new(3000, move || {
            snackbar.set_class_name(&curr_class_name.replace("show", ""));
        })
        .forget();
    });

    html! {
        <div id="snackbar" class={classes!(notification.get_class_name())}>{notification.msg}</div>
    }
}
