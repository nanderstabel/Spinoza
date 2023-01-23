use crate::components::atoms::bb_link::{BBLink, LinkType};
use crate::router::Route;
use crate::store::Store;
use std::ops::Deref;
use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::*;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = Style::new(css!(
        r#"
            selection {
                border-bottom: 1px solid antiquewhite;
                padding: 10px 20px;
                display: flex;
                justify-congtent: space-between;
            }

            .nav-right {
                display: flex;
            }

            .nav-right button {
                margin-left: 10px;
            }
        "#
    ))
    .unwrap();

    let (store, dispatch) = use_store::<Store>();
    let token = store.token.clone();

    html! {
        <div class={stylesheet}>
            <section>
                <BBLink text={"Spinoza".to_owned()} data_test={"logo".to_owned()} route={Route::Home} />

                if !is_logged_in(&token) {
                    <div>
                        <BBLink text={"Create Identity".to_owned()} data_test={"create-account".to_owned()} route={Route::CreateAccount} link_type={LinkType::Button} />
                    </div>
                }
            </section>
        </div>
    }
}

fn is_logged_in(token: &str) -> bool {
    !token.deref().is_empty()
}
