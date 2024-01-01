#![allow(non_snake_case, unused)]

use dioxus::{html::div, prelude::*};
use dioxus_fullstack::prelude::*;
mod get;
use get::get_data;

fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    let mut count = use_server_future(cx, (), |_| async {
        match get_server_data().await {
            Ok(price) => price,
            Err(_) => -1.0,
        }
    })?;
    cx.render(rsx! {
        main{
            class:"min-h-screen bg-black flex flex-col",
            p {
                class: "justify-center content-center text-white",
                "Here are some data:"
                        rsx!{
                            "{count.value():?}"
                        }
            }
            button {
                class: "text-white border-t-neutral-50 w-5 h-3 p-5 bg-red-600",
                onclick: move |_| {
                    // changing the count will cause the component to re-render
                    count.restart()
                },
                "Refresh"
            }
        }
    })
}

#[server]
async fn get_server_data() -> Result<f32, ServerFnError> {
    Ok(get_data().await?)
}
