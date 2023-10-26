use std::{str::FromStr, fmt};

use dioxus::prelude::*;
use dioxus_material::{use_theme, Icon, IconKind, NavigationRail, NavigationRailItem};
use dioxus_router::prelude::*;
use dioxus_signals::use_signal;

pub mod api;

pub mod components;

mod screens;
use self::screens::{ExploreScreen, KrateScreen, SearchScreen};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KrateTab {
    Readme,
    Versions
}

impl FromStr for KrateTab {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       Ok( match s {
            "readme" => Self::Readme,
            "versions" => Self::Versions,
            _ => todo!()
        })
    }
}

impl fmt::Display for KrateTab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Readme => "readme",
            Self::Versions => "versions"
        })
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Wrap)]
    #[route("/")]
    ExploreScreen,
    #[route("/search/?:query")]
    SearchScreen { query: String },
    #[route("/crate/:name/:tab")]
    KrateScreen { name: String, tab: KrateTab }
}


#[component]
pub fn Wrap(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let theme = use_theme(cx);

    let route: Route = use_route(cx).unwrap();
    let query_ref = use_signal(cx, || {
        if let Route::SearchScreen { query } = &route {
            query.strip_prefix("q=").unwrap_or_default().to_owned()
        } else {
            String::new()
        }
    });

    use_effect(cx, &route, |route| async move {
        if let Route::SearchScreen { query } = route {
            let q = query.strip_prefix("q=").unwrap_or_default().to_owned();

            query_ref.set(q);
        } else {
            query_ref.set(String::new());
        }
    });

    render!(
        div {
            position: "absolute",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            display: "flex",
            flex_direction: "row",
            font_family: "sans-serif",
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
                    onselect: |_| {}
                }
            }
            div { flex: 1, display: "flex", flex_direction: "column",
                form {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    gap: "10px",
                    width: "100%",
                    max_width: "400px",
                    margin: "20px auto",
                    padding: "10px 20px",
                    background: "#eee",
                    border_radius: &*theme.border_radius_small,
                    prevent_default: "onchange",
                    onsubmit: move |_| {
                        navigator
                            .push(Route::SearchScreen {
                                query: format!("q={}", query_ref.read()),
                            });
                    },
                    Icon { kind: dioxus_material::IconKind::Search }
                    input {
                        value: "{query_ref}",
                        placeholder: "Search",
                        flex: 1,
                        margin: 0,
                        padding: 0,
                        font_size: "{theme.label_medium}px",
                        outline: "none",
                        border: "none",
                        background: "none",
                        onchange: move |event: FormEvent| { query_ref.set(event.value.clone()) }
                    }
                }
                div { flex: 1, overflow_y: "auto", Outlet::<Route> {} }
            }
        }
    )
}
