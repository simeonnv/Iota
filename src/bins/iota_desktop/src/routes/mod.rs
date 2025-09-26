use dioxus::prelude::*;

mod home;
use home::Home;

mod blog;
use blog::Blog;

mod navbar;
use navbar::Navbar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/blog/:id")]
        Blog { id: i32 },
}
