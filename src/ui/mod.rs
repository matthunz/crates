use dioxus::prelude::*;
use dioxus_material::{Icon, IconKind, NavigationRail, NavigationRailItem};
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
    let navigator = use_navigator(cx);

    render!(
        div { display: "flex", flex_direction: "row", font_family: "sans-serif",
            NavigationRail {
                NavigationRailItem {
                    icon: render!(Icon { kind : IconKind::Explore }),
                    label: render!("Explore"),
                    is_selected: true,
                    onselect: |_| {
                        navigator.push(Route::HomeScreen);
                    }
                }
                NavigationRailItem {
                    icon: render!(Icon { kind : IconKind::Person }),
                    label: render!("Sign in"),
                    is_selected: false,
                    onselect: |_| {
                        navigator.push(Route::HomeScreen);
                    }
                }
            }
            Outlet::<Route> {}
        }
    )
}

#[component]
pub fn HomeScreen(cx: Scope) -> Element {
    render!(KratesScreen {})
}
