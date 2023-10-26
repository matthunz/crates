use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod crate_item;
#[cfg(feature = "lookbook")]
pub use crate_item::CrateItemPreview;
pub use crate_item::{CrateItem, CrateItemProps};

mod krate_screen;
pub use krate_screen::KrateScreen;

mod krates_screen;

pub use krates_screen::KratesScreen;

mod krate;

#[cfg(feature = "lookbook")]
pub use krate::KratePreview;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Wrap)]
    #[route("/")]
    HomeScreen,
    #[route("/crate/:name")]
    KrateScreen { name: String },
}

#[component]
pub fn Wrap(cx: Scope) -> Element {
    render!(
        div { font_family: "sans-serif", Outlet::<Route> {} }
    )
}

#[component]
pub fn HomeScreen(cx: Scope) -> Element {
    render!(KratesScreen {})
}
