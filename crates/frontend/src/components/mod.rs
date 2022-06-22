#![allow(non_snake_case)]
use crate::api::Api;
use dioxus::prelude::*;
use rpc::example::SumRequest;

#[derive(Props, PartialEq)]
pub(crate) struct SumResultProps {
    pub(crate) numbers: Vec<i64>,
}

pub(crate) fn SumResult(cx: Scope<SumResultProps>) -> Element {
    let mut api = cx.consume_context::<Api>().unwrap();

    let sum = use_future(&cx, &cx.props.numbers, |numbers| async move {
        api.example
            .add_numbers(SumRequest { numbers })
            .await
            .ok()
            .map(|res| res.into_inner().sum)
    });

    let v = sum.value().and_then(|v| *v);

    cx.render(rsx! {
        div {
            v.map(|v| rsx! { div { "{v}" } })
                .unwrap_or_else(|| rsx! { "waiting..." })
        }
    })
}
