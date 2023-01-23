use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub is_selected: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub id: String,
    pub label: String,
    pub options: Vec<SelectOption>,
    pub onchange: Callback<String>,
}

#[styled_component(BBSelect)]
pub fn bb_select(props: &Props) -> Html {
    let stylesheet = Style::new(css!(
        r#"
            label {
                font-size: 24px;
            }

            select {
                font-size: 24px;
                width: 100%;
            }
        "#
    ))
    .unwrap();
    html! {
        <div class={stylesheet}>

        </div>
    }
}
