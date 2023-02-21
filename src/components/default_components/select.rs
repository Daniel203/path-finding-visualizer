use stylist::{css, yew::styled_component, Style};
use yew::{html, virtual_dom::VNode, Html, Properties};

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum SelectColor {
    Gray,
    Green,
    Blue,
    Red,
    Orange,
}

impl Default for SelectColor {
    fn default() -> Self {
        return Self::Gray;
    }
}

impl ToString for SelectColor {
    fn to_string(&self) -> String {
        return match self {
            SelectColor::Orange => "orange",
            SelectColor::Red => "red",
            SelectColor::Green => "green",
            SelectColor::Blue => "blue",
            SelectColor::Gray => "gray",
        }
        .to_owned();
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub options: VNode,
    pub color: Option<SelectColor>,
}

#[styled_component(Select)]
pub fn select(props: &Props) -> Html {
    let stylesheet = get_style();
    let color = props.color.clone().unwrap_or_default();
    let options = props.options.clone();

    return html! {
        <span class={stylesheet}>
            <select class={color.to_string()}>
                {options}
            </select>
        </span>
    };
}

fn get_style() -> Style {
    return Style::new(css!(
        r#"
            select {
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

            select:hover {
                background-color: var(--dark_gray);
                cursor: pointer;
            }

            select.green {
              background-color: var(--green);
            }

            select.blue {
              background-color: var(--blue);
            }

            select.red {
              background-color: var(--red);
            }

            select.orange {
              background-color: var(--orange);
            }

            select.green:hover {
              background-color: var(--dark_green);
            }

            select.blue:hover {
              background-color: var(--dark_blue);
            }

            select.red:hover {
              background-color: var(--dark_red);
            }

            select.orange:hover {
              background-color: var(--dark_orange);
            }
        "#
    ))
    .unwrap();
}
