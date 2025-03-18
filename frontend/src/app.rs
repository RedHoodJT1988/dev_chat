#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;
use console_error_panic_hook;

use crate::prelude::*;

pub fn App(cx: Scope) -> Element {
    console_error_panic_hook::set_once();
    use_init_atom_root(cx);
    cx.render(rsx! {
        Router {
            Route { to: page::ACCOUNT_REGISTER, page::Register {} },
        }
    })
}