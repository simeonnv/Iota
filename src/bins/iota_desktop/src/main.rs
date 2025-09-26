#[cfg(feature = "desktop")]
use dioxus::desktop::{Config, WindowBuilder};

use dioxus::prelude::*;

use components::Hero;

pub mod components;
pub mod routes;
pub mod widgets;
use crate::{routes::Route, widgets::WindowHandle};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_decorations(false)
                    .with_resizable(true),
            ),
        )
        .launch(App);

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "min-h-screen max-h-screen w-full",

            {cfg!(feature = "desktop").then(|| rsx! { WindowHandle {} })}

            Router::<Route> {}
        }
    }
}
