use daisy_rsx::Button;
use dioxus::{desktop::use_window, prelude::*};

const CLOSE_SVG: Asset = asset!("/assets/svg/close.svg");
const FULL_SCREEN_SVG: Asset = asset!("/assets/svg/box.svg");
const MINIMIZE_SVG: Asset = asset!("/assets/svg/minimize.svg");

#[component]
pub fn WindowHandle() -> Element {
    let window = use_window();

    rsx! {
        div {
            class: "flex flex-row w-full h-4 bg-base px-4 my-2",
            div { class: "grow" },
            div {
                class: "flex flex-row gap-2",
                button {
                    class: "btn btn-square",
                    onclick: {
                        let window = window.clone();
                        move |_| window.set_minimized(true)
                    },
                    img {
                        width: 20,
                        src: MINIMIZE_SVG
                    }
                }
                button {
                    class: "btn btn-square",
                    onclick: {
                        let window = window.clone();
                        move |_| window.set_fullscreen(!window.fullscreen().is_some())
                    },
                    img {
                        width: 20,
                        src: FULL_SCREEN_SVG
                    },
                }
                button {
                    class: "btn btn-square",
                    onclick: move |_| window.close(),
                    img {
                        width: 20,
                        src: CLOSE_SVG
                    }
                }

            }
        }
    }
}
