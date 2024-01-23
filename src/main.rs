#![allow(non_snake_case, unused)]

use dioxus::{html::div, prelude::*};
use dioxus_fullstack::prelude::*;
use dse_lib::get_stock;

#[cfg(feature = "ssr")]
fn main() {
    LaunchBuilder::new(app)
        .addr(std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), std::env::var("PORT").unwrap_or("8080".to_string()).parse::<u16>().unwrap()))
        .launch();
}

#[cfg(not(feature = "ssr"))]
fn main() {
    LaunchBuilder::new(app).launch();
}

fn app(cx: Scope) -> Element {
    // Using use_future to create a future that will be executed on the server
    let mut count = use_future(cx, (), |_| async {
        match get_server_data().await {
            Ok(data) => data,
            Err(_) => -1.0,
        }
    });
    let mut result = match count.value() {
        Some(data) => {
            if data == &0.0 {
                "Loading...".to_string()
            }
            else if data == &-1.0{
                "Error".to_string()
            }
             else {
                format!("{} Taka", data)
            }
        }
        None => "Loading...".to_string(),
    };
    

    cx.render(rsx! {
        main{
            class:"min-h-screen bg-black flex flex-col",
            p {
                class: "justify-center content-center text-white",
                "Here are some data:"
                        rsx!{
                            {result}
                        }
            }
            button {
                disabled: count.value() == Some(&0.0),
                class: "text-white border-t-neutral-50 w-5 h-3 p-5 bg-red-600",
                onclick: move |_|{
                    // changing the count will cause the component to re-render
                    // set count to 0
                    count.set(0.0);
                    // restart the future
                    count.restart()
                },
                "Refresh"
            }
        }
    })
}

#[server]
async fn get_server_data() -> Result<f32, ServerFnError> {
    Ok(get_stock("ARAMIT").await?)
}
