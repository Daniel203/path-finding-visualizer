use stylist::{css, yew::styled_component, Style};
use web_sys::MouseEvent;
use yew::{html, Callback, Html, Properties};

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum ButtonColor {
    Gray,
    Green,
    Blue,
    Red,
    Orange,
}

impl Default for ButtonColor {
    fn default() -> Self {
        return Self::Gray;
    }
}

impl ToString for ButtonColor {
    fn to_string(&self) -> String {
        return match self {
            ButtonColor::Orange => "orange",
            ButtonColor::Red => "red",
            ButtonColor::Green => "green",
            ButtonColor::Blue => "blue",
            ButtonColor::Gray => "gray",
        }
        .to_owned();
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub color: Option<ButtonColor>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[styled_component(Button)]
pub fn button(props: &Props) -> Html {
    let stylesheet = get_style();
    let color = props.color.clone().unwrap_or_default();

    let onclick = {
        let props_onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(props_onclick) = props_onclick.clone() {
                props_onclick.emit(event);
            }
        })
    };

    return html! {
        <span class={stylesheet}>
            <button  {onclick} class={color.to_string()}>{&props.label}</button>
        </span>
    };
}

fn get_style() -> Style {
    return Style::new(css!(
        r#"
            button {
                background-color: var(--gray); 
                border: none;
                color: white;
                padding: 13px;
                margin: 4px 4px;
                display: inline-block;
                font-size: 16px;
                font-weight: bold;
                border-radius: 7px;
                transition: background-color 0.2s ease;
                font-family: "Segoe UI", Arial, sans-serif;
            }

            button:hover {
                background-color: var(--dark_gray);
                cursor: pointer;
            }

            button.green {
              background-color: var(--green);
            }

            button.blue {
              background-color: var(--blue);
            }

            button.red {
              background-color: var(--red);
            }

            button.orange {
              background-color: var(--orange);
            }

            button.green:hover {
              background-color: var(--dark_green);
            }

            button.blue:hover {
              background-color: var(--dark_blue);
            }

            button.red:hover {
              background-color: var(--dark_red);
            }

            button.orange:hover {
              background-color: var(--dark_orange);
            }
        "#
    ))
    .unwrap();
}
