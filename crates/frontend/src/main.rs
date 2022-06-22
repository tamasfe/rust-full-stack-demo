use api::Api;
use dioxus::prelude::*;

use crate::components::SumResult;

mod api;
pub mod components;

fn main() {
    dioxus::web::launch_cfg(app, |cfg| cfg.rootname("app"));
}

fn app(cx: Scope) -> Element {
    cx.provide_context(Api::new());

    let input_value = use_state(&cx, || String::from("1 + 2 + 3"));

    let numbers = input_value
        .get()
        .split('+')
        .filter_map(|val| val.trim().parse().ok());

    cx.render(rsx! {
        div { "web-scale number addition:" },
        input {
            value: "{input_value}",
            oninput: |event| input_value.set(event.data.value.clone()),
        }
        SumResult {
            numbers: numbers.collect(),
         },
    })
}
