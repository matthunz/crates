use crate::api::{self, Version};
use concoct::{
    hook::{use_effect, use_state},
    view::{OneOf2, OneOf4},
    View, ViewBuilder,
};
use concoct_web::html;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Copy)]
enum Tab {
    Readme,
    Versions,
    Dependencies,
    Dependants,
}

#[derive(Clone)]
pub struct CrateScreen {
    name: String,
}

impl CrateScreen {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl ViewBuilder for CrateScreen {
    fn build(&self) -> impl View {
        let (data, set_data) = use_state(|| None);
        let (tab, set_tab) = use_state(|| Tab::Readme);

        let name = self.name.clone();
        use_effect(&self.name, move || {
            spawn_local(async move {
                let crate_data = api::get_crate(&name).await;
                set_data(Some(Rc::new(crate_data)));
            })
        });

        (
            html::h1(self.name.clone()),
            data.as_ref().map(|data| {
                (
                    html::h2(data.krate.max_stable_version.clone()),
                    html::p(data.krate.description.clone()),
                    html::ul((
                        data.krate
                            .homepage
                            .as_ref()
                            .map(|homepage| link_item(homepage)),
                        data.krate.repository.as_ref().map(|repo| link_item(repo)),
                    )),
                    html::ul((
                        tab_item("Readme", Tab::Readme, set_tab.clone()),
                        tab_item("Versions", Tab::Versions, set_tab.clone()),
                        tab_item("Dependencies", Tab::Dependencies, set_tab.clone()),
                        tab_item("Dependants", Tab::Dependants, set_tab),
                    ))
                    .class("tabs"),
                    match tab {
                        Tab::Readme => OneOf4::A(Readme {
                            crate_name: self.name.clone(),
                            version: data.krate.max_stable_version.clone(),
                        }),
                        Tab::Versions => OneOf4::B(versions(&data.versions)),
                        Tab::Dependencies => OneOf4::C("Dependencies"),
                        Tab::Dependants => OneOf4::D("Dependants"),
                    },
                )
            }),
        )
    }
}

struct Readme {
    crate_name: String,
    version: String,
}

impl ViewBuilder for Readme {
    fn build(&self) -> impl View {
        let (content, set_content) = use_state(|| None);

        use_effect(&self.crate_name, || {
            let name = self.crate_name.clone();
            let version = self.version.clone();
            spawn_local(async move {
                let readme = api::get_readme(&name, &version).await;
                set_content(Some(readme));
            })
        });

        content
            .map(|content| OneOf2::A(content))
            .unwrap_or_else(|| OneOf2::B("Loading..."))
    }
}

fn tab_item(name: &'static str, tab: Tab, set_tab: impl Fn(Tab) + 'static) -> impl View {
    html::li(html::a(name).on_click(move |_| set_tab(tab))).class("tab")
}

fn link_item(url: &str) -> impl View {
    html::li(html::a(url.to_owned()))
}

fn versions(versions: &[Version]) -> impl View {
    versions
        .iter()
        .map(|version| (version.num.clone(), html::li(version.num.clone())))
        .collect::<Vec<_>>()
}
