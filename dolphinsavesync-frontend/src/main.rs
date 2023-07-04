use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

#[derive(PartialEq, Props)]
struct AppProps {
    found_config: bool,
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            "Hello World"
        }
    })
}