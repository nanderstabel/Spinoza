use components::molecules::error_message::ErrorMessage;
use components::organisms::navbar::Navbar;
use router::{switch, Route};
use store::Store;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

mod api;
mod components;
mod pages;
mod router;
mod store;

#[function_component(Spinoza)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<Store>();

    html! {
        <BrowserRouter>
            <Navbar />
            <ErrorMessage />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
