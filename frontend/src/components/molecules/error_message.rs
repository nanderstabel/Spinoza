use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;

#[styled_component(ErrorMessage)]
pub fn error_message() -> Html {
    let stylesheet = Style::new(css!(
        r#"
            .error-message {
                text-align: center;
                background-color: red;
                animation: 10s fadeinout 2s linear;
                opacity: 0;
            }

            .hide {
                display: none;
            }

            @keyframes fadeinout {
                from {
                    opacity: 0;
                }

                5% {
                    opacity: 100%;
                }

                95% {
                    opacity: 100%;
                }

                to {
                    opacity: 0;
                }
            }
        "#
    ))
    .unwrap();
    html! {
        <div class={stylesheet}>


        </div>
    }
}
