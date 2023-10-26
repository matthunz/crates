use dioxus::prelude::*;
use dioxus_material::{Chip, Icon, IconKind, Tab, TabRow};

#[component]
pub fn Krate<'a>(
    cx: Scope<'a>,
    name: &'a str,
    version: &'a str,
    description: &'a str,
) -> Element<'a> {
    render!(
        div { width: "100%", max_width: "800px", margin: "auto",
            div { display: "flex", flex_direction: "row", align_items: "center", gap: "10px",
                h2 { name }
                Chip { onclick: |_| {}, version }
            }
            p { description }
            TabRow {
                onselect: |_| {},
                tabs: cx
                    .bump()
                    .alloc([
                        render!(TabItem { icon : IconKind::Description, label : "Readme" }),
                        render!(TabItem { icon : IconKind::Label, label : "Versions" }),
                        render!(TabItem { icon : IconKind::DeployedCode, label : "Dependencies" }),
                        render!(TabItem { icon : IconKind::AccountTree, label : "Dependents" }),
                    ])
            }
        }
    )
}

#[component]
fn TabItem<'a>(cx: Scope<'a>, icon: IconKind, label: &'a str) -> Element<'a> {
    render!(
        Tab { 
            div { display: "flex", flex_direction: "row", align_items: "center", gap: "10px",
                Icon { kind: *icon }
                label
            }
        }
    )
}

#[cfg(feature = "lookbook")]
#[lookbook::preview]
pub fn KratePreview<'a>(
    cx: Scope<'a>,

    /// Name of the crate.
    #[lookbook(default = "dioxus")]
    name: &'a str,

    /// Semver version of the crate.
    #[lookbook(default = "v0.5.0")]
    version: &'a str,

    /// Description of the crate.
    #[lookbook(
        default = "Portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust"
    )]
    description: &'a str,
) -> Element<'a> {
    render!( Krate { name: name, version: version, description: description } )
}
