use crate::{
    components::atoms::bb_select::BBSelect,
    store::{self, Store},
};
use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[styled_component(Home)]
pub fn component() -> Html {
    let stylesheet = Style::new(css!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
        "#
    ))
    .unwrap();

    let (store, dispatch) = use_store::<Store>();

    let filter_options = store.sort_options.clone();
    let filter_onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |sort_value| {
            store::select_sort(dispatch.clone(), sort_value);
        })
    };

    let token = store.token.clone();
    html! {
        <section class={stylesheet}>
            if !token.is_empty() {
                <div>
                    <h2>{"Hello"}</h2>
                    <div class="filter">
                        <BBSelect data_test="filter" id="filter" label="Filter Tasks" options={filter_options.clone()} onchange={filter_onchange} />


                    </div>
                </div>
            }
        </section>
    }
}
