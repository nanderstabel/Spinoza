use crate::{
    api,
    components::molecules::account_form::{AccountForm, Action, User},
    router::Route,
    store::{self, login_reducer, Store},
};
use stylist::{css, yew::styled_component, Style};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[styled_component(CreateAccount)]
pub fn component() -> Html {
    let stylesheet = Style::new(css!(
        r#"
            section {
                display: flex;
                justify-content: center;
            }

            section > div {
                width: 75vw;
            }
        "#
    ))
    .unwrap();

    let history = use_navigator().unwrap();
    let (_store, dispatch) = use_store::<Store>();

    let onsubmit = {
        let store_dispatch = dispatch.clone();
        Callback::from(move |user: User| {
            let history = history.clone();
            let store_dispatch = store_dispatch.clone();

            spawn_local(async move {
                let result = api::create_account(user.stronghold, user.password).await;
                login_reducer(result, store_dispatch);
                history.push(&Route::Home)
            });
        })
    };

    html! {
        <div class={stylesheet}>
            <h1>{"Create Identity"}</h1>
            <section>
                <div>
                    <AccountForm {onsubmit} action={Action::CreateAccount} />
                </div>
            </section>
        </div>
    }
}
