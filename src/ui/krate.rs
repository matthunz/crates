use dioxus::prelude::*;
use dioxus_material::{Icon, IconKind, Tab, TabRow};

#[component]
pub fn Krate<'a>(cx: Scope<'a>, name: &'a str) -> Element<'a> {
    render!(
        div { width: "100%",
            h1 { name }
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
pub fn KratePreview<'a>(cx: Scope<'a>, #[lookbook(default = "tokio")] name: &'a str) -> Element<'a> {
    render!( Krate { name: name } )
}
