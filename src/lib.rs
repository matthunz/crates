use dioxus::prelude::*;
use dioxus_material::{Icon, IconKind, NavigationRail, NavigationRailItem};
use dioxus_router::prelude::*;

pub mod api;

pub mod components;

mod screens;

use screens::{ExploreScreen, KrateScreen};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Wrap)]
    #[route("/")]
    ExploreScreen,
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
                        navigator.push(Route::ExploreScreen);
                    }
                }
                NavigationRailItem {
                    icon: render!(Icon { kind : IconKind::Person }),
                    label: render!("Sign in"),
                    is_selected: false,
                    onselect: |_| {
                        navigator.push(Route::ExploreScreen);
                    }
                }
            }
            Outlet::<Route> {}
        }
    )
}
