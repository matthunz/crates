use dioxus::prelude::*;
use dioxus_material::{Chip, Icon, IconKind, Tab, TabRow};

use crate::api::Version;

#[component]
pub fn Krate<'a>(
    cx: Scope<'a>,
    name: &'a str,
    version: &'a str,
    description: &'a str,
    versions: &'a [Version],
    selected: usize,
    onselect: EventHandler<'a, usize>,
) -> Element<'a> {
    render!(
        div { flex: 1, display: "flex", flex_direction: "column", max_width: "800px",
            div { display: "flex", flex_direction: "row", align_items: "center", gap: "10px",
                h2 { name }
                Chip { onclick: |_| {}, version }
            }
            p { description }
            TabRow {
                selected: *selected,
                onselect: |idx| onselect.call(idx),
                tabs: cx
                    .bump()
                    .alloc([
                        render!(TabItem { icon : IconKind::Description, label : "Readme" }),
                        render!(TabItem { icon : IconKind::Label, label : "Versions" }),
                        render!(TabItem { icon : IconKind::DeployedCode, label : "Dependencies" }),
                        render!(TabItem { icon : IconKind::AccountTree, label : "Dependents" }),
                    ])
            }
            match selected {
                0 => versions
                        .first()
                        .and_then(|version| render!(iframe {
                            src: "https://crates.io{version.readme_path}",
                            flex: 1,
                            outline: "none",
                            border: "none"
                        })),
                1 => render!(ul { display: "flex", flex_direction: "column", gap: "10px", list_style: "none", versions.iter().map(|version| render!(li { "{version.num}" })) }),
                _ => todo!()
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

    #[lookbook(default =versions())] versions: lookbook::Json<Vec<Version>>,
) -> Element<'a> {
    render!(
        Krate {
            name: name,
            version: version,
            description: description,
            versions: cx.bump().alloc(versions.0.clone()),
            selected: 0,
            onselect: |_| {}
        }
    )
}

#[cfg(feature = "lookbook")]
fn versions() -> Vec<Version> {
    vec![Version {
        features: std::collections::HashMap::new(),
        num: String::from("v0.1.0"),
        readme_path: String::from(""),
    }]
}
