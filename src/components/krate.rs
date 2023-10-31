use crate::api::{Category, Version};
use dioxus::prelude::*;
use dioxus_material::{use_theme, Button, Chip, Icon, IconKind, Ripple, Tab, TabRow};

#[component]
pub fn Krate<'a>(
    cx: Scope<'a>,
    name: &'a str,
    version: &'a str,
    description: &'a str,
    versions: &'a [Version],
    categories: &'a [Category],
    selected: usize,
    onselect: EventHandler<'a, usize>,
) -> Element<'a> {
    let mut cargo_command_str = format!("cargo add {}", name);
    if *version == &versions[0].num {
        cargo_command_str.push_str(&format!("@{version}"));
    }
    let cargo_toml_str = format!("{} = {}", name, version);

    render!(
        div { flex: 1, display: "flex", flex_direction: "row", max_width: "1000px", gap: "40px",
            div { flex: 1, display: "flex", flex_direction: "column",
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
                    0 => render!(ReadmeTab { versions: *versions }),
                    1 => render!(ul { display: "flex", flex_direction: "column", gap: "10px", list_style: "none", versions.iter().map(|version| render!(li { "{version.num}" })) }),
                    _ => todo!()
                }
            }
            div { flex: 1, display: "flex", flex_direction: "column", gap: "30px", max_width: "200px",
                div { flex: 1, display: "flex", flex_direction: "column", gap: "15px",
                    h4 { margin: 0, padding: 0, "Install" }
                    p { color: "#777", font_size: "14px", margin: 0, padding: 0,
                        "Run the following Cargo command in your project directory:"
                    }
                    InstallButton { label: "{cargo_command_str}", onpress: |_| {} }
                    p { color: "#777", font_size: "14px", margin: 0, padding: 0,
                        "Or add the following line to your Cargo.toml:"
                    }
                    InstallButton { label: "{cargo_toml_str}", onpress: |_| {} }
                }

                div { flex: 1, display: "flex", flex_direction: "column", gap: "15px",
                    h4 { margin: 0, padding: 0, "Categories" }
                    ul { display: "flex", flex_direction: "row", flex_wrap: "wrap", margin: 0, padding: 0, gap: "10px",
                        for category in categories {
                            Chip { onclick: |_| {}, "{category.category}" }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn InstallButton<'a>(cx: Scope<'a>, label: &'a str, onpress: EventHandler<'a>) -> Element<'a> {
    let theme = use_theme(cx);

    render!(
        div {
            display: "flex",
            height: "40px",
            line_height: "40px",
            border_radius: &*theme.border_radius_small,
            border: "2px solid #eee",
            overflow: "hidden",
            Ripple { onclick: |_| onpress.call(()),
                div { width: "100%", padding: "0 20px", label }
            }
        }
    )
}

#[component]
fn ReadmeTab<'a>(cx: Scope<'a>, versions: &'a [Version]) -> Element<'a> {
    let readme_task = use_future(cx, (), |_| {
        let readme_path = versions
            .first()
            .map(|version| version.readme_path.clone())
            .unwrap_or_default();
        async move {
            reqwest::get(&format!("https://crates.io{readme_path}"))
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        }
    });

    if let Some(readme) = readme_task.value() {
        render!(div {
            dangerous_inner_html: "{readme}"
        })
    } else {
        None
    }
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

    #[lookbook(default =categories())] categories: lookbook::Json<Vec<Category>>,
) -> Element<'a> {
    render!(Krate {
        name: name,
        version: version,
        description: description,
        versions: cx.bump().alloc(versions.0.clone()),
        selected: 0,
        categories: cx.bump().alloc(categories.0.clone()),
        onselect: |_| {}
    })
}

#[cfg(feature = "lookbook")]
fn versions() -> Vec<Version> {
    vec![Version {
        features: std::collections::HashMap::new(),
        num: String::from("v0.1.0"),
        readme_path: String::from(""),
    }]
}

#[cfg(feature = "lookbook")]
fn categories() -> Vec<Category> {
    vec![
        Category {
            category: String::from("Asynchronous"),
        },
        Category {
            category: String::from("Network programming"),
        },
    ]
}
