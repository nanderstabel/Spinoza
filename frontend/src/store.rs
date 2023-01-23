use crate::{api::AuthResponse, components::atoms::bb_select::SelectOption};
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct Store {
    pub did: String,
    pub username: String,
    pub token: String,
    pub tasks: Vec<Task>,
    pub filter_options: Vec<SelectOption>,
    pub sort_options: Vec<SelectOption>,
    pub error_message: String,
}

#[derive(PartialEq, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed_at: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
}

pub fn login_reducer(auth_response: AuthResponse, dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(move |store| {
        store.did = auth_response.did;
        // store.username = auth_response.data.username;
        // store.token = auth_response.data.token;
    })
}

pub fn select_sort(dispatch: Dispatch<Store>, sort_value: String) {
    dispatch.reduce_mut(move |store| {
        store.sort_options.iter_mut().for_each(move |sort_option| {
            if sort_option.value == sort_value {
                sort_option.is_selected = true;
            } else {
                sort_option.is_selected = false;
            }
        })
    })
}
